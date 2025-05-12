use actix_web::{web, HttpResponse};
use log::{info, error, debug};
use std::collections::HashMap;
use neo4rs::query;
use validator::Validate;
use crate::db::neo4j::client::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::questionnaire::{QuestionnaireSubmission, create_questionnaire, QuestionOptionValue};

/// Endpoint para diagnóstico de conexión a Neo4j
pub async fn diagnose_neo4j(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    info!("Realizando diagnóstico de Neo4j");
    
    // 1. Prueba de conexión básica
    let basic_test = match db.execute(query("RETURN 1 as n")).await {
        Ok(_) => format!("✅ Conexión básica a Neo4j exitosa"),  // Usar format en ambos
        Err(e) => format!("❌ Error de conexión básica: {}", e),
    };

    // 2. Verificar nodos de emoción
    let emotions_test = match db.execute(query("MATCH (e:Emocion) RETURN count(e) as count")).await {
        Ok(mut result) => {
            if let Ok(Some(row)) = result.next().await {
                if let Ok(count) = row.get::<i64>("count") {
                    format!("✅ Encontrados {} nodos de Emocion", count)
                } else {
                    "❌ No se pudo leer el conteo de emociones".to_string()
                }
            } else {
                "❌ No se recibieron resultados al contar emociones".to_string()
            }
        },
        Err(e) => format!("❌ Error al contar emociones: {}", e),
    };
    
    // 3. Verificar nodos de juego
    let games_test = match db.execute(query("MATCH (j:Juego) RETURN count(j) as count")).await {
        Ok(mut result) => {
            if let Ok(Some(row)) = result.next().await {
                if let Ok(count) = row.get::<i64>("count") {
                    format!("✅ Encontrados {} nodos de Juego", count)
                } else {
                    "❌ No se pudo leer el conteo de juegos".to_string()
                }
            } else {
                "❌ No se recibieron resultados al contar juegos".to_string()
            }
        },
        Err(e) => format!("❌ Error al contar juegos: {}", e),
    };
    
    // 4. Verificar relaciones de resonancia
    let resonance_test = match db.execute(query("MATCH (:Juego)-[r:RESUENA_CON]->(:Emocion) RETURN count(r) as count")).await {
        Ok(mut result) => {
            if let Ok(Some(row)) = result.next().await {
                if let Ok(count) = row.get::<i64>("count") {
                    format!("✅ Encontradas {} relaciones de resonancia", count)
                } else {
                    "❌ No se pudo leer el conteo de relaciones".to_string()
                }
            } else {
                "❌ No se recibieron resultados al contar relaciones".to_string()
            }
        },
        Err(e) => format!("❌ Error al contar relaciones: {}", e),
    };
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "Diagnóstico completado",
        "tests": {
            "connection": basic_test,
            "emotions": emotions_test,
            "games": games_test,
            "resonance": resonance_test
        }
    })))
}

/// Endpoint directo para test de recomendaciones
pub async fn test_recommendation_query(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    info!("Ejecutando prueba de consulta de recomendaciones");
    
    // Consulta directa para obtener recomendaciones para la emoción "relajante"
    let test_query = r#"
    MATCH (j:Juego)-[r:RESUENA_CON]->(e:Emocion {tipo: "relajante"})
    RETURN j.id as id, j.nombre as nombre, j.descripcion as descripcion, r.intensidad as resonancia
    ORDER BY r.intensidad DESC
    LIMIT 3
    "#;
    
    let mut result = db.execute(query(test_query)).await?;
    
    let mut recommendations = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        let id: String = row.get("id").unwrap_or_default();
        let nombre: String = row.get("nombre").unwrap_or_default();
        let descripcion: String = row.get("descripcion").unwrap_or_default();
        let resonancia: f64 = row.get("resonancia").unwrap_or(0.0);
        
        recommendations.push(serde_json::json!({
            "id": id,
            "nombre": nombre,
            "descripcion": descripcion,
            "resonancia": resonancia
        }));
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "emotion": "relajante",
        "recommendations": recommendations
    })))
}

