//---------------------------------------
//Este archivo contiene las consultas para obtener recomendaciones de juegos
// basadas en el estado emocional del usuario y sus preferencias.
// También incluye una consulta de diagnóstico para verificar la conexión
// y la integridad de los datos en la base de datos Neo4j.
// Además, se proporciona una consulta alternativa más simple
// para depurar problemas de conexión o datos.
//-----------------------------------------

use anyhow::{Context, Result};
use neo4rs::query;
use log::{info, debug, error};

use crate::db::neo4j::client::DbPool;
use crate::models::recommendation::GameRecommendation;

/// Obtener recomendaciones basadas en estado emocional esto si funciona hehe 
pub async fn get_recommendations(
    db: &DbPool,
    emotion_type: &str,
    dealbreakers: &[String],
) -> Result<Vec<GameRecommendation>> {
    info!("Buscando recomendaciones para emoción: {}", emotion_type);
    debug!("Dealbreakers: {:?}", dealbreakers);
    
    // Consulta adaptada para trabajar con dealbreakers y caracteristicas
    let query_text = r#"
    // Buscar juegos que resuenan con la emoción proporcionada
    MATCH (j:Juego)-[r:RESUENA_CON]->(e:Emocion {tipo: $emotion_type})
    
    // Recopilar características para filtrado
    OPTIONAL MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
    WITH j, r, e, collect(distinct c.nombre) as caracteristicas
    
    // Filtrar por dealbreakers si hay alguno
    WHERE size($dealbreakers) = 0 OR NONE(dealbreaker IN $dealbreakers WHERE dealbreaker IN caracteristicas)
    
    // Recopilar géneros si existen
    OPTIONAL MATCH (j)-[:TIENE_GENERO]->(g:Genero)
    WITH j, r, e, caracteristicas, collect(distinct g.nombre) as generos
    
    // Ordenar por intensidad de resonancia
    ORDER BY r.intensidad DESC
    LIMIT 5
    
    // Devolver datos para recomendación
    RETURN 
        j.id as id,
        j.nombre as nombre,
        COALESCE(j.descripcion, 'Sin descripción disponible') as descripcion,
        r.intensidad as resonancia,
        caracteristicas,
        generos,
        collect(e.tipo) as emociones_coincidentes
    "#;
    
    debug!("Ejecutando consulta de recomendaciones: {}", query_text);
    
    // Ejecutar la consulta con parámetros
    let mut result = db.execute(
        query(query_text)
            .param("emotion_type", emotion_type)
            .param("dealbreakers", dealbreakers)
    ).await.context("Error al ejecutar consulta de recomendaciones")?;
    
    let mut recommendations = Vec::new();
    
    // Procesar resultados de la consulta, tipo los ids de los juegos
    while let Ok(Some(row)) = result.next().await {
        debug!("Procesando fila de resultados");
        
        // Extraer datos con manejo de errores
        let id = match row.get::<String>("id") {
            Ok(val) => val,
            Err(e) => {
                error!("Error al extraer ID: {}", e);
                continue;
            }
        };
        
        let nombre = match row.get::<String>("nombre") {
            Ok(val) => val,
            Err(e) => {
                error!("Error al extraer nombre para juego {}: {}", id, e);
                "Juego sin nombre".to_string()
            }
        };
        
        let descripcion = match row.get::<String>("descripcion") {
            Ok(val) => val,
            Err(e) => {
                error!("Error al extraer descripción para juego {}: {}", id, e);
                "Sin descripción disponible".to_string()
            }
        };
        
        let resonancia = match row.get::<f64>("resonancia") {
            Ok(val) => val,
            Err(e) => {
                error!("Error al extraer resonancia para juego {}: {}", id, e);
                0.5 // valor por defecto
            }
        };
        
        // Extraer arrays con manejo de errores por si acaso no funciona neo4j
        let caracteristicas = match row.get::<Vec<String>>("caracteristicas") {
            Ok(val) => val,
            Err(e) => {
                error!("Error al extraer características para juego {}: {}", id, e);
                Vec::new()
            }
        };
        
        let generos = match row.get::<Vec<String>>("generos") {
            Ok(val) => val,
            Err(e) => {
                error!("Error al extraer géneros para juego {}: {}", id, e);
                Vec::new()
            }
        };
        
        let emociones_coincidentes = match row.get::<Vec<String>>("emociones_coincidentes") {
            Ok(val) => val,
            Err(e) => {
                error!("Error al extraer emociones coincidentes para juego {}: {}", id, e);
                vec![emotion_type.to_string()]
            }
        };
        
        // Añadir a las recomendaciones
        recommendations.push(GameRecommendation {
            id,
            nombre,
            descripcion,
            resonancia,
            resonancia_desglosada: None,
            generos,
            caracteristicas,
            emociones_coincidentes,
        });
    }
    
    info!("Encontradas {} recomendaciones para {}", recommendations.len(), emotion_type);
    
    // Si no encontramos recomendaciones, intenta una consulta más simple que son boiler
    if recommendations.is_empty() {
        info!("No se encontraron recomendaciones, intentando consulta alternativa");
        recommendations = get_recommendations_alternative(db, emotion_type).await?;
    }
    
    Ok(recommendations)
}

