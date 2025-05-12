use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;



/// Enum para los rangos de duración según la base de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DurationRange {
    MuyCorto,  // < 30 min
    Corto,     // 30-60 min
    Medio,     // 1-3 horas
    Largo,     // 3-8 horas
    MuyLargo   // > 8 horas
}

impl DurationRange {
    /// Obtener valores min/max en minutos
    pub fn get_range_values(&self) -> (i32, i32) {
        match self {
            DurationRange::MuyCorto => (0, 30),
            DurationRange::Corto => (30, 60),
            DurationRange::Medio => (60, 180),
            DurationRange::Largo => (180, 480),
            DurationRange::MuyLargo => (480, 9999),
        }
    }
    
    /// Obtener el nombre exacto como en la base de datos
    pub fn get_db_name(&self) -> &'static str {
        match self {
            DurationRange::MuyCorto => "muy_corto",
            DurationRange::Corto => "corto",
            DurationRange::Medio => "medio",
            DurationRange::Largo => "largo",
            DurationRange::MuyLargo => "muy_largo",
        }
    }
    
    /// Obtener la descripción exacta como en la base de datos
    pub fn get_description(&self) -> &'static str {
        match self {
            DurationRange::MuyCorto => "Menos de 30 minutos",
            DurationRange::Corto => "Entre 30 minutos y 1 hora",
            DurationRange::Medio => "Entre 1 y 3 horas",
            DurationRange::Largo => "Entre 3 y 8 horas",
            DurationRange::MuyLargo => "Más de 8 horas",
        }
    }
}

/// Tipos de preguntas en el cuestionario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    EmotionalState,
    TimeAvailable,
    ActivityPreference,
    MoodState,
    GoalState,
}

/// Valores posibles para las opciones de preguntas
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum QuestionOptionValue {
    /// Para mapeos emocionales (emoción -> intensidad)
    EmotionMapping(HashMap<String, f64>),
    
    /// Para tiempo disponible
    TimeValue(DurationRange),
    
    /// Para otras opciones simples
    StringValue(String),
}

/// Opción para una pregunta del cuestionario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: String,
    pub text: String,
    pub value: QuestionOptionValue,
}

/// Estructura para las preguntas del cuestionario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireQuestion {
    pub id: String,
    pub text: String,
    pub question_type: QuestionType,
    pub options: Vec<QuestionOption>,
}


/// Respuesta del usuario a una pregunta del cuestionario
/// Esta estructura se utiliza para validar las respuestas
/// Solicitud de envío del cuestionario completad
/// no se por que no funciona validate

#[derive(Debug, Deserialize, Validate)]
pub struct QuestionnaireSubmission {
    #[validate(length(min = 1, message = "Debe proporcionar un ID de usuario"))]
    pub user_id: String,
    
    #[validate(length(min = 1, message = "Debe proporcionar respuestas"))]
    pub answers: HashMap<String, String>, // question_id -> option_id
    
    pub dealbreakers: Option<Vec<String>>,
}


/// Perfil emocional calculado del usuario
#[derive(Debug, Serialize, Deserialize)]
pub struct EmotionalProfile {
    pub user_id: String,
    pub emotions: HashMap<String, f64>,
    pub dominant_emotion: String,
    pub time_available: DurationRange,
}

/// Respuesta con el cuestionario completo
#[derive(Debug, Serialize)]
pub struct QuestionnaireResponse {
    pub questions: Vec<QuestionnaireQuestion>,
    pub available_emotions: Vec<String>,
    pub available_characteristics: Vec<String>,
}

