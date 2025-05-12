use actix_web::{web, HttpResponse};
use log::{info, error};
use neo4rs::query;

use crate::db::neo4j::client::DbPool;
use crate::error::AppResult;

// Endpoint para diagnosticar datos existentes
pub async fn diagnose_neo4j_data(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    info!("Diagnosticando datos existentes en Neo4j");
    
    // Obtener estadísticas de nodos
    let node_stats = get_node_stats(&db).await;
    
    // Obtener estadísticas de relaciones
    let relation_stats = get_relation_stats(&db).await;
    
    // Obtener detalles de juegos con emoción "relajante"
    let relaxing_games = get_relaxing_games(&db).await;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Diagnóstico completado",
        "node_stats": node_stats,
        "relation_stats": relation_stats,
        "relaxing_games": relaxing_games
    })))
}

// Obtener estadísticas de nodos existentes
async fn get_node_stats(db: &DbPool) -> serde_json::Value {
    let query_text = r#"
    MATCH (n)
    WITH labels(n) AS nodeType, count(n) AS count
    RETURN nodeType, count
    ORDER BY count DESC
    "#;
    
    let mut result = match db.execute(query(query_text)).await {
        Ok(result) => result,
        Err(e) => {
            error!("Error al obtener estadísticas de nodos: {}", e);
            return serde_json::json!({ "error": format!("{}", e) });
        }
    };
    
    let mut stats = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        let node_type: Vec<String> = match row.get("nodeType") {
            Ok(val) => val,
            Err(_) => continue,
        };
        
        let count: i64 = match row.get("count") {
            Ok(val) => val,
            Err(_) => 0,
        };
        
        let node_type_str = node_type.join(":");
        stats.push(serde_json::json!({
            "type": node_type_str,
            "count": count
        }));
    }
    
    serde_json::json!(stats)
}

// Obtener estadísticas de relaciones existentes
async fn get_relation_stats(db: &DbPool) -> serde_json::Value {
    let query_text = r#"
    MATCH ()-[r]->()
    RETURN type(r) AS relationType, count(r) AS count
    ORDER BY count DESC
    "#;
    
    let mut result = match db.execute(query(query_text)).await {
        Ok(result) => result,
        Err(e) => {
            error!("Error al obtener estadísticas de relaciones: {}", e);
            return serde_json::json!({ "error": format!("{}", e) });
        }
    };
    
    let mut stats = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        let relation_type: String = match row.get("relationType") {
            Ok(val) => val,
            Err(_) => continue,
        };
        
        let count: i64 = match row.get("count") {
            Ok(val) => val,
            Err(_) => 0,
        };
        
        stats.push(serde_json::json!({
            "type": relation_type,
            "count": count
        }));
    }
    
    serde_json::json!(stats)
}

// Obtener detalles de juegos con emoción "relajante"
async fn get_relaxing_games(db: &DbPool) -> serde_json::Value {
    let query_text = r#"
    MATCH (e:Emocion {tipo: "relajante"})
    MATCH (j:Juego)-[r:RESUENA_CON]->(e)
    OPTIONAL MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
    OPTIONAL MATCH (j)-[:TIENE_DURACION]->(d:RangoDuracion)
    WITH j, r, collect(DISTINCT c.nombre) as caracteristicas, collect(DISTINCT d.nombre) as duraciones
    RETURN j.id AS id, 
           j.nombre AS nombre, 
           r.intensidad AS resonancia,
           caracteristicas,
           duraciones[0] AS duracion
    ORDER BY r.intensidad DESC
    LIMIT 5
    "#;
    
    let mut result = match db.execute(query(query_text)).await {
        Ok(result) => result,
        Err(e) => {
            error!("Error al obtener juegos relajantes: {}", e);
            return serde_json::json!({ "error": format!("{}", e) });
        }
    };
    
    let mut games = Vec::new();
    
    while let Ok(Some(row)) = result.next().await {
        let id: String = match row.get("id") {
            Ok(val) => val,
            Err(_) => continue,
        };
        
        let nombre: String = match row.get("nombre") {
            Ok(val) => val,
            Err(_) => "Nombre desconocido".to_string(),
        };
        
        let resonancia: f64 = match row.get("resonancia") {
            Ok(val) => val,
            Err(_) => 0.0,
        };
        
        let caracteristicas: Vec<String> = match row.get("caracteristicas") {
            Ok(val) => val,
            Err(_) => Vec::new(),
        };
        
        let duracion: Option<String> = match row.get("duracion") {
            Ok(val) => val,
            Err(_) => None,
        };
        
        games.push(serde_json::json!({
            "id": id,
            "nombre": nombre,
            "resonancia": resonancia,
            "caracteristicas": caracteristicas,
            "duracion": duracion
        }));
    }
    
    serde_json::json!(games)
}

