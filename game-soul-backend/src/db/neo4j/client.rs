use anyhow::{Context, Result};
use log::info;
use neo4rs::{Graph, query};
use std::env;
use std::sync::Arc;

// Tipo para el pool de conexiones
pub type DbPool = Arc<Graph>;

// Crear un pool de conexiones a Neo4j
pub async fn create_connection_pool() -> Result<DbPool> {
    let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
    let username = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
    let password = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".to_string());
    
    info!("Conectando a Neo4j: {}", uri);
    
    let graph = Graph::new(uri, username, password)
        .await
        .context("Error al conectar con Neo4j")?;
    
    // Verificar la conexión con una consulta simple
    let mut result = graph.execute(query("RETURN 1 AS n")).await
        .context("Error al verificar la conexión a Neo4j")?;
    
    if let Ok(Some(_)) = result.next().await {
        info!("Conexión a Neo4j verificada correctamente");
    } else {
        return Err(anyhow::anyhow!("No se pudo verificar la conexión a Neo4j"));
    }
    
    Ok(Arc::new(graph))
}

// Función para verificar si existen los nodos de emociones y crearlos si no
pub async fn ensure_emotion_nodes(db: &DbPool) -> Result<()> {
    let emotion_types = vec![
        "alegre", "relajante", "melancólico", "exploración", 
        "desafiante", "contemplativo", "social", "competitivo", "creativo",
    ];
    
    for emotion_type in emotion_types {
        let query_text = "MERGE (:Emocion {tipo: $tipo})";
        db.execute(query(query_text).param("tipo", emotion_type))
            .await
            .context(format!("Error al crear el nodo de emoción: {}", emotion_type))?;
    }
    
    info!("✅ Nodos de emociones verificados/creados correctamente");
    Ok(())
}

// Función para verificar si existen los nodos de rangos de duración y crearlos si no
pub async fn ensure_duration_range_nodes(db: &DbPool) -> Result<()> {
    let query_text = r#"
    MERGE (:RangoDuracion {nombre: "muy_corto", min: 0, max: 30, descripcion: "Menos de 30 minutos"})
    MERGE (:RangoDuracion {nombre: "corto", min: 30, max: 60, descripcion: "Entre 30 minutos y 1 hora"})
    MERGE (:RangoDuracion {nombre: "medio", min: 60, max: 180, descripcion: "Entre 1 y 3 horas"})
    MERGE (:RangoDuracion {nombre: "largo", min: 180, max: 480, descripcion: "Entre 3 y 8 horas"})
    MERGE (:RangoDuracion {nombre: "muy_largo", min: 480, max: 9999, descripcion: "Más de 8 horas"})
    "#;
    
    db.execute(query(query_text))
        .await
        .context("Error al crear nodos de rangos de duración")?;
    
    info!("✅ Nodos de rangos de duración verificados/creados correctamente");
    Ok(())
}

// Función para verificar si existen las características y crearlas si no
pub async fn ensure_characteristic_nodes(db: &DbPool) -> Result<()> {
    let characteristics = vec![
        "social", "exploración", "desafiante", "historia", "puzzles",
        "coleccionable", "difícil", "combate", "atmósfera", "inmersivo",
        "decisiones", "artístico", "trabajo en equipo", "habilidades",
        "estrategia", "rápido", "personajes", "estilizado"
    ];
    
    for characteristic in characteristics {
        let query_text = "MERGE (:Caracteristica {nombre: $nombre})";
        db.execute(query(query_text).param("nombre", characteristic))
            .await
            .context(format!("Error al crear el nodo de característica: {}", characteristic))?;
    }
    
    info!("✅ Nodos de características verificados/creados correctamente");
    Ok(())
}

// Función de inicialización que verifica toda la estructura básica
pub async fn initialize_database(db: &DbPool) -> Result<()> {
    // Verificar/crear nodos de emociones
    ensure_emotion_nodes(db).await?;
    
    // Verificar/crear nodos de rangos de duración
    ensure_duration_range_nodes(db).await?;
    
    // Verificar/crear nodos de características
    ensure_characteristic_nodes(db).await?;
    
    info!("✅ Base de datos inicializada correctamente");
    Ok(())
}