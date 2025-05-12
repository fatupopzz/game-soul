//--------------------------------------------------
// GAME SOUL - MODELO DE EMOCIONES Y CARACTERÍSTICAS
//
// Este archivo define las emociones y características
// disponibles en el sistema, asegurando que coincidan
// exactamente con las definidas en la base de datos Neo4j.
//--------------------------------------------------

use serde::{Deserialize, Serialize};

/// Emociones disponibles en el sistema
/// Estas coinciden exactamente con los nodos en Neo4j
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emotion {
    pub tipo: String,
    pub descripcion: Option<String>,
}

/// Lista de todas las emociones disponibles
pub fn get_available_emotions() -> Vec<Emotion> {
    vec![
        Emotion { 
            tipo: "alegre".to_string(), 
            descripcion: Some("Experiencias divertidas y positivas".to_string())
        },
        Emotion { 
            tipo: "relajante".to_string(), 
            descripcion: Some("Experiencias calmadas y sin estrés".to_string())
        },
        Emotion { 
            tipo: "melancólico".to_string(), 
            descripcion: Some("Experiencias emotivas y nostálgicas".to_string())
        },
        Emotion { 
            tipo: "exploración".to_string(), 
            descripcion: Some("Experiencias de descubrimiento y curiosidad".to_string())
        },
        Emotion { 
            tipo: "desafiante".to_string(), 
            descripcion: Some("Experiencias que prueban tus habilidades".to_string())
        },
        Emotion { 
            tipo: "contemplativo".to_string(), 
            descripcion: Some("Experiencias reflexivas y pensativas".to_string())
        },
        Emotion { 
            tipo: "social".to_string(), 
            descripcion: Some("Experiencias de conexión con otros".to_string())
        },
        Emotion { 
            tipo: "competitivo".to_string(), 
            descripcion: Some("Experiencias de competición y superación".to_string())
        },
        Emotion { 
            tipo: "creativo".to_string(), 
            descripcion: Some("Experiencias de expresión y creación".to_string())
        },
    ]
}

/// Características de juegos disponibles
/// Estas coinciden exactamente con los nodos en Neo4j
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Characteristic {
    pub nombre: String,
    pub descripcion: Option<String>,
}

/// Lista de todas las características disponibles
pub fn get_available_characteristics() -> Vec<Characteristic> {
    vec![
        Characteristic {
            nombre: "social".to_string(),
            descripcion: Some("Juegos con enfoque en interacciones sociales".to_string())
        },
        Characteristic {
            nombre: "exploración".to_string(),
            descripcion: Some("Juegos que incentivan descubrir el mundo".to_string())
        },
        Characteristic {
            nombre: "desafiante".to_string(),
            descripcion: Some("Juegos que ponen a prueba tus habilidades".to_string())
        },
        Characteristic {
            nombre: "historia".to_string(),
            descripcion: Some("Juegos con narrativas desarrolladas".to_string())
        },
        Characteristic {
            nombre: "puzzles".to_string(),
            descripcion: Some("Juegos con rompecabezas y acertijos".to_string())
        },
        Characteristic {
            nombre: "coleccionable".to_string(),
            descripcion: Some("Juegos que incluyen elementos coleccionables".to_string())
        },
        Characteristic {
            nombre: "difícil".to_string(),
            descripcion: Some("Juegos con alto nivel de dificultad".to_string())
        },
        Characteristic {
            nombre: "combate".to_string(),
            descripcion: Some("Juegos con sistemas de combate".to_string())
        },
        Characteristic {
            nombre: "atmósfera".to_string(),
            descripcion: Some("Juegos con ambientes inmersivos".to_string())
        },
        Characteristic {
            nombre: "inmersivo".to_string(),
            descripcion: Some("Juegos que te sumergen completamente en su mundo".to_string())
        },
        Characteristic {
            nombre: "decisiones".to_string(),
            descripcion: Some("Juegos donde tus decisiones importan".to_string())
        },
        Characteristic {
            nombre: "artístico".to_string(),
            descripcion: Some("Juegos con estética y diseño artístico destacable".to_string())
        },
        Characteristic {
            nombre: "trabajo en equipo".to_string(),
            descripcion: Some("Juegos que requieren coordinación en grupo".to_string())
        },
        Characteristic {
            nombre: "habilidades".to_string(),
            descripcion: Some("Juegos que requieren desarrollo de destrezas específicas".to_string())
        },
        Characteristic {
            nombre: "estrategia".to_string(),
            descripcion: Some("Juegos que requieren planificación".to_string())
        },
        Characteristic {
            nombre: "rápido".to_string(),
            descripcion: Some("Juegos con ritmo acelerado".to_string())
        },
        Characteristic {
            nombre: "personajes".to_string(),
            descripcion: Some("Juegos con desarrollo de personajes destacable".to_string())
        },
        Characteristic {
            nombre: "estilizado".to_string(),
            descripcion: Some("Juegos con estilo visual distintivo".to_string())
        },
    ]
}

/// Características que pueden ser dealbreakers
/// Subconjunto de características que los usuarios podrían querer evitar
pub fn get_dealbreaker_characteristics() -> Vec<String> {
    vec![
        "combate".to_string(),
        "difícil".to_string(),
        "social".to_string(),
        "rápido".to_string(),
    ]
}