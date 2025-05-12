use anyhow::{Context, Result};
use neo4rs::query;
use log::info;

use crate::db::neo4j::client::DbPool;
use crate::models::recommendation::GameRecommendation;

// Obtener recomendaciones basadas en estado emocional
pub async fn get_emotional_recommendations(
    db: &DbPool,
    user_id: &str,
    emotion_type: &str,
    time_available: i32,
    dealbreakers: Vec<String>,
) -> Result<Vec<GameRecommendation>> {
    info!("Buscando recomendaciones para usuario: {}, emoción: {}, tiempo: {}min", 
          user_id, emotion_type, time_available);
    
    let query_text = r#"
    // Buscar el estado emocional del usuario o usar el proporcionado
    OPTIONAL MATCH (u:Usuario {id: $user_id})-[:ESTADO_EMOCIONAL]->(estado:Emocion)
    WITH COALESCE(estado.tipo, $emotion_type) AS estado_tipo
    
    // Encontrar la emoción correspondiente
    MATCH (estado:Emocion {tipo: estado_tipo})
    
    // Buscar juegos que resuenan con esa emoción
    MATCH (j:Juego)-[r:RESUENA_CON]->(estado)
    
    // Filtrar juegos ya jugados
    WHERE NOT EXISTS {
      MATCH (u:Usuario {id: $user_id})-[:HA_JUGADO]->(j)
    }
    
    // Filtrar por duración usando la relación a RangoDuracion
    WITH u, j, estado, r
    MATCH (j)-[:TIENE_DURACION]->(rd:RangoDuracion)
    WHERE rd.max >= $time_available
    
    // Filtrar por características excluidas ("dealbreakers")
    WITH j, estado, r
    WHERE NOT EXISTS {
        MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
        WHERE c.nombre IN $dealbreakers
    }
    
    // Calcular puntuación básica de resonancia emocional directa
    WITH j, 
         collect(r.intensidad) AS resonancias_directas,
         collect(estado.tipo) AS emociones_coincidentes
    WITH j, 
         reduce(s = 0.0, x IN resonancias_directas | s + x) AS puntuacion_directa,
         emociones_coincidentes
    
    // Agregar puntuación por resonancia indirecta a través de géneros
    OPTIONAL MATCH (j)-[:TIENE_GENERO]->(g:Genero)-[rg:RELACIONADO_CON]->(e:Emocion)
    WHERE e.tipo IN emociones_coincidentes
    WITH j, 
         puntuacion_directa,
         emociones_coincidentes,
         sum(coalesce(rg.intensidad * 0.5, 0)) AS puntuacion_genero
    
    // Agregar puntuación por resonancia indirecta a través de características
    OPTIONAL MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)-[rc:RELACIONADO_CON]->(e:Emocion)
    WHERE e.tipo IN emociones_coincidentes
    WITH j, 
         puntuacion_directa,
         puntuacion_genero,
         emociones_coincidentes,
         sum(coalesce(rc.intensidad * 0.3, 0)) AS puntuacion_caracteristica
    
    // Calcular puntuación total ponderada
    WITH j, 
         puntuacion_directa,
         puntuacion_genero,
         puntuacion_caracteristica,
         emociones_coincidentes,
         (puntuacion_directa * 1.0 + puntuacion_genero * 0.5 + puntuacion_caracteristica * 0.3) AS puntuacion_total
    
    // Ordenar y limitar resultados
    ORDER BY puntuacion_total DESC
    LIMIT 5
    
    // Retornar resultados con detalles
    RETURN j.id AS id,
           j.nombre AS nombre, 
           j.descripcion AS descripcion,
           puntuacion_total AS resonancia,
           puntuacion_directa AS resonancia_directa,
           puntuacion_genero AS resonancia_genero,
           puntuacion_caracteristica AS resonancia_caracteristica,
           [(j)-[:TIENE_GENERO]->(g) | g.nombre] AS generos,
           [(j)-[:TIENE_CARACTERISTICA]->(c) | c.nombre] AS caracteristicas,
           emociones_coincidentes
    "#;
    
    let mut result = db.execute(
        query(query_text)
            .param("user_id", user_id)
            .param("emotion_type", emotion_type)
            .param("time_available", time_available)
            .param("dealbreakers", dealbreakers)
    ).await.context("Error al ejecutar consulta de recomendaciones")?;
    
    let mut recommendations = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        let id: String = row.get("id").unwrap_or_default();
        let nombre: String = row.get("nombre").unwrap_or_default();
        let descripcion: String = row.get("descripcion").unwrap_or_default();
        let resonancia: f64 = row.get("resonancia").unwrap_or(0.0);
        let resonancia_directa: f64 = row.get("resonancia_directa").unwrap_or(0.0);
        let resonancia_genero: f64 = row.get("resonancia_genero").unwrap_or(0.0);
        let resonancia_caracteristica: f64 = row.get("resonancia_caracteristica").unwrap_or(0.0);
        
        // Obtener arrays como Vec<String>
        let generos: Vec<String> = row.get("generos").unwrap_or_default();
        let caracteristicas: Vec<String> = row.get("caracteristicas").unwrap_or_default();
        let emociones_coincidentes: Vec<String> = row.get("emociones_coincidentes").unwrap_or_default();
        
        recommendations.push(GameRecommendation {
            id,
            nombre,
            descripcion,
            resonancia,
            resonancia_desglosada: Some(crate::models::recommendation::ResonanciaDesglosada {
                directa: resonancia_directa,
                por_genero: resonancia_genero,
                por_caracteristica: resonancia_caracteristica,
            }),
            generos,
            caracteristicas,
            emociones_coincidentes,
        });
    }
    
    info!("Encontradas {} recomendaciones", recommendations.len());
    Ok(recommendations)
}

