//--------------------------------------------------
// GAME SOUL - MANEJO DE ERRORES
//
// Este archivo define los tipos de errores personalizados 
// para la aplicación y proporciona implementaciones para 
// convertir estos errores en respuestas HTTP.
//--------------------------------------------------

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de base de datos: {0}")]
    DatabaseError(String),
    
    #[error("Error de autenticación: {0}")]
    AuthError(String),
    
    #[error("No encontrado: {0}")]
    NotFoundError(String),
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error interno del servidor: {0}")]
    InternalError(String),
    
    #[error("Error de configuración: {0}")]
    ConfigError(String),
}


impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

impl From<neo4rs::Error> for AppError {
    fn from(err: neo4rs::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(message) => {
                log::error!("Error de base de datos: {}", message);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    status: "error".to_string(),
                    message: "Error en la base de datos".to_string(),
                })
            }
            AppError::AuthError(message) => {
                log::warn!("Error de autenticación: {}", message);
                HttpResponse::Unauthorized().json(ErrorResponse {
                    status: "error".to_string(),
                    message: message.clone(),
                })
            }
            AppError::NotFoundError(message) => {
                log::debug!("Recurso no encontrado: {}", message);
                HttpResponse::NotFound().json(ErrorResponse {
                    status: "error".to_string(),
                    message: message.clone(),
                })
            }
            AppError::ValidationError(message) => {
                log::debug!("Error de validación: {}", message);
                HttpResponse::BadRequest().json(ErrorResponse {
                    status: "error".to_string(),
                    message: message.clone(),
                })
            }
            AppError::ConfigError(message) => {
                log::error!("Error de configuración: {}", message);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    status: "error".to_string(),
                    message: "Error de configuración del servidor".to_string(),
                })
            }
            AppError::InternalError(message) => {
                log::error!("Error interno del servidor: {}", message);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    status: "error".to_string(),
                    message: "Error interno del servidor".to_string(),
                })
            }
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;