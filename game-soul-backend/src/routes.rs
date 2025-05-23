//se agregaron las nuevas rutas de usuario al archivo de rutas

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
                .route("/neo4j-questionnaire", web::post().to(handlers::diagnostics::neo4j_questionnaire)) 
                
                // Rutas de recomendación
                .route("/recommendations", web::post().to(handlers::recommendation::get_recommendations))
                
                // ✨ NUEVAS RUTAS DE USUARIO (agregar estas dos líneas)
                .route("/user/{user_id}", web::get().to(handlers::user::get_user_info))
                .route("/user/{user_id}/profile", web::get().to(handlers::user::get_user_profile))
                
                // Rutas de diagnóstico
                .route("/diagnose", web::get().to(handlers::diagnostics::diagnose_neo4j))
                .route("/test-recommendation", web::get().to(handlers::diagnostics::test_recommendation_query))
                .route("/diagnose-neo4j", web::get().to(handlers::diagnostics::diagnose_neo4j))
        )
        
        // Ruta de health check
        .route("/health", web::get().to(|| async { "¡Game Soul API está funcionando correctamente!" }));
}