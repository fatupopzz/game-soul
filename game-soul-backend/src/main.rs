//--------------------------------------------------
// GAME SOUL - PUNTO DE ENTRADA PRINCIPAL
//
// Este archivo es el punto de entrada de la aplicación.
// Inicializa el servidor, la conexión a la base de datos,
// y configura los componentes esenciales del sistema.
//--------------------------------------------------


use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use log::info;
use actix_web::middleware::Logger;

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
    
    info!("Iniciando conexión a Neo4j...");
    
    // Inicializar conexión a Neo4j
    let db_pool = match db::neo4j::client::create_connection_pool().await {
        Ok(pool) => {
            info!("✅ Conexión exitosa a Neo4j");
            pool
        },
        Err(e) => {
            panic!("❌ Error al conectar con Neo4j: {}", e);
        }
    };
    
    // Inicializar la base de datos (crear nodos esenciales si no existen)
    if let Err(e) = db::neo4j::client::initialize_database(&db_pool).await {
        panic!("❌ Error al inicializar la base de datos: {}", e);
    }
    
    info!("🚀 Iniciando servidor en http://{}", server_address);
    
    // Crear y configurar el servidor HTTP
    HttpServer::new(move || {
        App::new()
            // Middleware para logging de requests
            .wrap(middleware::Logger::default())
            // Middleware para CORS
            .wrap(app_middleware::cors::cors())
            // Middleware para métricas
            .wrap(app_middleware::metrics::metrics_middleware())
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