/// Endpoint para procesar el cuestionario directamente con Neo4j
pub async fn neo4j_questionnaire(
    db: web::Data<DbPool>,
    req: web::Json<QuestionnaireSubmission>,
) -> AppResult<HttpResponse> {
    info!("【DIRECT】Procesando cuestionario directo con Neo4j");
    
    // Validar la solicitud
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    info!("【DIRECT】Usuario: {}", req.user_id);
    
    // 1. Procesar respuestas para determinar la emoción dominante
    let questions = create_questionnaire();
    let mut emotional_profile: HashMap<String, f64> = HashMap::new();
    
    // Procesar cada respuesta del usuario
    for (question_id, option_id) in &req.answers {
        // Buscar la pregunta correspondiente
        if let Some(question) = questions.iter().find(|q| &q.id == question_id) {
            // Buscar la opción seleccionada
            if let Some(option) = question.options.iter().find(|o| &o.id == option_id) {
                debug!("【DIRECT】Procesando respuesta '{}' para pregunta '{}'", option.text, question.text);
                
                match &option.value {
                    // Si es un mapeo emocional, agregar al perfil emocional
                    QuestionOptionValue::EmotionMapping(emotions) => {
                        for (emotion, intensity) in emotions {
                            let current = emotional_profile.entry(emotion.clone()).or_insert(0.0);
                            *current += intensity;
                            debug!("【DIRECT】Añadiendo emoción '{}' con intensidad {:.2}", emotion, intensity);
                        }
                    },
                    // Ignoramos otros tipos de valores para simplificar
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
            debug!("【DIRECT】Emoción normalizada: {} = {:.2}", emotion, value);
        }
    }
    
    // Encontrar la emoción dominante
    let dominant_emotion = emotional_profile
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(k, v)| {
            info!("【DIRECT】Emoción dominante: {} ({:.2})", k, v);
            k.clone()
        })
        .unwrap_or_else(|| {
            info!("【DIRECT】No se encontró emoción dominante, usando 'relajante'");
            "relajante".to_string()
        });
    
    // 2. Obtener dealbreakers si existen
    let dealbreakers = req.dealbreakers.clone().unwrap_or_else(Vec::new);
    if !dealbreakers.is_empty() {
        info!("【DIRECT】Características a evitar: {:?}", dealbreakers);
    }
    
    // 3. Consultar juegos directamente en Neo4j
    info!("【DIRECT】Consultando Neo4j para emoción: {}", dominant_emotion);
    
    // Consulta optimizada que obtiene toda la información en un solo paso
    let query_text = r#"
    // Buscar juegos que resuenan con la emoción dominante
    MATCH (j:Juego)-[r:RESUENA_CON]->(e:Emocion {tipo: $emotion_type})
    
    // Obtener características para filtrado
    OPTIONAL MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
    WITH j, r, e, collect(DISTINCT c.nombre) as caracteristicas
    
    // Filtrar por dealbreakers si hay alguno
    WHERE NONE(dealbreaker IN $dealbreakers WHERE dealbreaker IN caracteristicas)
    
    // Obtener géneros
    OPTIONAL MATCH (j)-[:TIENE_GENERO]->(g:Genero)
    WITH j, r, e, caracteristicas, collect(DISTINCT g.nombre) as generos
    
    // Ordenar por resonancia y limitar resultados
    ORDER BY r.intensidad DESC
    LIMIT 5
    
    // Devolver toda la información
    RETURN 
        j.id as id,
        j.nombre as nombre,
        j.descripcion as descripcion,
        r.intensidad as resonancia,
        caracteristicas,
        generos,
        [e.tipo] as emociones_coincidentes
    "#;
    
    info!("【DIRECT】Ejecutando consulta Neo4j");
    
    // Ejecutar la consulta
    let mut result = match db.execute(
        query(query_text)
            .param("emotion_type", dominant_emotion.as_str())
            .param("dealbreakers", dealbreakers)
    ).await {
        Ok(res) => {
            info!("【DIRECT】Consulta ejecutada exitosamente");
            res
        },
        Err(e) => {
            error!("【DIRECT】Error al ejecutar consulta: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Error al consultar la base de datos",
                "error": format!("{}", e)
            })));
        }
    };
    
    // 4. Procesar resultados
    let mut recommendations = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        debug!("【DIRECT】Procesando fila de resultados");
        
        // Extraer datos con manejo de errores
        let id = match row.get::<String>("id") {
            Ok(val) => {
                info!("【DIRECT】ID extraído: {}", val);
                val
            },
            Err(e) => {
                error!("【DIRECT】Error al extraer ID: {}", e);
                continue;
            }
        };
        
        let nombre = match row.get::<String>("nombre") {
            Ok(val) => {
                info!("【DIRECT】Nombre extraído: {}", val);
                val
            },
            Err(e) => {
                error!("【DIRECT】Error al extraer nombre para juego {}: {}", id, e);
                format!("Juego {}", id)
            }
        };
        
        let descripcion = match row.get::<String>("descripcion") {
            Ok(val) => val,
            Err(e) => {
                error!("【DIRECT】Error al extraer descripción para juego {}: {}", id, e);
                "Sin descripción disponible".to_string()
            }
        };
        
        let resonancia = match row.get::<f64>("resonancia") {
            Ok(val) => val,
            Err(e) => {
                error!("【DIRECT】Error al extraer resonancia para juego {}: {}", id, e);
                0.5 // valor por defecto
            }
        };
        
        let caracteristicas = match row.get::<Vec<String>>("caracteristicas") {
            Ok(val) => val,
            Err(e) => {
                error!("【DIRECT】Error al extraer características para juego {}: {}", id, e);
                Vec::new()
            }
        };
        
        let generos = match row.get::<Vec<String>>("generos") {
            Ok(val) => val,
            Err(e) => {
                error!("【DIRECT】Error al extraer géneros para juego {}: {}", id, e);
                Vec::new()
            }
        };
        
        let emociones_coincidentes = match row.get::<Vec<String>>("emociones_coincidentes") {
            Ok(val) => val,
            Err(e) => {
                error!("【DIRECT】Error al extraer emociones coincidentes para juego {}: {}", id, e);
                vec![dominant_emotion.clone()]
            }
        };
        
        // Añadir recomendación
        recommendations.push(serde_json::json!({
            "id": id,
            "nombre": nombre,
            "descripcion": descripcion,
            "resonancia": resonancia,
            "caracteristicas": caracteristicas,
            "generos": generos,
            "emociones_coincidentes": emociones_coincidentes
        }));
    }
    
    // 5. Devolver respuesta
    info!("【DIRECT】Devolviendo {} recomendaciones", recommendations.len());
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "emocion_dominante": dominant_emotion,
        "recomendaciones": recommendations
    })))
}