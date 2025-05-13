//quiero hacer esto mas corto pero no se como
//aaaaa
//--------------------------------------------

use actix_web::{web, HttpResponse};
use log::{info, error, debug};
use std::collections::HashMap;
use validator::Validate;

use crate::db::neo4j::client::DbPool;
use crate::db::neo4j::queries::recommendations;
use crate::error::{AppError, AppResult};
use crate::models::questionnaire::{
    create_questionnaire, DurationRange, EmotionalProfile, QuestionnaireResponse,
    QuestionnaireSubmission, QuestionOptionValue,
};
use crate::models::recommendation::RecommendationResponse;

/// Obtener el cuestionario completo con las 5 preguntas
pub async fn get_questionnaire(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    info!("Solicitud de obtención del cuestionario");
    
    // Intentar diagnóstico de Neo4j
    if let Err(e) = recommendations::diagnose_database(&db).await {
        error!("Diagnóstico de Neo4j falló: {}", e);
    }
    
    // Crear el cuestionario con las preguntas predefinidas
    let questions = create_questionnaire();
    
    // Obtener emociones directamente desde Neo4j
    let available_emotions = match crate::db::neo4j::client::get_all_nodes_of_type(&db, "Emocion", "tipo").await {
        Ok(emotions) => {
            if emotions.is_empty() {
                error!("No se encontraron emociones en Neo4j");
                // Usar las emociones predefinidas como respaldo
                crate::models::emotion::get_available_emotions()
                    .into_iter()
                    .map(|e| e.tipo)
                    .collect()
            } else {
                info!("Obtenidas {} emociones desde Neo4j", emotions.len());
                emotions
            }
        },
        Err(e) => {
            error!("Error al obtener emociones desde Neo4j: {}", e);
            // Usar las emociones predefinidas como respaldo
            crate::models::emotion::get_available_emotions()
                .into_iter()
                .map(|e| e.tipo)
                .collect()
        }
    };
    
    // Obtener características desde Neo4j
    let available_characteristics = match crate::db::neo4j::client::get_all_nodes_of_type(&db, "Caracteristica", "nombre").await {
        Ok(characteristics) => {
            if characteristics.is_empty() {
                error!("No se encontraron características en Neo4j");
                // Usar las características predefinidas como respaldo
                crate::models::emotion::get_dealbreaker_characteristics()
            } else {
                info!("Obtenidas {} características desde Neo4j", characteristics.len());
                characteristics
            }
        },
        Err(e) => {
            error!("Error al obtener características desde Neo4j: {}", e);
            // Usar las características predefinidas como respaldo
            crate::models::emotion::get_dealbreaker_characteristics()
        }
    };
    
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
    
    // Inicializar el perfil emocional
    let mut emotional_profile: HashMap<String, f64> = HashMap::new();
    let mut time_range = DurationRange::Medio; // Valor por defecto
    
    // Procesar cada respuesta del usuario
    for (question_id, option_id) in &req.answers {
        // Buscar la pregunta correspondiente
        if let Some(question) = questions.iter().find(|q| &q.id == question_id) {
            // Buscar la opción seleccionada
            if let Some(option) = question.options.iter().find(|o| &o.id == option_id) {
                debug!("Procesando respuesta '{}' para pregunta '{}'", option.text, question.text);
                
                match &option.value {
                    // Si es un mapeo emocional, agregar al perfil emocional
                    QuestionOptionValue::EmotionMapping(emotions) => {
                        for (emotion, intensity) in emotions {
                            let current = emotional_profile.entry(emotion.clone()).or_insert(0.0);
                            *current += intensity;
                            debug!("  Añadiendo emoción '{}' con intensidad {:.2}", emotion, intensity);
                        }
                    },
                    // Si es tiempo disponible, guardar el rango
                    QuestionOptionValue::TimeValue(range) => {
                        time_range = range.clone();
                        debug!("  Tiempo disponible: {} ({})", range.get_description(), range.get_db_name());
                    },
                    // Otros valores posibles
                    _ => {}
                }
            }
        }
    }
    
    // Normalizar el perfil emocional
    let sum: f64 = emotional_profile.values().sum();
    if sum > 0.0 {
        for (emotion, value) in emotional_profile.iter_mut() {
            *value /= sum;
            debug!("Emoción normalizada: {} = {:.2}", emotion, value);
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
            info!("No se encontró emoción dominante, usando 'relajante'");
            "relajante".to_string()
        });
    
    // Obtener dealbreakers
    let dealbreakers = req.dealbreakers.clone().unwrap_or_else(Vec::new);
    if !dealbreakers.is_empty() {
        info!("Características a evitar: {:?}", dealbreakers);
    }
    
    // Obtener recomendaciones basadas en el perfil emocional
    info!("Buscando recomendaciones para emoción '{}'", dominant_emotion);
    
    let recommendations_result = recommendations::get_recommendations(
        &db,
        &dominant_emotion,
        &dealbreakers,
    ).await;
    
    // Procesar el resultado de la consulta
    let emotional_recommendations = match recommendations_result {
        Ok(recs) => {
            info!("Encontradas {} recomendaciones desde Neo4j", recs.len());
            recs
        },
        Err(e) => {
            error!("Error al obtener recomendaciones desde Neo4j: {}", e);
            
            // Generar recomendaciones de respaldo
            info!("Generando recomendaciones de respaldo");
            generate_fallback_recommendations(&dominant_emotion)
        }
    };
    
    // Crear la respuesta con las recomendaciones
    let response = RecommendationResponse::new(
        emotional_recommendations,
        None, // Sin recomendaciones exploratorias por ahora
    );
    
    info!("Enviando respuesta con {} recomendaciones", 
          response.recomendaciones_emocionales.len());
    
    Ok(HttpResponse::Ok().json(response))
}

