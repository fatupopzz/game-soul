use anyhow::{Context, Result};
use neo4rs::query;
use log::info;
use std::collections::HashMap;

use crate::db::neo4j::client::DbPool;
use crate::models::questionnaire::EmotionalProfile;

/// Guarda el perfil emocional del usuario
pub async fn save_emotional_profile(
    db: &DbPool,
    profile: &EmotionalProfile,
) -> Result<()> {
    info!("Guardando perfil emocional para usuario: {}", profile.user_id);
    
    // Asegurar que el usuario existe
    let create_user_query = "MERGE (u:Usuario {id: $user_id}) RETURN u.id";
    db.execute(
        query(create_user_query)
            .param("user_id", &profile.user_id)
    ).await
    .context("Error al crear/verificar usuario")?;
    
    // Guardar el estado emocional dominante
    let emotion_query = r#"
    MATCH (u:Usuario {id: $user_id})
    MATCH (e:Emocion {tipo: $emotion_type})
    MERGE (u)-[r:ESTADO_EMOCIONAL]->(e)
    ON CREATE SET r.fecha = datetime()
    ON MATCH SET r.fecha = datetime()
    RETURN e.tipo
    "#;
    
    db.execute(
        query(emotion_query)
            .param("user_id", &profile.user_id)
            .param("emotion_type", &profile.dominant_emotion)
    ).await
    .context("Error al guardar estado emocional dominante")?;
    
    // Guardar todas las resonancias emocionales
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
            
            db.execute(
                query(resonance_query)
                    .param("user_id", &profile.user_id)
                    .param("emotion_type", emotion)
                    .param("weight", *weight)
            ).await
            .context(format!("Error al guardar resonancia con emoción: {}", emotion))?;
        }
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
    
    db.execute(
        query(time_query)
            .param("user_id", &profile.user_id)
            .param("range_name", time_range)
    ).await
    .context("Error al guardar preferencia de tiempo")?;
    
    info!("Perfil emocional guardado correctamente");
    Ok(())
}

/// Obtiene el perfil emocional de un usuario
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
        let dominant_emotion: Option<String> = row.get("dominant_emotion");
        let time_range_name: Option<String> = row.get("time_range_name");
        
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