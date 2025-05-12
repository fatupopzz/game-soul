use actix_web::{web, HttpResponse};
use log::{error, info};
use std::collections::HashMap;
use validator::Validate;

use crate::db::neo4j::client::DbPool;
use crate::db::neo4j::queries::recommendations::{get_emotional_recommendations, get_exploration_recommendations};
use crate::db::neo4j::queries::user::save_emotional_profile;
use crate::error::{AppError, AppResult};
use crate::models::questionnaire::{
    create_questionnaire, DurationRange, EmotionalProfile, QuestionnaireResponse,
    QuestionnaireSubmission, QuestionOptionValue,
};
use crate::models::recommendation::RecommendationResponse;


/// Obtener el cuestionario completo con las 5 preguntas
pub async fn get_questionnaire() -> AppResult<HttpResponse> {
    info!("Solicitud de obtención del cuestionario");
    
    // Crear el cuestionario con las preguntas predefinidas
    let questions = create_questionnaire();
    
    // Obtener lista de emociones disponibles
    let available_emotions = crate::models::emotion::get_available_emotions()
        .into_iter()
        .map(|e| e.tipo)
        .collect();
    
    // Obtener lista de características que pueden ser dealbreakers
    let available_characteristics = crate::models::emotion::get_dealbreaker_characteristics();
    
    info!("Enviando cuestionario con {} preguntas, {} emociones y {} características", 
        questions.len(), 
        available_emotions.len(),
        available_characteristics.len());
    
    // Crear la respuesta
    let response = QuestionnaireResponse {
        questions,
        available_emotions,
        available_characteristics,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Procesar el cuestionario enviado y devolver recomendaciones
pub async fn submit_questionnaire(
    db: web::Data<DbPool>,
    req: web::Json<QuestionnaireSubmission>,
) -> AppResult<HttpResponse> {
    // Validar la solicitud
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    info!("Procesando respuestas del cuestionario para usuario: {}", req.user_id);
    
    // Obtener las preguntas del cuestionario
    let questions = create_questionnaire();
    
    // Verificar que se hayan respondido todas las preguntas
    for question in &questions {
        if !req.answers.contains_key(&question.id) {
            return Err(AppError::ValidationError(
                format!("Falta respuesta para la pregunta: {}", question.id)
            ));
        }
    }
    
    // Inicializar el perfil emocional
    let mut emotional_profile: HashMap<String, f64> = HashMap::new();
    let mut time_range = DurationRange::Medio; // Valor por defecto
    
    // Procesar cada respuesta del usuario
    for (question_id, option_id) in &req.answers {
        // Buscar la pregunta correspondiente
        if let Some(question) = questions.iter().find(|q| &q.id == question_id) {
            // Buscar la opción seleccionada
            if let Some(option) = question.options.iter().find(|o| &o.id == option_id) {
                info!("Procesando respuesta '{}' para pregunta '{}'", option.text, question.text);
                
                match &option.value {
                    // Si es un mapeo emocional, agregar al perfil emocional
                    QuestionOptionValue::EmotionMapping(emotions) => {
                        for (emotion, intensity) in emotions {
                            let current = emotional_profile.entry(emotion.clone()).or_insert(0.0);
                            *current += intensity;
                            info!("  Añadiendo emoción '{}' con intensidad {:.2}", emotion, intensity);
                        }
                    },
                    // Si es tiempo disponible, guardar el rango
                    QuestionOptionValue::TimeValue(range) => {
                        time_range = range.clone();
                        info!("  Tiempo disponible: {} ({})", range.get_description(), range.get_db_name());
                    },
                    // Otros valores posibles
                    _ => {}
                }
            } else {
                return Err(AppError::ValidationError(
                    format!("Opción no válida '{}' para pregunta '{}'", option_id, question_id)
                ));
            }
        }
    }
    
    // Normalizar el perfil emocional (para que las intensidades sumen 1.0)
    let sum: f64 = emotional_profile.values().sum();
    if sum > 0.0 {
        for (emotion, value) in emotional_profile.iter_mut() {
            *value /= sum;
            info!("Emoción normalizada: {} = {:.2}", emotion, value);
        }
    }
    
    // Encontrar la emoción dominante
    let dominant_emotion = emotional_profile
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(k, v)| {
            info!("Emoción dominante: {} ({:.2})", k, v);
            k.clone()
        })
        .unwrap_or_else(|| {
            info!("No se encontró emoción dominante, usando 'neutral'");
            "neutral".to_string()
        });
    
    // Crear objeto para almacenar perfil emocional completo
    let profile = EmotionalProfile {
        user_id: req.user_id.clone(),
        emotions: emotional_profile.clone(),
        dominant_emotion: dominant_emotion.clone(),
        time_available: time_range.clone(),
    };
    
    // Obtener el valor mínimo del rango de tiempo
    let (min_time, _) = time_range.get_range_values();
    
    // Obtener dealbreakers
    let dealbreakers = req.dealbreakers.clone().unwrap_or_else(Vec::new);
    if !dealbreakers.is_empty() {
        info!("Características a evitar: {:?}", dealbreakers);
    }
    
    // Obtener recomendaciones basadas en el perfil emocional
    info!("Buscando recomendaciones para emoción '{}', tiempo mínimo: {}", dominant_emotion, min_time);
    
    let emotional_recommendations = get_emotional_recommendations(
        &db,
        &req.user_id,
        &dominant_emotion,
        min_time,
        dealbreakers.clone(),
    )
    .await
    .map_err(|e| {
        error!("Error al obtener recomendaciones emocionales: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;
    
    info!("Encontradas {} recomendaciones emocionales", emotional_recommendations.len());
    
    // Obtener recomendaciones exploratorias si hay pocas recomendaciones emocionales
    let exploration_recommendations = if emotional_recommendations.len() < 3 {
        info!("Pocas recomendaciones emocionales, buscando alternativas exploratorias");
        
        match get_exploration_recommendations(&db, &req.user_id, min_time).await {
            Ok(recommendations) => {
                info!("Encontradas {} recomendaciones exploratorias", recommendations.len());
                if !recommendations.is_empty() {
                    Some(recommendations)
                } else {
                    None
                }
            },
            Err(e) => {
                error!("Error al obtener recomendaciones exploratorias: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    // Guardar el perfil emocional del usuario en la base de datos
    if let Err(e) = save_emotional_profile(&db, &profile).await {
        error!("Error al guardar perfil emocional: {}", e);
        // No retornamos el error para no interrumpir la respuesta
    }
    
    // Crear la respuesta con las recomendaciones
    let response = RecommendationResponse::new(
        emotional_recommendations,
        exploration_recommendations,
    );
    
    info!("Enviando respuesta con {} recomendaciones emocionales y {} exploratorias", 
          response.recomendaciones_emocionales.len(),
          response.recomendaciones_exploracion.as_ref().map_or(0, |r| r.len()));
    
    Ok(HttpResponse::Ok().json(response))
}

/// Obtener el historial de respuestas del cuestionario de un usuario
pub async fn get_questionnaire_history(
    db: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = user_id.into_inner();
    
    info!("Obteniendo historial de cuestionarios para usuario: {}", user_id);
    
    // En una implementación completa, aquí obtendríamos el historial desde la base de datos
    // Por ahora, devolvemos un mensaje indicando que la funcionalidad no está implementada
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "info",
        "message": "Funcionalidad en desarrollo",
        "user_id": user_id
    })))
}