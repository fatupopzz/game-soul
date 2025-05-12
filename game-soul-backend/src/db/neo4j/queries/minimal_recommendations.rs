use anyhow::{Context, Result};
use neo4rs::query;
use log::{info, debug, error};

use crate::db::neo4j::client::DbPool;
use crate::models::recommendation::GameRecommendation;

/// Versión simplificada de obtener recomendaciones - adaptada a la estructura existente
pub async fn get_minimal_recommendations(
    db: &DbPool,
    emotion_type: &str,
    dealbreakers: &[String],
) -> Result<Vec<GameRecommendation>> {
    info!("Buscando recomendaciones mínimas para emoción: {}", emotion_type);
    debug!("Dealbreakers: {:?}", dealbreakers);
    
    // Consulta absolutamente mínima
    let query_text = r#"
    // Buscar la emoción especificada
    MATCH (e:Emocion {tipo: $emotion_type})
    
    // Encontrar juegos que resuenan con esa emoción
    MATCH (j:Juego)-[r:RESUENA_CON]->(e)
    
    // Recopilar características para filtrado
    OPTIONAL MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
    WITH j, r, collect(DISTINCT c.nombre) as caracteristicas
    
    // Filtrar por dealbreakers
    WHERE NONE(d IN $dealbreakers WHERE d IN caracteristicas)
    
    // Ordenar por intensidad
    ORDER BY r.intensidad DESC
    LIMIT 5
    
    // Devolver datos básicos
    RETURN 
        j.id as id,
        j.nombre as nombre,
        COALESCE(j.descripcion, 'Sin descripción disponible') as descripcion,
        r.intensidad as resonancia,
        caracteristicas,
        [$emotion_type] as emociones_coincidentes
    "#;
    
    debug!("Ejecutando consulta mínima: {}", query_text);
    
    // Ejecutar la consulta con manejo de errores detallado
    let mut result = db.execute(
        query(query_text)
            .param("emotion_type", emotion_type)
            .param("dealbreakers", dealbreakers)
    ).await.context("Error al ejecutar consulta de recomendaciones mínimas")?;
    
    let mut recommendations = Vec::new();
    
    // Procesar resultados con errores exhaustivos
    while let Ok(Some(row)) = result.next().await {
        // Log detallado de cada columna para diagnóstico
        debug!("Procesando fila de resultados...");
        
        // Extraer id con manejo de errores
        let id = match row.get::<String>("id") {
            Ok(val) => {
                debug!("ID extraído: {}", val);
                val
            },
            Err(e) => {
                error!("Error al extraer ID: {}", e);
                continue;
            }
        };
        
        // Extraer nombre con manejo de errores
        let nombre = match row.get::<String>("nombre") {
            Ok(val) => {
                debug!("Nombre extraído: {}", val);
                val
            },
            Err(e) => {
                error!("Error al extraer nombre para juego {}: {}", id, e);
                "Nombre desconocido".to_string()
            }
        };
        
        // Extraer descripción con manejo de errores
        let descripcion = match row.get::<String>("descripcion") {
            Ok(val) => {
                debug!("Descripción extraída de longitud: {}", val.len());
                val
            },
            Err(e) => {
                error!("Error al extraer descripción para juego {}: {}", id, e);
                "Sin descripción disponible".to_string()
            }
        };
        
        // Extraer resonancia con manejo de errores
        let resonancia = match row.get::<f64>("resonancia") {
            Ok(val) => {
                debug!("Resonancia extraída: {}", val);
                val
            },
            Err(e) => {
                error!("Error al extraer resonancia para juego {}: {}", id, e);
                0.5 // Valor por defecto
            }
        };
        
        // Extraer características con manejo de errores
        let caracteristicas = match row.get::<Vec<String>>("caracteristicas") {
            Ok(val) => {
                debug!("Características extraídas: {:?}", val);
                val
            },
            Err(e) => {
                error!("Error al extraer características para juego {}: {}", id, e);
                Vec::new()
            }
        };
        
        // Extraer emociones coincidentes con manejo de errores
        let emociones_coincidentes = match row.get::<Vec<String>>("emociones_coincidentes") {
            Ok(val) => {
                debug!("Emociones coincidentes extraídas: {:?}", val);
                val
            },
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
            generos: Vec::new(),
            caracteristicas,
            emociones_coincidentes,
        });
    }
    
    info!("Encontradas {} recomendaciones mínimas", recommendations.len());
    
    // Si no hay recomendaciones, proporcionar un mensaje claro
    if recommendations.is_empty() {
        error!("No se encontraron recomendaciones. Verifica que existan juegos con emoción {}", emotion_type);
    }
    
    Ok(recommendations)
}
