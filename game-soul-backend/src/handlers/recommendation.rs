//necesito quitar los fallbacks por que no se de donde agarra los datos, recommendations o questionnaire

use actix_web::{web, HttpResponse};
use log::{error, info, debug};
use validator::Validate;

use crate::db::neo4j::client::DbPool;
use crate::db::neo4j::queries::recommendations;
use crate::error::{AppError, AppResult};
use crate::models::recommendation::{RecommendationRequest, RecommendationResponse, FeedbackRequest};

/// Controlador para obtener recomendaciones de juegos directamente
pub async fn get_recommendations(
    db: web::Data<DbPool>,
    req: web::Json<RecommendationRequest>,
) -> AppResult<HttpResponse> {
    // Validar la solicitud
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // En una implementación real, obtendríamos el ID del usuario del token JWT
    // Por ahora, usamos un ID genérico para desarrollo
    let user_id = "user_test";
    
    info!("Procesando solicitud de recomendación para usuario: {}", user_id);
    info!("Estado emocional: {}", req.estado_emocional);
    debug!("Tiempo disponible: {} minutos", req.get_tiempo_disponible());
    debug!("Dealbreakers: {:?}", req.get_dealbreakers());
    
    // Obtener recomendaciones basadas en estado emocional
    let recommendations_result = recommendations::get_recommendations(
        &db,
        &req.estado_emocional,
        &req.get_dealbreakers(),
    )
    .await;
    
    // Manejar el resultado de la consulta
    let emotional_recommendations = match recommendations_result {
        Ok(recs) => {
            if recs.is_empty() {
                info!("No se encontraron recomendaciones, intentando consulta alternativa");
                // Intentar una consulta alternativa más simple como respaldo
                match recommendations::get_recommendations_alternative(&db, &req.estado_emocional).await {
                    Ok(alt_recs) => {
                        info!("Consulta alternativa encontró {} recomendaciones", alt_recs.len());
                        alt_recs
                    },
                    Err(e) => {
                        error!("Error en consulta alternativa: {}", e);
                        // Generar recomendaciones manualmente como último recurso
                        generate_fallback_recommendations(&req.estado_emocional)
                    }
                }
            } else {
                info!("Encontradas {} recomendaciones para {}", recs.len(), req.estado_emocional);
                recs
            }
        },
        Err(e) => {
            error!("Error al obtener recomendaciones: {}", e);
            // Generar recomendaciones manualmente como último recurso
            generate_fallback_recommendations(&req.estado_emocional)
        }
    };
    
    // Crear la respuesta con las recomendaciones
    let response = RecommendationResponse::new(
        emotional_recommendations,
        None, // Sin recomendaciones exploratorias por ahora
    );
    
    info!("Enviando {} recomendaciones", response.recomendaciones_emocionales.len());
    
    // Devolver respuesta JSON
    Ok(HttpResponse::Ok().json(response))
}

/// Controlador para proporcionar feedback sobre una recomendación
pub async fn provide_feedback(
    db: web::Data<DbPool>,
    req: web::Json<FeedbackRequest>,
) -> AppResult<HttpResponse> {
    // Validar la solicitud
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    info!("Procesando feedback para usuario: {}, juego: {}, satisfacción: {}", 
          req.user_id, req.game_id, req.satisfaction);
    
    // Guardar el feedback en la base de datos
    // Esta función es la que necesitas implementar en queries/recommendations.rs
    match save_feedback(&db, &req.user_id, &req.game_id, req.satisfaction, req.emotions_experienced.clone()).await {
        Ok(_) => {
            info!("Feedback guardado correctamente");
            // Devolver respuesta de éxito
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Feedback procesado correctamente"
            })))
        },
        Err(e) => {
            error!("Error al guardar feedback: {}", e);
            Err(AppError::DatabaseError(format!("Error al guardar feedback: {}", e)))
        }
    }
}

/// Función para guardar feedback (implementación básica)
async fn save_feedback(
    db: &DbPool,
    user_id: &str, 
    game_id: &str, 
    satisfaction: i32,
    emotions_experienced: Option<Vec<String>>,
) -> anyhow::Result<()> {
    // Crear consulta Cypher para guardar feedback
    let query_text = r#"
    // Crear relación HA_JUGADO si no existe
    MERGE (u:Usuario {id: $user_id})
    MERGE (j:Juego {id: $game_id})
    MERGE (u)-[h:HA_JUGADO]->(j)
    ON CREATE SET h.fecha = datetime(), h.satisfaccion = $satisfaction
    ON MATCH SET h.fecha = datetime(), h.satisfaccion = $satisfaction
    
    // Registrar emociones experimentadas si se proporcionaron
    WITH u, j
    FOREACH (emocion IN CASE WHEN size($emotions) > 0 THEN $emotions ELSE [] END |
        MERGE (e:Emocion {tipo: emocion})
        MERGE (u)-[r:EXPERIMENTO_EMOCION]->(e)
        ON CREATE SET r.fecha = datetime(), r.juego_id = $game_id
        ON MATCH SET r.fecha = datetime(), r.juego_id = $game_id
    )
    
    RETURN 'success' as result
    "#;
    
    // Preparar emociones como array vacío si no se proporcionaron
    let emotions = emotions_experienced.unwrap_or_else(Vec::new);
    
    // Ejecutar la consulta
    db.execute(
        neo4rs::query(query_text)
            .param("user_id", user_id)
            .param("game_id", game_id)
            .param("satisfaction", satisfaction)
            .param("emotions", emotions)
    ).await?;
    
    Ok(())
}

/// Endpoint de diagnóstico para probar conexión y obtener datos de juegos
pub async fn diagnose_games(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    info!("Ejecutando diagnóstico de juegos en Neo4j");
    
    // Consulta simple para obtener algunos juegos
    let query_text = r#"
    MATCH (j:Juego)-[r:RESUENA_CON]->(e:Emocion)
    RETURN j.id, j.nombre, j.descripcion, e.tipo as emocion, r.intensidad
    LIMIT 10
    "#;
    
    match db.execute(neo4rs::query(query_text)).await {
        Ok(mut result) => {
            let mut games = Vec::new();
            
            while let Ok(Some(row)) = result.next().await {
                let id: String = row.get("j.id").unwrap_or_default();
                let nombre: String = row.get("j.nombre").unwrap_or_default();
                let descripcion: String = row.get("j.descripcion").unwrap_or_default();
                let emocion: String = row.get("emocion").unwrap_or_default();
                let intensidad: f64 = row.get("r.intensidad").unwrap_or(0.0);
                
                games.push(serde_json::json!({
                    "id": id,
                    "nombre": nombre,
                    "descripcion": descripcion,
                    "emocion": emocion,
                    "intensidad": intensidad
                }));
            }
            
            if games.is_empty() {
                return Ok(HttpResponse::Ok().json(serde_json::json!({
                    "status": "warning",
                    "message": "No se encontraron juegos en Neo4j"
                })));
            }
            
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "count": games.len(),
                "games": games
            })))
        },
        Err(e) => {
            error!("Error al consultar juegos: {}", e);
            Err(AppError::DatabaseError(format!("Error al consultar juegos: {}", e)))
        }
    }
}


/// Función para generar recomendaciones manuales de respaldo
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