//--------------------------------------------------
// GAME SOUL - MODELO DE RECOMENDACIÓN
//
// Este archivo define las estructuras de datos relacionadas
// con las recomendaciones de juegos y solicitudes.
//--------------------------------------------------

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Estructura que detalla los componentes de la puntuación de resonancia
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResonanciaDesglosada {
    /// Puntuación basada en la conexión directa entre juego y emoción
    pub directa: f64,
    /// Puntuación basada en la conexión a través de géneros
    pub por_genero: f64,
    /// Puntuación basada en la conexión a través de características
    pub por_caracteristica: f64,
}

/// Estructura que representa una recomendación de juego
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameRecommendation {
    /// Identificador único del juego
    pub id: String,
    /// Nombre del juego
    pub nombre: String,
    /// Descripción del juego
    pub descripcion: String,
    /// Puntuación de resonancia emocional (más alta = mejor coincidencia)
    pub resonancia: f64,
    /// Desglose de la puntuación de resonancia (opcional)
    pub resonancia_desglosada: Option<ResonanciaDesglosada>,
    /// Géneros del juego
    pub generos: Vec<String>,
    /// Características del juego
    pub caracteristicas: Vec<String>,
    /// Emociones con las que coincide
    pub emociones_coincidentes: Vec<String>,
}

/// Solicitud para obtener recomendaciones de juegos directamente
#[derive(Debug, Deserialize, Validate)]
pub struct RecommendationRequest {
    /// Estado emocional actual del usuario
    #[validate(length(min = 1, message = "El estado emocional no puede estar vacío"))]
    pub estado_emocional: String,
    
    /// Tiempo disponible en minutos
    #[validate(range(min = 5, max = 1440, message = "El tiempo debe estar entre 5 minutos y 24 horas"))]
    pub tiempo_disponible: Option<i32>,
    
    /// Características que el usuario quiere evitar
    pub dealbreakers: Option<Vec<String>>,
    
    /// Si se deben incluir recomendaciones exploratorias
    pub incluir_exploracion: Option<bool>,
}

/// Respuesta con recomendaciones de juegos
#[derive(Debug, Serialize)]
pub struct RecommendationResponse {
    /// Recomendaciones basadas en el estado emocional
    pub recomendaciones_emocionales: Vec<GameRecommendation>,
    
    /// Recomendaciones exploratorias para evitar fatiga (opcional)
    pub recomendaciones_exploracion: Option<Vec<GameRecommendation>>,
}

/// Solicitud para proporcionar feedback sobre una recomendación
#[derive(Debug, Deserialize, Validate)]
pub struct FeedbackRequest {
    /// ID del usuario
    #[validate(length(min = 1, message = "El ID de usuario no puede estar vacío"))]
    pub user_id: String,
    
    /// ID del juego
    #[validate(length(min = 1, message = "El ID del juego no puede estar vacío"))]
    pub game_id: String,
    
    /// Satisfacción con la recomendación (1-5)
    #[validate(range(min = 1, max = 5, message = "La satisfacción debe estar entre 1 y 5"))]
    pub satisfaction: i32,
    
    /// Emociones experimentadas jugando (opcional)
    pub emotions_experienced: Option<Vec<String>>,
    
    /// Comentarios adicionales (opcional)
    pub comments: Option<String>,
}

// Métodos de implementación para RecommendationRequest
impl RecommendationRequest {
    /// Obtiene el tiempo disponible o un valor predeterminado
    pub fn get_tiempo_disponible(&self) -> i32 {
        self.tiempo_disponible.unwrap_or(60) // 60 minutos por defecto
    }
    
    /// Obtiene las características a evitar o un vector vacío
    pub fn get_dealbreakers(&self) -> Vec<String> {
        self.dealbreakers.clone().unwrap_or_else(Vec::new)
    }
    
    /// Determina si se deben incluir recomendaciones exploratorias
    pub fn should_include_exploration(&self) -> bool {
        self.incluir_exploracion.unwrap_or(true)
    }
}

// Métodos de implementación para RecommendationResponse
impl RecommendationResponse {
    /// Crea una nueva respuesta con las recomendaciones dadas
    pub fn new(
        recomendaciones_emocionales: Vec<GameRecommendation>,
        recomendaciones_exploracion: Option<Vec<GameRecommendation>>,
    ) -> Self {
        RecommendationResponse {
            recomendaciones_emocionales,
            recomendaciones_exploracion,
        }
    }
}