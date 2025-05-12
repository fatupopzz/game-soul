//---------------------------------------
// Este archivo contiene la configuración de las rutas de la API
// y la lógica para manejar las solicitudes HTTP.
// Incluye rutas para obtener el cuestionario, enviar respuestas
// y obtener recomendaciones basadas en el perfil emocional del usuario
//---------------------------------------

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
                
                // Rutas de recomendación
                .route("/recommendations", web::post().to(handlers::recommendation::get_recommendations))
                
                // Rutas de diagnóstico y reparación
                .route("/diagnose", web::get().to(handlers::data_diagnosis::diagnose_neo4j_data))
                .route("/repair", web::post().to(handlers::data_diagnosis::repair_neo4j_structure))
        )
        
        // Ruta de health check
        .route("/health", web::get().to(|| async { "¡Game Soul API está funcionando correctamente!" }));
}
