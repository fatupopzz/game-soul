use actix_cors::Cors;
use actix_web::http::header;
use std::env;

/// Configura el middleware CORS para permitir peticiones desde el frontend
pub fn cors() -> Cors {
    // Obtener la URL del frontend desde las variables de entorno
    let frontend_url = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    // Imprimir la configuración para debugging
    log::info!("Configurando CORS para permitir peticiones desde: {}", frontend_url);
    
    // Crear una configuración CORS que permita el origen especificado
    Cors::default()
        // Permitir el origen del frontend
        .allowed_origin(&frontend_url)
        // Métodos HTTP permitidos
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        // Cabeceras permitidas
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ])
        // Permitir enviar cookies desde el navegador
        .supports_credentials()
        // Tiempo máximo de caché de la configuración CORS en el navegador
        .max_age(3600)
}