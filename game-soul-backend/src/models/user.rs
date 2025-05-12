use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Estructura que representa un usuario en el sistema
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// Identificador único del usuario
    pub id: String,
    /// Nombre de usuario
    pub username: String,
    /// Email (opcional)
    pub email: Option<String>,
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    /// Fecha de última actualización
    pub updated_at: DateTime<Utc>,
}

/// Historial de juegos de un usuario
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GamePlayHistory {
    /// ID del usuario
    pub user_id: String,
    /// ID del juego
    pub game_id: String,
    /// Nombre del juego (para facilitar visualización)
    pub game_name: String,
    /// Fecha de primera jugada
    pub first_played_at: DateTime<Utc>,
    /// Fecha de última jugada
    pub last_played_at: DateTime<Utc>,
    /// Tiempo total jugado (en minutos)
    pub total_play_time: i32,
    /// Satisfacción reportada (1-5)
    pub satisfaction: Option<i32>,
    /// Emociones reportadas
    pub emotions_reported: Vec<String>,
}

/// Solicitud para registrar o actualizar un usuario
#[derive(Debug, Deserialize, Validate)]
pub struct UserRegistrationRequest {
    /// Nombre de usuario deseado
    #[validate(length(min = 3, max = 50, message = "El nombre de usuario debe tener entre 3 y 50 caracteres"))]
    pub username: String,
    
    /// Email (opcional)
    #[validate(email(message = "El email debe tener un formato válido"))]
    pub email: Option<String>,
}

/// Respuesta a una solicitud de registro o actualización
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// ID del usuario
    pub id: String,
    /// Nombre de usuario
    pub username: String,
    /// Email (si se proporcionó)
    pub email: Option<String>,
    /// Fecha de creación o actualización
    pub timestamp: DateTime<Utc>,
}