// Obtener recomendaciones de exploración para evitar fatiga
pub async fn get_exploration_recommendations(
    db: &DbPool,
    user_id: &str,
    time_available: i32,
) -> Result<Vec<GameRecommendation>> {
    let query_text = r#"
    // Encontrar géneros que el usuario ha jugado recientemente
    MATCH (u:Usuario {id: $user_id})-[h:HA_JUGADO]->(j:Juego)-[:TIENE_GENERO]->(g:Genero)
    WHERE h.fecha >= datetime() - duration('P30D')
    WITH u, collect(g.nombre) AS generos_recientes
    
    // Encontrar juegos de géneros diferentes
    MATCH (nuevo:Juego)-[:TIENE_GENERO]->(g:Genero)
    WHERE NOT g.nombre IN generos_recientes
    
    // Filtrar juegos ya jugados
    AND NOT EXISTS {
      MATCH (u)-[:HA_JUGADO]->(nuevo)
    }
    
    // Filtrar por duración usando la relación a RangoDuracion
    WITH nuevo, u
    MATCH (nuevo)-[:TIENE_DURACION]->(rd:RangoDuracion)
    WHERE rd.max >= $time_available
    
    // Encontrar juegos con características interesantes 
    WITH nuevo, u
    MATCH (nuevo)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
    WHERE NOT EXISTS {
        MATCH (u)-[h:HA_JUGADO]->(j)-[:TIENE_CARACTERISTICA]->(c)
        WHERE h.fecha >= datetime() - duration('P30D')
    }
    
    // Dar prioridad a juegos con emociones similares a las que al usuario le gustan
    WITH nuevo, rand() + 0.1 AS puntuacion_exploracion
    
    // Ordenar aleatoriamente pero con cierta ponderación
    ORDER BY puntuacion_exploracion DESC
    LIMIT 3
    
    // Retornar recomendaciones exploratorias
    RETURN nuevo.id AS id,
           nuevo.nombre AS nombre,
           nuevo.descripcion AS descripcion,
           puntuacion_exploracion AS resonancia,
           0.0 AS resonancia_directa,
           0.0 AS resonancia_genero,
           0.0 AS resonancia_caracteristica,
           [(nuevo)-[:TIENE_GENERO]->(g) | g.nombre] AS generos,
           [(nuevo)-[:TIENE_CARACTERISTICA]->(c) | c.nombre] AS caracteristicas,
           [] AS emociones_coincidentes
    "#;
    
    let mut result = db.execute(
        query(query_text)
            .param("user_id", user_id)
            .param("time_available", time_available)
    ).await.context("Error al ejecutar consulta de recomendaciones de exploración")?;
    
    let mut recommendations = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        let id: String = row.get("id").unwrap_or_default();
        let nombre: String = row.get("nombre").unwrap_or_default();
        let descripcion: String = row.get("descripcion").unwrap_or_default();
        let resonancia: f64 = row.get("resonancia").unwrap_or(0.0);
        
        // Obtener arrays como Vec<String>
        let generos: Vec<String> = row.get("generos").unwrap_or_default();
        let caracteristicas: Vec<String> = row.get("caracteristicas").unwrap_or_default();
        
        recommendations.push(GameRecommendation {
            id,
            nombre,
            descripcion,
            resonancia,
            resonancia_desglosada: None,
            generos,
            caracteristicas,
            emociones_coincidentes: vec!["exploración".to_string()],
        });
    }
    
    info!("Encontradas {} recomendaciones de exploración", recommendations.len());
    Ok(recommendations)
}

