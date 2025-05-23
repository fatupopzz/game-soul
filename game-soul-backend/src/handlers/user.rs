//--------------------------------------------------
// GAME SOUL - HANDLER DE USUARIOS
//
// Este archivo contiene los handlers para gestionar
// usuarios y sus perfiles en la aplicación.
//--------------------------------------------------

use actix_web::{web, HttpResponse};
use log::{info, error};

use crate::db::neo4j::client::DbPool;
use crate::db::neo4j::queries::user;
use crate::error::{AppError, AppResult};

/// Obtener información de un usuario específico
pub async fn get_user_info(
    db: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = user_id.into_inner();
    
    info!("Obteniendo información del usuario: {}", user_id);
    
    match user::get_user_info(&db, &user_id).await {
        Ok(Some(user_info)) => {
            info!("Información de usuario encontrada: {}", user_id);
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "user": user_info
            })))
        },
        Ok(None) => {
            info!("Usuario no encontrado: {}", user_id);
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "status": "not_found",
                "message": "Usuario no encontrado"
            })))
        },
        Err(e) => {
            error!("Error al obtener información del usuario {}: {}", user_id, e);
            Err(AppError::DatabaseError(format!("Error al consultar usuario: {}", e)))
        }
    }
}

/// Obtener el perfil emocional de un usuario
pub async fn get_user_profile(
    db: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = user_id.into_inner();
    
    info!("Obteniendo perfil emocional del usuario: {}", user_id);
    
    match user::get_user_emotional_profile(&db, &user_id).await {
        Ok(Some(profile)) => {
            info!("Perfil emocional encontrado para usuario: {}", user_id);
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "profile": profile
            })))
        },
        Ok(None) => {
            info!("Perfil emocional no encontrado para usuario: {}", user_id);
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "status": "not_found",
                "message": "Perfil emocional no encontrado"
            })))
        },
        Err(e) => {
            error!("Error al obtener perfil emocional del usuario {}: {}", user_id, e);
            Err(AppError::DatabaseError(format!("Error al consultar perfil: {}", e)))
        }
    }
}