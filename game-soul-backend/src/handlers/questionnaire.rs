use actix_web::{web, HttpResponse};
use log::{error, info, debug};
use std::collections::HashMap;
use validator::Validate;

use crate::db::neo4j::client::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::questionnaire::{
    create_questionnaire, DurationRange, EmotionalProfile, QuestionnaireResponse,
    QuestionnaireSubmission, QuestionOptionValue,
};
use crate::models::recommendation::{RecommendationResponse, GameRecommendation};


/// Obtener el cuestionario completo con las 5 preguntas
pub async fn get_questionnaire() -> AppResult<HttpResponse> {
    info!("Solicitud de obtención del cuestionario");
    
    // Crear el cuestionario con las preguntas predefinidas
    let questions = create_questionnaire();
    
    // Obtener lista de emociones disponibles
    let available_emotions: Vec<String> = crate::models::emotion::get_available_emotions()
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
    _db: web::Data<DbPool>, // No usamos la base de datos para nada
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
    
    // Obtener dealbreakers
    let dealbreakers = req.dealbreakers.clone().unwrap_or_else(Vec::new);
    if !dealbreakers.is_empty() {
        info!("Características a evitar: {:?}", dealbreakers);
    }
    
    // Crear recomendaciones automáticas según la emoción dominante
    let emotional_recommendations = match dominant_emotion.as_str() {
        "relajante" => vec![
            GameRecommendation {
                id: "stardew_valley".to_string(),
                nombre: "Stardew Valley".to_string(),
                descripcion: "Un juego de simulación de granja en el que puedes cultivar, pescar, minar y hacer amigos.".to_string(),
                resonancia: 0.95,
                resonancia_desglosada: None,
                generos: vec!["simulación".to_string()],
                caracteristicas: vec!["relajante".to_string(), "social".to_string()],
                emociones_coincidentes: vec!["relajante".to_string()],
            },
            GameRecommendation {
                id: "animal_crossing".to_string(),
                nombre: "Animal Crossing: New Horizons".to_string(),
                descripcion: "Un juego de simulación de vida donde construyes una comunidad en una isla desierta.".to_string(),
                resonancia: 0.9,
                resonancia_desglosada: None,
                generos: vec!["simulación".to_string()],
                caracteristicas: vec!["coleccionable".to_string(), "relajante".to_string()],
                emociones_coincidentes: vec!["relajante".to_string()],
            },
            GameRecommendation {
                id: "journey".to_string(),
                nombre: "Journey".to_string(),
                descripcion: "Una aventura atmosférica donde exploras un desierto místico.".to_string(),
                resonancia: 0.85,
                resonancia_desglosada: None,
                generos: vec!["aventura".to_string()],
                caracteristicas: vec!["atmósfera".to_string(), "artístico".to_string()],
                emociones_coincidentes: vec!["contemplativo".to_string(), "relajante".to_string()],
            },
        ],
        "desafiante" => vec![
            GameRecommendation {
                id: "elden_ring".to_string(),
                nombre: "Elden Ring".to_string(),
                descripcion: "Un juego de rol de acción en un vasto mundo abierto con combate desafiante.".to_string(),
                resonancia: 0.95,
                resonancia_desglosada: None,
                generos: vec!["rpg".to_string(), "acción".to_string()],
                caracteristicas: vec!["combate".to_string(), "exploración".to_string(), "atmósfera".to_string()],
                emociones_coincidentes: vec!["desafiante".to_string()],
            },
            GameRecommendation {
                id: "dark_souls".to_string(),
                nombre: "Dark Souls".to_string(),
                descripcion: "Un juego de rol de acción conocido por su dificultad y combate táctico.".to_string(),
                resonancia: 0.9,
                resonancia_desglosada: None,
                generos: vec!["rpg".to_string(), "acción".to_string()],
                caracteristicas: vec!["difícil".to_string(), "combate".to_string(), "atmósfera".to_string()],
                emociones_coincidentes: vec!["desafiante".to_string()],
            },
        ],
        "exploración" => vec![
            GameRecommendation {
                id: "breath_of_the_wild".to_string(),
                nombre: "The Legend of Zelda: Breath of the Wild".to_string(),
                descripcion: "Un juego de aventuras en un vasto mundo abierto con enfoque en la exploración.".to_string(),
                resonancia: 0.95,
                resonancia_desglosada: None,
                generos: vec!["aventura".to_string(), "acción".to_string()],
                caracteristicas: vec!["exploración".to_string(), "puzzles".to_string()],
                emociones_coincidentes: vec!["exploración".to_string()],
            },
            GameRecommendation {
                id: "no_mans_sky".to_string(),
                nombre: "No Man's Sky".to_string(),
                descripcion: "Un juego de exploración espacial con universo procedural y billones de planetas.".to_string(),
                resonancia: 0.9,
                resonancia_desglosada: None,
                generos: vec!["exploración".to_string(), "supervivencia".to_string()],
                caracteristicas: vec!["exploración".to_string(), "espacial".to_string()],
                emociones_coincidentes: vec!["exploración".to_string(), "contemplativo".to_string()],
            },
        ],
        _ => vec![
            GameRecommendation {
                id: "minecraft".to_string(),
                nombre: "Minecraft".to_string(),
                descripcion: "Un juego de mundo abierto que permite construir y explorar libremente.".to_string(),
                resonancia: 0.8,
                resonancia_desglosada: None,
                generos: vec!["sandbox".to_string()],
                caracteristicas: vec!["creativo".to_string(), "exploración".to_string()],
                emociones_coincidentes: vec!["creativo".to_string(), "exploración".to_string()],
            },
            GameRecommendation {
                id: "portal".to_string(),
                nombre: "Portal 2".to_string(),
                descripcion: "Un juego de puzzles en primera persona con portal guns y humor.".to_string(),
                resonancia: 0.75,
                resonancia_desglosada: None,
                generos: vec!["puzzle".to_string()],
                caracteristicas: vec!["puzzles".to_string(), "historia".to_string()],
                emociones_coincidentes: vec!["contemplativo".to_string()],
            },
        ],
    };
    
    // Filtrar por dealbreakers manualmente
    let filtered_recommendations = if dealbreakers.is_empty() {
        emotional_recommendations
    } else {
        emotional_recommendations
            .into_iter()
            .filter(|rec| {
                // Un juego pasa el filtro si NINGUNA de sus características está en la lista de dealbreakers
                !rec.caracteristicas.iter().any(|c| dealbreakers.contains(c))
            })
            .collect()
    };
    
    info!("Generadas {} recomendaciones manuales", filtered_recommendations.len());
    
    // Crear la respuesta con las recomendaciones
    let response = RecommendationResponse::new(
        filtered_recommendations,
        None,
    );
    
    info!("Enviando respuesta con {} recomendaciones emocionales", 
          response.recomendaciones_emocionales.len());
    
    Ok(HttpResponse::Ok().json(response))
}

/// Obtener el historial de respuestas del cuestionario de un usuario
pub async fn get_questionnaire_history(
    _db: web::Data<DbPool>,
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
