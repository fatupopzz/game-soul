//--------------------------------------------------------------------------
// GAME SOUL - CONSULTAS DE USUARIO (Actualizado)
//
// Este archivo contiene las consultas relacionadas con usuarios y perfiles
// emocionales en la base de datos Neo4j. Incluye funciones para crear,
// verificar y gestionar usuarios automáticamente.
//--------------------------------------------------------------------------

use anyhow::{Context, Result};
use neo4rs::query;
use log::{info, debug};
use std::collections::HashMap;
use chrono::Utc;

use crate::db::neo4j::client::DbPool;
use crate::models::questionnaire::EmotionalProfile;

/// Crear o verificar que existe un usuario en Neo4j
pub async fn ensure_user_exists(
    db: &DbPool,
    user_id: &str,
) -> Result<()> {
    info!("Verificando/creando usuario: {}", user_id);
    
    let query_text = r#"
    MERGE (u:Usuario {id: $user_id})
    ON CREATE SET 
        u.fecha_creacion = datetime(),
        u.nombre = $user_id,
        u.estado = "activo"
    ON MATCH SET 
        u.ultima_actividad = datetime()
    RETURN u.id, u.fecha_creacion
    "#;
    
    let mut result = db.execute(
        query(query_text)
            .param("user_id", user_id)
    ).await
    .context("Error al crear/verificar usuario")?;
    
    if let Ok(Some(row)) = result.next().await {
        let id: String = row.get("u.id").unwrap_or_default();
        debug!("Usuario procesado correctamente: {}", id);
    }
    
    info!("Usuario {} verificado/creado exitosamente", user_id);
    Ok(())
}

/// Guarda el perfil emocional del usuario (versión mejorada)
/// Guarda el perfil emocional del usuario (versión corregida)
pub async fn save_emotional_profile(
    db: &DbPool,
    profile: &EmotionalProfile,
) -> Result<()> {
    info!("Guardando perfil emocional para usuario: {}", profile.user_id);
    
    // Primero asegurar que el usuario existe
    ensure_user_exists(db, &profile.user_id).await?;
    
    // Eliminar estado emocional anterior
    let clear_previous_query = r#"
    MATCH (u:Usuario {id: $user_id})-[r:ESTADO_EMOCIONAL]->()
    DELETE r
    "#;
    
    let mut _result = db.execute(
        query(clear_previous_query)
            .param("user_id", profile.user_id.as_str())
    ).await
    .context("Error al limpiar estado emocional anterior")?;
    
    // Procesar el resultado (necesario para que se ejecute la consulta)
    while let Ok(Some(_)) = _result.next().await {
        // Procesar filas si las hay
    }
    
    // Guardar el estado emocional dominante
    let emotion_query = r#"
    MATCH (u:Usuario {id: $user_id})
    MATCH (e:Emocion {tipo: $emotion_type})
    CREATE (u)-[r:ESTADO_EMOCIONAL]->(e)
    SET r.fecha = datetime(), r.intensidad = $intensidad
    RETURN e.tipo
    "#;
    
    // Obtener la intensidad de la emoción dominante
    let dominant_intensity = profile.emotions.get(&profile.dominant_emotion).unwrap_or(&1.0);
    
    let mut _result = db.execute(
        query(emotion_query)
            .param("user_id", profile.user_id.as_str())
            .param("emotion_type", profile.dominant_emotion.as_str())
            .param("intensidad", *dominant_intensity)
    ).await
    .context("Error al guardar estado emocional dominante")?;
    
    // Procesar el resultado
    while let Ok(Some(_)) = _result.next().await {
        // Procesar filas si las hay
    }
    
    // Guardar preferencia de tiempo
    let time_range = profile.time_available.get_db_name();
    let time_query = r#"
    MATCH (u:Usuario {id: $user_id})
    MATCH (r:RangoDuracion {nombre: $range_name})
    MERGE (u)-[rel:PREFIERE_DURACION]->(r)
    ON CREATE SET rel.fecha = datetime()
    ON MATCH SET rel.fecha = datetime()
    RETURN r.nombre
    "#;
    
    let mut _result = db.execute(
        query(time_query)
            .param("user_id", profile.user_id.as_str())
            .param("range_name", time_range)
    ).await
    .context("Error al guardar preferencia de tiempo")?;
    
    // Procesar el resultado
    while let Ok(Some(_)) = _result.next().await {
        // Procesar filas si las hay
    }
    
    // Guardar todas las resonancias emocionales significativas
    for (emotion, weight) in &profile.emotions {
        if *weight > 0.1 { // Solo guardar emociones con peso significativo
            let resonance_query = r#"
            MATCH (u:Usuario {id: $user_id})
            MATCH (e:Emocion {tipo: $emotion_type})
            MERGE (u)-[r:RESUENA_CON]->(e)
            ON CREATE SET r.intensidad = $weight, r.fecha = datetime()
            ON MATCH SET r.intensidad = $weight, r.fecha = datetime()
            RETURN e.tipo
            "#;
            
            let mut _result = db.execute(
                query(resonance_query)
                    .param("user_id", profile.user_id.as_str())
                    .param("emotion_type", emotion.as_str())
                    .param("weight", *weight)
            ).await
            .context(format!("Error al guardar resonancia con emoción: {}", emotion))?;
            
            // Procesar el resultado
            while let Ok(Some(_)) = _result.next().await {
                // Procesar filas si las hay
            }
        }
    }
    
    info!("Perfil emocional guardado correctamente para usuario: {}", profile.user_id);
    Ok(())
}