// Procesar feedback del usuario sobre una recomendación
pub async fn save_recommendation_feedback(
    db: &DbPool,
    user_id: &str,
    game_id: &str,
    satisfaction: i32,
    emotions_experienced: Option<Vec<String>>,
) -> Result<()> {
    info!("Guardando feedback para usuario: {}, juego: {}, satisfacción: {}", 
          user_id, game_id, satisfaction);
    
    // Normalizar satisfacción a un delta de resonancia (-0.2 a +0.2)
    let delta_satisfaction = (satisfaction as f64 - 3.0) / 10.0;
    
    // Consulta para actualizar la relación de resonancia
    let query_text = r#"
    // Actualizar o crear relación HA_JUGADO
    MERGE (u:Usuario {id: $user_id})
    MERGE (j:Juego {id: $game_id})
    MERGE (u)-[h:HA_JUGADO]->(j)
    ON CREATE SET h.fecha = datetime(), h.satisfaccion = $satisfaction
    ON MATCH SET h.fecha = datetime(), h.satisfaccion = $satisfaction
    
    // Actualizar relación de resonancia con cada emoción experimentada
    WITH u, j
    UNWIND $emotions_experienced AS emocion_tipo
    MATCH (e:Emocion {tipo: emocion_tipo})
    MERGE (j)-[r:RESUENA_CON]->(e)
    ON CREATE SET r.intensidad = 0.5 + $delta_satisfaction
    ON MATCH SET r.intensidad = r.intensidad + $delta_satisfaction,
                 r.intensidad = CASE 
                                  WHEN r.intensidad > 1.0 THEN 1.0
                                  WHEN r.intensidad < 0.0 THEN 0.0
                                  ELSE r.intensidad
                                END,
                 r.ultima_actualizacion = datetime()
    
    RETURN count(*) as processed
    "#;
    
    // Si no se proporcionaron emociones, usar un vector con la emoción principal del juego
    let emotions = emotions_experienced.unwrap_or_else(|| vec!["neutral".to_string()]);
    
    db.execute(
        query(query_text)
            .param("user_id", user_id)
            .param("game_id", game_id)
            .param("satisfaction", satisfaction)
            .param("delta_satisfaction", delta_satisfaction)
            .param("emotions_experienced", emotions)
        ).await
        .context("Error al guardar feedback de recomendación")?;
        
        info!("Feedback procesado correctamente");
        Ok(())
    }
    
    // Guardar el estado emocional actual del usuario
    pub async fn save_user_emotional_state(
        db: &DbPool,
        user_id: &str,
        emotion_type: &str,
    ) -> Result<()> {
        info!("Guardando estado emocional para usuario: {}, emoción: {}", user_id, emotion_type);
        
        let query_text = r#"
        // Crear o actualizar el usuario
        MERGE (u:Usuario {id: $user_id})
        
        // Eliminar relaciones de estado emocional anteriores
        OPTIONAL MATCH (u)-[old:ESTADO_EMOCIONAL]->()
        DELETE old
        
        // Crear nueva relación de estado emocional
        WITH u
        MATCH (e:Emocion {tipo: $emotion_type})
        CREATE (u)-[:ESTADO_EMOCIONAL {fecha: datetime()}]->(e)
        
        RETURN u.id
        "#;
        
        db.execute(
            query(query_text)
                .param("user_id", user_id)
                .param("emotion_type", emotion_type)
        ).await
        .context("Error al guardar estado emocional del usuario")?;
        
        info!("Estado emocional guardado correctamente");
        Ok(())
    }