/// Consulta alternativa más simple (para depuración) por si no funciona la anterior
pub async fn get_recommendations_alternative(
    db: &DbPool,
    emotion_type: &str,
) -> Result<Vec<GameRecommendation>> {
    let simple_query = r#"
    MATCH (j:Juego)-[r:RESUENA_CON]->(e:Emocion {tipo: $emotion_type})
    RETURN 
        j.id as id,
        j.nombre as nombre,
        j.descripcion as descripcion,
        r.intensidad as resonancia
    LIMIT 10
    "#;
    
    debug!("Ejecutando consulta alternativa: {}", simple_query);
    
    let mut result = db.execute(
        query(simple_query)
            .param("emotion_type", emotion_type)
    ).await.context("Error al ejecutar consulta alternativa")?;
    
    let mut recommendations = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        let id = row.get::<String>("id").unwrap_or_else(|_| "unknown".to_string());
        let nombre = row.get::<String>("nombre").unwrap_or_else(|_| "Juego sin nombre".to_string());
        let descripcion = row.get::<String>("descripcion").unwrap_or_else(|_| "Sin descripción".to_string());
        let resonancia = row.get::<f64>("resonancia").unwrap_or(0.5);
        
        recommendations.push(GameRecommendation {
            id,
            nombre,
            descripcion,
            resonancia,
            resonancia_desglosada: None,
            generos: Vec::new(),
            caracteristicas: Vec::new(),
            emociones_coincidentes: vec![emotion_type.to_string()],
        });
    }
    
    info!("Consulta alternativa encontró {} recomendaciones", recommendations.len());
    Ok(recommendations)
}

/// Consulta de diagnóstico para verificar problemas de conexión
/// se verifica usando curl localhost:3001/diagnose o algo asi 
pub async fn diagnose_database(db: &DbPool) -> Result<String> {
    let diagnose_query = r#"
    // Contar juegos
    MATCH (j:Juego) 
    WITH count(j) as juegos
    
    // Contar emociones
    MATCH (e:Emocion) 
    WITH juegos, count(e) as emociones
    
    // Contar características
    MATCH (c:Caracteristica) 
    WITH juegos, emociones, count(c) as caracteristicas
    
    // Contar relaciones de resonancia
    MATCH (:Juego)-[r:RESUENA_CON]->(:Emocion) 
    WITH juegos, emociones, caracteristicas, count(r) as resonancias
    
    RETURN juegos, emociones, caracteristicas, resonancias
    "#;
    
    let mut result = db.execute(query(diagnose_query)).await?;
    
    if let Ok(Some(row)) = result.next().await {
        let juegos: i64 = row.get("juegos").unwrap_or(0);
        let emociones: i64 = row.get("emociones").unwrap_or(0);
        let caracteristicas: i64 = row.get("caracteristicas").unwrap_or(0);
        let resonancias: i64 = row.get("resonancias").unwrap_or(0);
        
        let report = format!(
            "Diagnóstico Neo4j: {} juegos, {} emociones, {} características, {} relaciones de resonancia",
            juegos, emociones, caracteristicas, resonancias
        );
        
        info!("{}", report);
        return Ok(report);
    }
    
    Err(anyhow::anyhow!("No se pudieron obtener métricas de diagnóstico"))
}