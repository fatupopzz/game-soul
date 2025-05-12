use anyhow::{Context, Result};
use neo4rs::query;
use log::info;

use crate::db::neo4j::client::DbPool;

/// Obtiene todos los géneros disponibles en la base de datos
pub async fn get_all_genres(db: &DbPool) -> Result<Vec<String>> {
    info!("Consultando todos los géneros en Neo4j");
    
    let query_text = "MATCH (g:Genero) RETURN g.nombre AS nombre ORDER BY nombre";
    
    let mut result = db.execute(query(query_text))
        .await
        .context("Error al ejecutar consulta de géneros")?;
    
    let mut genres = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        if let Ok(nombre) = row.get::<String>("nombre") {
            genres.push(nombre);
        }
    }
    
    info!("Encontrados {} géneros", genres.len());
    Ok(genres)
}

/// Obtiene todas las características disponibles en la base de datos
pub async fn get_all_characteristics(db: &DbPool) -> Result<Vec<String>> {
    info!("Consultando todas las características en Neo4j");
    
    let query_text = "MATCH (c:Caracteristica) RETURN c.nombre AS nombre ORDER BY nombre";
    
    let mut result = db.execute(query(query_text))
        .await
        .context("Error al ejecutar consulta de características")?;
    
    let mut characteristics = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        if let Ok(nombre) = row.get::<String>("nombre") {
            characteristics.push(nombre);
        }
    }
    
    info!("Encontradas {} características", characteristics.len());
    Ok(characteristics)
}

/// Obtiene todas las emociones disponibles en la base de datos
pub async fn get_all_emotions(db: &DbPool) -> Result<Vec<String>> {
    info!("Consultando todas las emociones en Neo4j");
    
    let query_text = "MATCH (e:Emocion) RETURN e.tipo AS tipo ORDER BY tipo";
    
    let mut result = db.execute(query(query_text))
        .await
        .context("Error al ejecutar consulta de emociones")?;
    
    let mut emotions = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        if let Ok(tipo) = row.get::<String>("tipo") {
            emotions.push(tipo);
        }
    }
    
    info!("Encontradas {} emociones", emotions.len());
    Ok(emotions)
}

/// Obtiene todos los rangos de duración disponibles
pub async fn get_all_duration_ranges(db: &DbPool) -> Result<Vec<(String, i32, i32, String)>> {
    info!("Consultando todos los rangos de duración");
    
    let query_text = "MATCH (r:RangoDuracion) RETURN r.nombre AS nombre, r.min AS min, r.max AS max, r.descripcion AS descripcion ORDER BY r.min";
    
    let mut result = db.execute(query(query_text))
        .await
        .context("Error al ejecutar consulta de rangos de duración")?;
    
    let mut ranges = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        let nombre: String = row.get("nombre").unwrap_or_default();
        let min: i32 = row.get("min").unwrap_or_default();
        let max: i32 = row.get("max").unwrap_or_default();
        let descripcion: String = row.get("descripcion").unwrap_or_default();
        
        ranges.push((nombre, min, max, descripcion));
    }
    
    info!("Encontrados {} rangos de duración", ranges.len());
    Ok(ranges)
}