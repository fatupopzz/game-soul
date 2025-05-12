// src/main.rs - Versión mejorada para verificar la conexión

use actix_web::{App, HttpServer, web, middleware::Logger};
use dotenv::dotenv;
use log::{info, error};

mod config;
mod db;
mod error;
mod handlers;
mod middleware;
use middleware as app_middleware;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar variables de entorno
    dotenv().ok();
    
    // Inicializar logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Cargar configuración
    let config = config::AppConfig::from_env();
    let server_address = format!("{}:{}", config.host, config.port);
    
    info!("🔄 Iniciando conexión a Neo4j...");
    
    // Conectar a Neo4j con mejor manejo de errores
    let db_pool = match db::neo4j::client::create_connection_pool().await {
        Ok(pool) => {
            info!("✅ Conexión exitosa a Neo4j");
            pool
        },
        Err(e) => {
            error!("❌ Error al conectar con Neo4j: {}", e);
            error!("💡 Verifica las credenciales y la URL en las variables de entorno");
            error!("    NEO4J_URI: {}", std::env::var("NEO4J_URI").unwrap_or_else(|_| "No definido".to_string()));
            error!("    NEO4J_USER: {}", std::env::var("NEO4J_USER").unwrap_or_else(|_| "No definido".to_string()));
            error!("    NEO4J_PASSWORD: [oculto]");
            panic!("No se pudo conectar con Neo4j. Verifica las credenciales y la URL.");
        }
    };
    
    // Verificar la estructura de la base de datos
    match db::neo4j::client::verify_database_structure(&db_pool).await {
        Ok(_) => {
            info!("✅ Estructura de base de datos verificada correctamente");
        },
        Err(e) => {
            error!("⚠️ Advertencia: Problema con la estructura de Neo4j: {}", e);
            error!("   El servidor continuará, pero es posible que algunas consultas fallen");
        }
    };
    
    info!("🚀 Iniciando servidor en http://{}", server_address);
    
    // Crear y configurar el servidor HTTP
    HttpServer::new(move || {
        App::new()
            // Middleware para logging de requests
            .wrap(Logger::default())
            // Middleware para CORS
            .wrap(app_middleware::cors::cors())
            // Datos compartidos en la aplicación
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config.clone()))
            // Configurar rutas
            .configure(routes::configure)
    })
    .bind(&server_address)?
    .run()
    .await
}