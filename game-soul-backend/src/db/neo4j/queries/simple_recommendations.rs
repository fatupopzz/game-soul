use anyhow::{Context, Result};
use neo4rs::query;
use log::info;

use crate::db::neo4j::client::DbPool;
use crate::models::recommendation::GameRecommendation;

/// Obtener recomendaciones simplificadas
pub async fn get_recommendations(
    db: &DbPool,
    emotion_type: &str,
    time_available: i32,
    dealbreakers: &[String],
) -> Result<Vec<GameRecommendation>> {
    info!("Buscando recomendaciones simplificadas para emoción: {}, tiempo: {}min", 
          emotion_type, time_available);
    
    // Consulta simplificada que será más fácil de depurar
    let query_text = r#"
    // Buscar juegos que resuenan con la emoción proporcionada
    MATCH (e:Emocion {tipo: $emotion_type})
    MATCH (j:Juego)-[r:RESUENA_CON]->(e)
    
    // Incluir características para filtrado
    OPTIONAL MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
    
    // Agrupar las características por juego
    WITH j, r, collect(DISTINCT c.nombre) as caracteristicas
    
    // Filtrar por dealbreakers (características a evitar)
    WHERE NONE(dealbreaker IN $dealbreakers WHERE dealbreaker IN caracteristicas)
    
    // Ordenar por intensidad de resonancia
    ORDER BY r.intensidad DESC
    LIMIT 5
    
    // Retornar resultados
    RETURN j.id AS id,
           j.nombre AS nombre, 
           COALESCE(j.descripcion, 'Sin descripción disponible') AS descripcion,
           r.intensidad AS resonancia,
           caracteristicas,
           [$emotion_type] AS emociones_coincidentes
    "#;
    
    let mut result = db.execute(
        query(query_text)
            .param("emotion_type", emotion_type)
            .param("dealbreakers", dealbreakers)
    ).await.context("Error al ejecutar consulta de recomendaciones simplificada")?;
    
    let mut recommendations = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        // Extraer valores con manejo de errores
        let id = match row.get::<String>("id") {
            Ok(value) => value,
            Err(_) => continue
        };
        
        let nombre = match row.get::<String>("nombre") {
            Ok(value) => value,
            Err(_) => "Juego sin nombre".to_string()
        };
        
        let descripcion = match row.get::<String>("descripcion") {
            Ok(value) => value,
            Err(_) => "Sin descripción disponible".to_string()
        };
        
        let resonancia = match row.get::<f64>("resonancia") {
            Ok(value) => value,
            Err(_) => 0.5
        };
        
        let caracteristicas = match row.get::<Vec<String>>("caracteristicas") {
            Ok(value) => value,
            Err(_) => Vec::new()
        };
        
        let emociones_coincidentes = match row.get::<Vec<String>>("emociones_coincidentes") {
            Ok(value) => value,
            Err(_) => vec![emotion_type.to_string()]
        };
        
        recommendations.push(GameRecommendation {
            id,
            nombre,
            descripcion,
            resonancia,
            resonancia_desglosada: None,
            generos: Vec::new(),
            caracteristicas,
            emociones_coincidentes,
        });
    }
    
    info!("Encontradas {} recomendaciones simplificadas", recommendations.len());
    Ok(recommendations)
}