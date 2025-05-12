//--------------------------------------------------
// GAME SOUL - HANDLERS DE REFERENCIA
//
// Este archivo contiene los handlers para obtener información
// sobre emociones, características y géneros disponibles
// en el sistema.
//--------------------------------------------------
use actix_web::{web, HttpResponse};
use log::info;

use crate::db::neo4j::client::DbPool;
use crate::db::neo4j::queries::reference;
use crate::error::AppResult;
use crate::models::emotion::{
    get_available_emotions, get_available_characteristics
};

/// Obtiene todas las emociones disponibles
pub async fn get_emotions(
    db: web::Data<DbPool>
) -> AppResult<HttpResponse> {
    info!("Obteniendo lista de emociones disponibles");
    
    // Primero intentamos obtener desde la base de datos
    let emotions = match reference::get_all_emotions(&db).await {
        Ok(emotions) if !emotions.is_empty() => {
            info!("Obtenidas {} emociones de la base de datos", emotions.len());
            emotions.into_iter()
                .map(|tipo| crate::models::emotion::Emotion {
                    tipo,
                    descripcion: None,
                })
                .collect()
        },
        _ => {
            // Si falla o está vacío, usar el modelo predefinido
            info!("Usando lista de emociones predefinida");
            get_available_emotions()
        }
    };
    
    Ok(HttpResponse::Ok().json(emotions))
}

/// Obtiene todas las características disponibles
pub async fn get_characteristics(
    db: web::Data<DbPool>
) -> AppResult<HttpResponse> {
    info!("Obteniendo lista de características disponibles");
    
    // Primero intentamos obtener desde la base de datos
    let characteristics = match reference::get_all_characteristics(&db).await {
        Ok(characteristics) if !characteristics.is_empty() => {
            info!("Obtenidas {} características de la base de datos", characteristics.len());
            characteristics.into_iter()
                .map(|nombre| crate::models::emotion::Characteristic {
                    nombre,
                    descripcion: None,
                })
                .collect()
        },
        _ => {
            // Si falla o está vacío, usar el modelo predefinido
            info!("Usando lista de características predefinida");
            get_available_characteristics()
        }
    };
    
    Ok(HttpResponse::Ok().json(characteristics))
}

/// Obtiene los géneros de juegos desde la base de datos
pub async fn get_genres(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    info!("Obteniendo géneros de juegos desde la base de datos");
    
    let genres = reference::get_all_genres(&db).await?;
    
    Ok(HttpResponse::Ok().json(genres))
}

/// Obtiene los rangos de duración disponibles
pub async fn get_duration_ranges(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    info!("Obteniendo rangos de duración");
    
    let ranges = reference::get_all_duration_ranges(&db).await?;
    
    let formatted_ranges = ranges.into_iter()
        .map(|(nombre, min, max, descripcion)| {
            serde_json::json!({
                "nombre": nombre,
                "min": min,
                "max": max,
                "descripcion": descripcion
            })
        })
        .collect::<Vec<_>>();
    
    Ok(HttpResponse::Ok().json(formatted_ranges))
}