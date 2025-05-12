use std::collections::HashMap;

use crate::models::questionnaire::EmotionalProfile;
use crate::models::recommendation::GameRecommendation;

/// Calcula la puntuación de compatibilidad entre un perfil emocional y un juego
pub fn calculate_compatibility(
    profile: &EmotionalProfile,
    game: &GameRecommendation,
) -> f64 {
    let mut compatibility = 0.0;
    let mut total_weight = 0.0;
    
    // Iterar por cada emoción en el perfil y verificar si el juego coincide
    for (emotion, weight) in &profile.emotions {
        if game.emociones_coincidentes.contains(emotion) {
            // Sumar la contribución de esta emoción a la compatibilidad
            compatibility += weight * game.resonancia;
        }
        total_weight += weight;
    }
    
    // Normalizar la puntuación para que esté entre 0 y 1
    if total_weight > 0.0 {
        compatibility / total_weight
    } else {
        0.0
    }
}

/// Filtra y ordena recomendaciones basadas en un perfil emocional
pub fn filter_and_sort_recommendations(
    profile: &EmotionalProfile,
    recommendations: Vec<GameRecommendation>,
) -> Vec<GameRecommendation> {
    // Calcular compatibilidad para cada recomendación
    let mut scored_recommendations: Vec<(GameRecommendation, f64)> = recommendations
        .into_iter()
        .map(|game| {
            let compatibility = calculate_compatibility(profile, &game);
            (game, compatibility)
        })
        .collect();
    
    // Ordenar por compatibilidad (mayor a menor)
    scored_recommendations.sort_by(|a, b| 
        b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
    );
    
    // Devolver solo las recomendaciones, sin la puntuación
    scored_recommendations.into_iter()
        .map(|(game, _)| game)
        .collect()
}

/// Calcula el perfil emocional dominante basado en las respuestas del cuestionario
pub fn calculate_emotional_profile(
    answers: &HashMap<String, String>,
    questions: &[crate::models::questionnaire::QuestionnaireQuestion],
) -> HashMap<String, f64> {
    let mut profile: HashMap<String, f64> = HashMap::new();
    
    // Procesar cada respuesta
    for (question_id, option_id) in answers {
        // Buscar la pregunta correspondiente
        if let Some(question) = questions.iter().find(|q| &q.id == question_id) {
            // Buscar la opción seleccionada
            if let Some(option) = question.options.iter().find(|o| &o.id == option_id) {
                // Si es un mapeo emocional, agregar al perfil
                if let crate::models::questionnaire::QuestionOptionValue::EmotionMapping(emotions) = &option.value {
                    for (emotion, weight) in emotions {
                        let current = profile.entry(emotion.clone()).or_insert(0.0);
                        *current += weight;
                    }
                }
            }
        }
    }
    
    // Normalizar el perfil
    let sum: f64 = profile.values().sum();
    if sum > 0.0 {
        for weight in profile.values_mut() {
            *weight /= sum;
        }
    }
    
    profile
}