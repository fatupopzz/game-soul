// src/routes.rs - Corregido

use actix_web::web;
use crate::handlers;

/// Configura todas las rutas de la aplicación
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api")
                // Rutas de cuestionario
                .route("/questionnaire", web::get().to(handlers::questionnaire::get_questionnaire))
                .route("/questionnaire/submit", web::post().to(handlers::questionnaire::submit_questionnaire))
               // En la función configure de src/routes.rs, dentro del scope /api
                .route("/neo4j-questionnaire", web::post().to(handlers::diagnostics::neo4j_questionnaire)) 
                // Rutas de recomendación
                .route("/recommendations", web::post().to(handlers::recommendation::get_recommendations))
                
                // Rutas de diagnóstico
                .route("/diagnose", web::get().to(handlers::diagnostics::diagnose_neo4j))
                .route("/test-recommendation", web::get().to(handlers::diagnostics::test_recommendation_query))
                .route("/diagnose-neo4j", web::get().to(handlers::diagnostics::diagnose_neo4j))
        )
        
        // Ruta de health check
        .route("/health", web::get().to(|| async { "¡Game Soul API está funcionando correctamente!" }));
}