// Verificar y reparar la estructura de datos si es necesario
pub async fn repair_neo4j_structure(db: web::Data<DbPool>) -> AppResult<HttpResponse> {
    info!("Verificando y reparando estructura de Neo4j");
    
    // 1. Asegurar que todos los juegos tengan duración
    let fix_duration_query = r#"
    MATCH (j:Juego)
    WHERE NOT (j)-[:TIENE_DURACION]->()
    WITH j
    MATCH (r:RangoDuracion {nombre: "medio"})
    MERGE (j)-[:TIENE_DURACION]->(r)
    RETURN count(j) as fixed_count
    "#;
    
    let duration_fixes = match db.execute(query(fix_duration_query)).await {
        Ok(mut result) => {
            if let Ok(Some(row)) = result.next().await {
                match row.get::<i64>("fixed_count") {
                    Ok(count) => count,
                    Err(_) => 0,
                }
            } else {
                0
            }
        },
        Err(e) => {
            error!("Error al reparar duraciones: {}", e);
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"status": "error", "message": format!("Error al reparar duraciones: {}", e)})
            ));
        }
    };
    
    // 2. Asegurar que todos los juegos tengan al menos una emoción
    let fix_emotion_query = r#"
    MATCH (j:Juego)
    WHERE NOT (j)-[:RESUENA_CON]->(:Emocion)
    WITH j
    MATCH (e:Emocion) 
    WHERE e.tipo IN ["relajante", "creativo", "exploración"]
    WITH j, e, rand() as random
    ORDER BY random
    LIMIT 1
    MERGE (j)-[r:RESUENA_CON]->(e)
    ON CREATE SET r.intensidad = 0.7 + rand() * 0.3
    RETURN count(j) as fixed_count
    "#;
    
    let emotion_fixes = match db.execute(query(fix_emotion_query)).await {
        Ok(mut result) => {
            if let Ok(Some(row)) = result.next().await {
                match row.get::<i64>("fixed_count") {
                    Ok(count) => count,
                    Err(_) => 0,
                }
            } else {
                0
            }
        },
        Err(e) => {
            error!("Error al reparar emociones: {}", e);
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"status": "error", "message": format!("Error al reparar emociones: {}", e)})
            ));
        }
    };
    
    // 3. Asegurar que la emoción "relajante" tenga al menos 3 juegos
    let fix_relaxing_query = r#"
    MATCH (e:Emocion {tipo: "relajante"})
    WITH e
    MATCH (j:Juego)
    WHERE NOT (j)-[:RESUENA_CON]->(e)
    WITH e, j, rand() as random
    ORDER BY random
    LIMIT 3
    MERGE (j)-[r:RESUENA_CON]->(e)
    ON CREATE SET r.intensidad = 0.7 + rand() * 0.3
    RETURN count(j) as fixed_count
    "#;
    
    let relaxing_fixes = match db.execute(query(fix_relaxing_query)).await {
        Ok(mut result) => {
            if let Ok(Some(row)) = result.next().await {
                match row.get::<i64>("fixed_count") {
                    Ok(count) => count,
                    Err(_) => 0,
                }
            } else {
                0
            }
        },
        Err(e) => {
            error!("Error al reparar juegos relajantes: {}", e);
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"status": "error", "message": format!("Error al reparar juegos relajantes: {}", e)})
            ));
        }
    };
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Estructura reparada",
        "repairs": {
            "duration_fixes": duration_fixes,
            "emotion_fixes": emotion_fixes,
            "relaxing_fixes": relaxing_fixes
        }
    })))
}