/// Obtiene información básica de un usuario
pub async fn get_user_info(
    db: &DbPool,
    user_id: &str,
) -> Result<Option<HashMap<String, String>>> {
    info!("Consultando información del usuario: {}", user_id);
    
    let query_text = r#"
    MATCH (u:Usuario {id: $user_id})
    OPTIONAL MATCH (u)-[:ESTADO_EMOCIONAL]->(e:Emocion)
    OPTIONAL MATCH (u)-[:PREFIERE_DURACION]->(r:RangoDuracion)
    RETURN 
        u.id as id,
        u.nombre as nombre,
        u.estado as estado,
        u.fecha_creacion as fecha_creacion,
        u.ultima_actividad as ultima_actividad,
        e.tipo as emocion_actual,
        r.nombre as tiempo_preferido
    "#;
    
    let mut result = db.execute(
        query(query_text)
            .param("user_id", user_id)
    ).await
    .context("Error al consultar información del usuario")?;
    
    if let Ok(Some(row)) = result.next().await {
        let mut user_info = HashMap::new();
        
        user_info.insert("id".to_string(), row.get::<String>("id").unwrap_or_default());
        user_info.insert("nombre".to_string(), row.get::<String>("nombre").unwrap_or_default());
        user_info.insert("estado".to_string(), row.get::<String>("estado").unwrap_or_default());
        
        if let Ok(emocion) = row.get::<String>("emocion_actual") {
            user_info.insert("emocion_actual".to_string(), emocion);
        }
        
        if let Ok(tiempo) = row.get::<String>("tiempo_preferido") {
            user_info.insert("tiempo_preferido".to_string(), tiempo);
        }
        
        info!("Información de usuario encontrada: {}", user_id);
        return Ok(Some(user_info));
    }
    
    info!("Usuario no encontrado: {}", user_id);
    Ok(None)
}

/// Obtiene el perfil emocional de un usuario (función existente mejorada)
pub async fn get_user_emotional_profile(
    db: &DbPool,
    user_id: &str,
) -> Result<Option<EmotionalProfile>> {
    info!("Consultando perfil emocional para usuario: {}", user_id);
    
    // Consulta para obtener el estado emocional dominante y el rango de tiempo preferido
    let profile_query = r#"
    MATCH (u:Usuario {id: $user_id})
    OPTIONAL MATCH (u)-[:ESTADO_EMOCIONAL]->(e:Emocion)
    OPTIONAL MATCH (u)-[:PREFIERE_DURACION]->(r:RangoDuracion)
    RETURN e.tipo AS dominant_emotion, r.nombre AS time_range_name
    "#;
    
    let mut result = db.execute(
        query(profile_query)
            .param("user_id", user_id)
    ).await
    .context("Error al consultar perfil emocional")?;
    
    if let Ok(Some(row)) = result.next().await {
        let dominant_emotion: Option<String> = row.get("dominant_emotion").unwrap_or(None);
        let time_range_name: Option<String> = row.get("time_range_name").unwrap_or(None);
        
        if dominant_emotion.is_none() {
            info!("No se encontró perfil emocional para el usuario: {}", user_id);
            return Ok(None);
        }
        
        // Consulta para obtener todas las resonancias emocionales
        let emotions_query = r#"
        MATCH (u:Usuario {id: $user_id})-[r:RESUENA_CON]->(e:Emocion)
        RETURN e.tipo AS emotion, r.intensidad AS weight
        "#;
        
        let mut emotions_result = db.execute(
            query(emotions_query)
                .param("user_id", user_id)
        ).await
        .context("Error al consultar resonancias emocionales")?;
        
        let mut emotions = HashMap::new();
        while let Ok(Some(row)) = emotions_result.next().await {
            let emotion: String = row.get("emotion").unwrap_or_default();
            let weight: f64 = row.get("weight").unwrap_or(0.0);
            emotions.insert(emotion, weight);
        }
        
        // Determinar el rango de duración
        let time_range = match time_range_name.as_deref() {
            Some("muy_corto") => crate::models::questionnaire::DurationRange::MuyCorto,
            Some("corto") => crate::models::questionnaire::DurationRange::Corto,
            Some("medio") => crate::models::questionnaire::DurationRange::Medio,
            Some("largo") => crate::models::questionnaire::DurationRange::Largo,
            Some("muy_largo") => crate::models::questionnaire::DurationRange::MuyLargo,
            _ => crate::models::questionnaire::DurationRange::Medio, // valor por defecto
        };
        
        let profile = EmotionalProfile {
            user_id: user_id.to_string(),
            emotions,
            dominant_emotion: dominant_emotion.unwrap_or_else(|| "neutral".to_string()),
            time_available: time_range,
        };
        
        info!("Perfil emocional encontrado para usuario: {}", user_id);
        return Ok(Some(profile));
    }
    
    info!("No se encontró perfil emocional para el usuario: {}", user_id);
    Ok(None)
}