/// Crea el conjunto de preguntas del cuestionario
pub fn create_questionnaire() -> Vec<QuestionnaireQuestion> {
    vec![
        // Pregunta 1: Tipo de experiencia
        QuestionnaireQuestion {
            id: "tipo_experiencia".to_string(),
            text: "¿Qué tipo de experiencia buscas ahora mismo?".to_string(),
            question_type: QuestionType::EmotionalState,
            options: vec![
                QuestionOption {
                    id: "relajante".to_string(),
                    text: "Relajarme".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("relajante".to_string(), 0.9),
                        ("contemplativo".to_string(), 0.4),
                    ])),
                },
                QuestionOption {
                    id: "emocion".to_string(),
                    text: "Sentir emoción".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("desafiante".to_string(), 0.7),
                        ("alegre".to_string(), 0.6),
                    ])),
                },
                QuestionOption {
                    id: "desafio".to_string(),
                    text: "Desafiarme".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("desafiante".to_string(), 0.9),
                        ("competitivo".to_string(), 0.5),
                    ])),
                },
                QuestionOption {
                    id: "exploracion".to_string(),
                    text: "Explorar algo nuevo".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("exploración".to_string(), 0.9),
                        ("creativo".to_string(), 0.4),
                    ])),
                },
                QuestionOption {
                    id: "conexion".to_string(),
                    text: "Conectar con otros".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("social".to_string(), 0.9),
                        ("alegre".to_string(), 0.3),
                    ])),
                },
            ],
        },
        
        // Pregunta 2: Tiempo disponible
        QuestionnaireQuestion {
            id: "tiempo_disponible".to_string(),
            text: "¿Cuánto tiempo tienes disponible para jugar?".to_string(),
            question_type: QuestionType::TimeAvailable,
            options: vec![
                QuestionOption {
                    id: "muy_corto".to_string(),
                    text: "Muy poco (Menos de 30 minutos)".to_string(),
                    value: QuestionOptionValue::TimeValue(DurationRange::MuyCorto),
                },
                QuestionOption {
                    id: "corto".to_string(),
                    text: "Poco (Entre 30 minutos y 1 hora)".to_string(),
                    value: QuestionOptionValue::TimeValue(DurationRange::Corto),
                },
                QuestionOption {
                    id: "medio".to_string(),
                    text: "Moderado (Entre 1 y 3 horas)".to_string(),
                    value: QuestionOptionValue::TimeValue(DurationRange::Medio),
                },
                QuestionOption {
                    id: "largo".to_string(),
                    text: "Bastante (Entre 3 y 8 horas)".to_string(),
                    value: QuestionOptionValue::TimeValue(DurationRange::Largo),
                },
                QuestionOption {
                    id: "muy_largo".to_string(),
                    text: "Mucho (Más de 8 horas)".to_string(),
                    value: QuestionOptionValue::TimeValue(DurationRange::MuyLargo),
                },
            ],
        },
        
        // Pregunta 3: Estado de ánimo
        QuestionnaireQuestion {
            id: "estado_animo".to_string(),
            text: "¿Cómo describirías tu estado de ánimo actual?".to_string(),
            question_type: QuestionType::MoodState,
            options: vec![
                QuestionOption {
                    id: "energico".to_string(),
                    text: "Enérgico".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("alegre".to_string(), 0.7),
                        ("desafiante".to_string(), 0.6),
                        ("competitivo".to_string(), 0.5),
                    ])),
                },
                QuestionOption {
                    id: "tranquilo".to_string(),
                    text: "Tranquilo".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("relajante".to_string(), 0.8),
                        ("contemplativo".to_string(), 0.6),
                    ])),
                },
                QuestionOption {
                    id: "aburrido".to_string(),
                    text: "Aburrido".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("exploración".to_string(), 0.7),
                        ("desafiante".to_string(), 0.5),
                    ])),
                },
                QuestionOption {
                    id: "nostalgico".to_string(),
                    text: "Nostálgico".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("melancólico".to_string(), 0.8),
                        ("contemplativo".to_string(), 0.6),
                    ])),
                },
                QuestionOption {
                    id: "curioso".to_string(),
                    text: "Curioso".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("exploración".to_string(), 0.9),
                        ("creativo".to_string(), 0.5),
                    ])),
                },
                QuestionOption {
                    id: "estresado".to_string(),
                    text: "Estresado".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("relajante".to_string(), 0.8),
                        ("social".to_string(), 0.4),
                    ])),
                },
            ],
        },
        
        // Pregunta 4: Actividad preferida
        QuestionnaireQuestion {
            id: "actividad_preferida".to_string(),
            text: "Si tuvieras que elegir una actividad ahora mismo, ¿cuál sería?".to_string(),
            question_type: QuestionType::ActivityPreference,
            options: vec![
                QuestionOption {
                    id: "puzzle".to_string(),
                    text: "Resolver un puzzle".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("desafiante".to_string(), 0.7),
                        ("contemplativo".to_string(), 0.5),
                    ])),
                },
                QuestionOption {
                    id: "historia".to_string(),
                    text: "Contar una historia".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("creativo".to_string(), 0.8),
                        ("social".to_string(), 0.6),
                    ])),
                },
                QuestionOption {
                    id: "construir".to_string(),
                    text: "Construir algo".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("creativo".to_string(), 0.9),
                        ("relajante".to_string(), 0.4),
                    ])),
                },
                QuestionOption {
                    id: "competir".to_string(),
                    text: "Competir".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("competitivo".to_string(), 0.9),
                        ("desafiante".to_string(), 0.7),
                    ])),
                },
                QuestionOption {
                    id: "descubrir".to_string(),
                    text: "Descubrir un lugar nuevo".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("exploración".to_string(), 0.9),
                        ("alegre".to_string(), 0.4),
                    ])),
                },
            ],
        },
        
        // Pregunta 5: Meta emocional
        QuestionnaireQuestion {
            id: "meta_emocional".to_string(),
            text: "¿Qué te gustaría sentir después de jugar?".to_string(),
            question_type: QuestionType::GoalState,
            options: vec![
                QuestionOption {
                    id: "satisfaccion".to_string(),
                    text: "Satisfacción por superar un reto".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("desafiante".to_string(), 0.9),
                        ("competitivo".to_string(), 0.6),
                    ])),
                },
                QuestionOption {
                    id: "calma".to_string(),
                    text: "Calma y tranquilidad".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("relajante".to_string(), 0.9),
                        ("contemplativo".to_string(), 0.6),
                    ])),
                },
                QuestionOption {
                    id: "asombro".to_string(),
                    text: "Asombro y curiosidad".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("exploración".to_string(), 0.8),
                        ("contemplativo".to_string(), 0.5),
                    ])),
                },
                QuestionOption {
                    id: "diversion".to_string(),
                    text: "Diversión y alegría".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("alegre".to_string(), 0.9),
                        ("social".to_string(), 0.6),
                    ])),
                },
                QuestionOption {
                    id: "conexion".to_string(),
                    text: "Conexión con una historia o personajes".to_string(),
                    value: QuestionOptionValue::EmotionMapping(HashMap::from([
                        ("melancólico".to_string(), 0.6),
                        ("contemplativo".to_string(), 0.8),
                    ])),
                },
            ],
        },
    ]
}