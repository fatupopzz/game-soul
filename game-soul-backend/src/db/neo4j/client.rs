// src/db/neo4j/client.rs - Versión optimizada para conectar con Neo4j existente

use anyhow::{Result, Context};
use log::{info, error, debug};
use neo4rs::{Graph, query};
use std::env;
use std::sync::Arc;

// Tipo para el pool de conexiones
pub type DbPool = Arc<Graph>;

// Crear un pool de conexiones a Neo4j con mejor manejo de errores
pub async fn create_connection_pool() -> Result<DbPool> {
    let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
    let username = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
    let password = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".to_string());
    
    info!("Intentando conectar a Neo4j: {}", uri);
    
    // Intentamos conectar
    match Graph::new(uri.clone(), username.clone(), password.clone()).await {
        Ok(graph) => {
            // Verificar la conexión comprobando directamente los juegos
            let test_query = "MATCH (j:Juego) RETURN count(j) as count";
            match graph.execute(query(test_query)).await {
                Ok(mut result) => {
                    if let Ok(Some(row)) = result.next().await {
                        if let Ok(count) = row.get::<i64>("count") {
                            info!("✅ Conexión a Neo4j exitosa. Encontrados {} juegos", count);
                            return Ok(Arc::new(graph));
                        }
                    }
                    error!("No se pudo leer correctamente el conteo de juegos");
                    Err(anyhow::anyhow!("Error al verificar juegos en Neo4j"))
                },
                Err(e) => {
                    error!("Error al ejecutar consulta de prueba: {}", e);
                    Err(anyhow::anyhow!("Error al ejecutar consulta de prueba: {}", e))
                }
            }
        },
        Err(e) => {
            error!("Error al conectar con Neo4j ({}): {}", uri, e);
            Err(anyhow::anyhow!("Error al conectar con Neo4j: {}", e))
        }
    }
}

// Función para verificar la estructura de la base de datos existente
pub async fn verify_database_structure(db: &DbPool) -> Result<()> {
    debug!("Verificando estructura existente en Neo4j");
    
    // Verificar que existan emociones
    let emotions_query = "MATCH (e:Emocion) RETURN count(e) as count";
    let mut result = db.execute(query(emotions_query)).await?;
    if let Ok(Some(row)) = result.next().await {
        if let Ok(count) = row.get::<i64>("count") {
            info!("Encontradas {} emociones en Neo4j", count);
            if count == 0 {
                return Err(anyhow::anyhow!("No se encontraron nodos de emoción en Neo4j"));
            }
        }
    }
    
    // Verificar que existan juegos
    let games_query = "MATCH (j:Juego) RETURN count(j) as count";
    let mut result = db.execute(query(games_query)).await?;
    if let Ok(Some(row)) = result.next().await {
        if let Ok(count) = row.get::<i64>("count") {
            info!("Encontrados {} juegos en Neo4j", count);
            if count == 0 {
                return Err(anyhow::anyhow!("No se encontraron nodos de juego en Neo4j"));
            }
        }
    }
    
    // Verificar que existan características
    let characteristics_query = "MATCH (c:Caracteristica) RETURN count(c) as count";
    let mut result = db.execute(query(characteristics_query)).await?;
    if let Ok(Some(row)) = result.next().await {
        if let Ok(count) = row.get::<i64>("count") {
            info!("Encontradas {} características en Neo4j", count);
        }
    }
    
    // Verificar que las relaciones entre juegos y emociones existan
    let resonance_query = "MATCH (:Juego)-[r:RESUENA_CON]->(:Emocion) RETURN count(r) as count";
    let mut result = db.execute(query(resonance_query)).await?;
    if let Ok(Some(row)) = result.next().await {
        if let Ok(count) = row.get::<i64>("count") {
            info!("Encontradas {} relaciones de resonancia en Neo4j", count);
            if count == 0 {
                return Err(anyhow::anyhow!("No se encontraron relaciones de resonancia en Neo4j"));
            }
        }
    }
    
    info!("✅ Estructura de base de datos verificada correctamente");
    Ok(())
}

// Función para obtener todos los nodos de un tipo específico
pub async fn get_all_nodes_of_type(db: &DbPool, label: &str, property: &str) -> Result<Vec<String>> {
    let query_text = format!("MATCH (n:{}) RETURN n.{} AS value ORDER BY n.{}", label, property, property);
    debug!("Consultando nodos de tipo {}: {}", label, query_text);
    
    let mut result = db.execute(query(&query_text)).await
        .context(format!("Error al consultar nodos de tipo {}", label))?;
    
    let mut values = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        if let Ok(value) = row.get::<String>("value") {
            values.push(value);
        }
    }
    
    info!("Encontrados {} nodos de tipo {}", values.len(), label);
    Ok(values)
}