// Función de respaldo para generar recomendaciones predefinidas
fn generate_fallback_recommendations(emotion_type: &str) -> Vec<crate::models::recommendation::GameRecommendation> {
    // Recomendaciones por tipo de emoción con IDs correctos según Neo4j
    match emotion_type {
        "relajante" => vec![
            crate::models::recommendation::GameRecommendation {
                id: "game1".to_string(),  // Stardew Valley
                nombre: "Stardew Valley".to_string(),
                descripcion: "Un juego de simulación de granja en el que puedes cultivar, pescar, minar y hacer amigos.".to_string(),
                resonancia: 0.95,
                resonancia_desglosada: None,
                generos: vec!["simulación".to_string()],
                caracteristicas: vec!["relajante".to_string(), "social".to_string()],
                emociones_coincidentes: vec!["relajante".to_string()],
            },
            crate::models::recommendation::GameRecommendation {
                id: "game4".to_string(),  // Animal Crossing
                nombre: "Animal Crossing: New Horizons".to_string(),
                descripcion: "Un juego de simulación de vida donde construyes una comunidad en una isla desierta.".to_string(),
                resonancia: 0.9,
                resonancia_desglosada: None,
                generos: vec!["simulación".to_string()],
                caracteristicas: vec!["coleccionable".to_string(), "relajante".to_string()],
                emociones_coincidentes: vec!["relajante".to_string()],
            },
            crate::models::recommendation::GameRecommendation {
                id: "game30".to_string(),  // Satisfactory
                nombre: "Satisfactory".to_string(),
                descripcion: "Un juego de construcción de fábricas en primera persona en un planeta alienígena".to_string(),
                resonancia: 0.7,
                resonancia_desglosada: None,
                generos: vec!["simulación".to_string(), "construcción".to_string()],
                caracteristicas: vec!["relajante".to_string(), "creativo".to_string()],
                emociones_coincidentes: vec!["relajante".to_string()],
            }
        ],
        "desafiante" => vec![
            crate::models::recommendation::GameRecommendation {
                id: "game16".to_string(),  // Elden Ring
                nombre: "Elden Ring".to_string(),
                descripcion: "Un RPG de acción de mundo abierto con combate desafiante y exploración no lineal".to_string(),
                resonancia: 0.9,
                resonancia_desglosada: None,
                generos: vec!["acción".to_string(), "RPG".to_string()],
                caracteristicas: vec!["desafiante".to_string(), "exploración".to_string()],
                emociones_coincidentes: vec!["desafiante".to_string()],
            },
            crate::models::recommendation::GameRecommendation {
                id: "game28".to_string(),  // Slay the Spire
                nombre: "Slay the Spire".to_string(),
                descripcion: "Un roguelike de construcción de mazos con estrategia por turnos".to_string(),
                resonancia: 0.9,
                resonancia_desglosada: None,
                generos: vec!["estrategia".to_string(), "roguelike".to_string()],
                caracteristicas: vec!["desafiante".to_string()],
                emociones_coincidentes: vec!["desafiante".to_string()],
            },
            crate::models::recommendation::GameRecommendation {
                id: "game12".to_string(),  // Hades
                nombre: "Hades".to_string(),
                descripcion: "Un roguelike de acción con narrativa rica y combate frenético".to_string(),
                resonancia: 0.85,
                resonancia_desglosada: None,
                generos: vec!["acción".to_string(), "roguelike".to_string()],
                caracteristicas: vec!["desafiante".to_string(), "narrativa".to_string()],
                emociones_coincidentes: vec!["desafiante".to_string()],
            }
        ],
        "exploración" => vec![
            crate::models::recommendation::GameRecommendation {
                id: "game21".to_string(),  // No Man's Sky
                nombre: "No Man's Sky".to_string(),
                descripcion: "Un juego de exploración espacial con universo procedural virtualmente infinito".to_string(),
                resonancia: 0.9,
                resonancia_desglosada: None,
                generos: vec!["exploración".to_string(), "aventura".to_string()],
                caracteristicas: vec!["exploración".to_string(), "espacial".to_string()],
                emociones_coincidentes: vec!["exploración".to_string()],
            },
            crate::models::recommendation::GameRecommendation {
                id: "game20".to_string(),  // God of War (2018)
                nombre: "God of War (2018)".to_string(),
                descripcion: "Una aventura de acción con combate visceral y narrativa emotiva padre-hijo".to_string(),
                resonancia: 0.8,
                resonancia_desglosada: None,
                generos: vec!["aventura".to_string(), "acción".to_string()],
                caracteristicas: vec!["exploración".to_string(), "combate".to_string()],
                emociones_coincidentes: vec!["exploración".to_string()],
            },
            crate::models::recommendation::GameRecommendation {
                id: "game27".to_string(),  // Subnautica
                nombre: "Subnautica".to_string(),
                descripcion: "Un juego de supervivencia y exploración submarina en un planeta alienígena".to_string(),
                resonancia: 0.85,
                resonancia_desglosada: None,
                generos: vec!["exploración".to_string(), "supervivencia".to_string()],
                caracteristicas: vec!["exploración".to_string(), "atmósfera".to_string()],
                emociones_coincidentes: vec!["exploración".to_string()],
            }
        ],
        "creativo" => vec![
            crate::models::recommendation::GameRecommendation {
                id: "game11".to_string(),  // Factorio
                nombre: "Factorio".to_string(),
                descripcion: "Un juego de construcción y gestión de fábricas con énfasis en la automatización".to_string(),
                resonancia: 0.9,
                resonancia_desglosada: None,
                generos: vec!["estrategia".to_string(), "construcción".to_string()],
                caracteristicas: vec!["creativo".to_string(), "optimización".to_string()],
                emociones_coincidentes: vec!["creativo".to_string()],
            },
            crate::models::recommendation::GameRecommendation {
                id: "game30".to_string(),  // Satisfactory
                nombre: "Satisfactory".to_string(),
                descripcion: "Un juego de construcción de fábricas en primera persona en un planeta alienígena".to_string(),
                resonancia: 0.85,
                resonancia_desglosada: None,
                generos: vec!["simulación".to_string(), "construcción".to_string()],
                caracteristicas: vec!["creativo".to_string(), "exploración".to_string()],
                emociones_coincidentes: vec!["creativo".to_string()],
            }
        ],
        "social" => vec![
            crate::models::recommendation::GameRecommendation {
                id: "game13".to_string(),  // Among Us
                nombre: "Among Us".to_string(),
                descripcion: "Un juego de deducción social donde identificas impostores entre la tripulación".to_string(),
                resonancia: 0.95,
                resonancia_desglosada: None,
                generos: vec!["fiesta".to_string(), "deducción".to_string()],
                caracteristicas: vec!["social".to_string(), "colaborativo".to_string()],
                emociones_coincidentes: vec!["social".to_string()],
            },
            crate::models::recommendation::GameRecommendation {
                id: "game29".to_string(),  // Final Fantasy XIV
                nombre: "Final Fantasy XIV".to_string(),
                descripcion: "Un MMORPG con rica narrativa, diversas clases y contenido variado".to_string(),
                resonancia: 0.85,
                resonancia_desglosada: None,
                generos: vec!["MMORPG".to_string(), "RPG".to_string()],
                caracteristicas: vec!["social".to_string(), "colaborativo".to_string()],
                emociones_coincidentes: vec!["social".to_string()],
            }
        ],
        _ => vec![
            crate::models::recommendation::GameRecommendation {
                id: "game27".to_string(),  // Subnautica
                nombre: "Subnautica".to_string(),
                descripcion: "Un juego de supervivencia y exploración submarina en un planeta alienígena".to_string(),
                resonancia: 0.8,
                resonancia_desglosada: None,
                generos: vec!["supervivencia".to_string(), "exploración".to_string()],
                caracteristicas: vec!["atmósfera".to_string(), "exploración".to_string()],
                emociones_coincidentes: vec!["contemplativo".to_string()],
            }
        ],
    }
}