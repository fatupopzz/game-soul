use actix_web::{web, HttpResponse};
use log::{error, info};
use validator::Validate;

use crate::db::neo4j::client::DbPool;
use crate::db::neo4j::queries::recommendations::{get_emotional_recommendations, get_exploration_recommendations, save_recommendation_feedback};
use crate::error::{AppError, AppResult};
use crate::models::recommendation::{RecommendationRequest, RecommendationResponse, FeedbackRequest};

/// Controlador para obtener recomendaciones de juegos directamente
pub async fn get_recommendations(
    db: web::Data<DbPool>,
    req: web::Json<RecommendationRequest>,
) -> AppResult<HttpResponse> {
    // Validar la solicitud
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // En una implementación real, obtendríamos el ID del usuario del token JWT
    // Por ahora, usamos un ID genérico para desarrollo
    let user_id = "user_test";
    
    info!("Procesando solicitud de recomendación para usuario: {}", user_id);
    info!("Estado emocional: {}", req.estado_emocional);
    info!("Tiempo disponible: {} minutos", req.get_tiempo_disponible());
    
    // Obtener recomendaciones basadas en estado emocional
    let emotion_recommendations = get_emotional_recommendations(
        &db,
        user_id,
        &req.estado_emocional,
        req.get_tiempo_disponible(),
        req.get_dealbreakers(),
    )
    .await
    .map_err(|e| {
        error!("Error al obtener recomendaciones emocionales: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;
    
    // Opcional: Obtener recomendaciones de exploración para prevenir fatiga
    let exploration_recommendations = if req.should_include_exploration() {
        match get_exploration_recommendations(&db, user_id, req.get_tiempo_disponible()).await {
            Ok(recommendations) => {
                info!("Obtenidas {} recomendaciones exploratorias", recommendations.len());
                Some(recommendations)
            }
            Err(e) => {
                error!("Error al obtener recomendaciones exploratorias: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    // Crear la respuesta
    let response = RecommendationResponse::new(
        emotion_recommendations,
        exploration_recommendations,
    );
    
    info!("Enviando {} recomendaciones emocionales y {} exploratorias", 
          response.recomendaciones_emocionales.len(),
          response.recomendaciones_exploracion.as_ref().map_or(0, |v| v.len()));
    
    // Devolver respuesta JSON
    Ok(HttpResponse::Ok().json(response))
}

/// Controlador para proporcionar feedback sobre una recomendación
pub async fn provide_feedback(
    db: web::Data<DbPool>,
    req: web::Json<FeedbackRequest>,
) -> AppResult<HttpResponse> {
    // Validar la solicitud
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    info!("Procesando feedback para usuario: {}, juego: {}, satisfacción: {}", 
          req.user_id, req.game_id, req.satisfaction);
    
    // Guardar el feedback en la base de datos
    save_recommendation_feedback(
        &db, 
        &req.user_id, 
        &req.game_id, 
        req.satisfaction, 
        req.emotions_experienced.clone()
    )
    .await
    .map_err(|e| {
        error!("Error al guardar feedback: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;
    
    // Devolver respuesta de éxito
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Feedback procesado correctamente"
    })))
}