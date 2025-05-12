//--------------------------------------------------
// GAME SOUL - CONFIGURACIÓN
//
// Este archivo maneja la configuración de la aplicación,
// cargando valores desde variables de entorno o asignando
// valores predeterminados cuando sea necesario.
//--------------------------------------------------

use std::env;
use log::warn;

#[derive(Clone, Debug)]
pub struct AppConfig {
    // Configuración del servidor
    pub host: String,
    pub port: u16,
    
    // Configuración de Neo4j
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
    
    // Configuración de JWT (para futuras versiones)
    pub jwt_secret: String,
    pub jwt_expiration: i64, // en segundos
    
    // Otras configuraciones
    pub environment: Environment,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl Environment {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "production" | "prod" => Environment::Production,
            "testing" | "test" => Environment::Testing,
            _ => Environment::Development,
        }
    }
    
    pub fn is_development(&self) -> bool {
        *self == Environment::Development
    }
    
    pub fn is_production(&self) -> bool {
        *self == Environment::Production
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        // Configuración del entorno
        let environment = Environment::from_str(
            &env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
        );
        
        // Configuración del servidor
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse::<u16>()
            .unwrap_or_else(|_| {
                warn!("Puerto inválido en la configuración, usando 3001 por defecto");
                3001
            });
        
        // Configuración de Neo4j
        let neo4j_uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
        let neo4j_user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
        let neo4j_password = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| {
            warn!("NEO4J_PASSWORD no está definida, usando 'password' por defecto, ¡esto es inseguro!");
            "password".to_string()
        });
        
        // Configuración de JWT
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            if environment.is_production() {
                panic!("JWT_SECRET debe estar definido en producción");
            }
            warn!("JWT_SECRET no está definido, usando valor por defecto, ¡esto es inseguro!");
            "development_jwt_secret_key".to_string()
        });
        
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string()) // 24 horas por defecto
            .parse::<i64>()
            .unwrap_or(86400);
        
        AppConfig {
            host,
            port,
            neo4j_uri,
            neo4j_user,
            neo4j_password,
            jwt_secret,
            jwt_expiration,
            environment,
        }
    }
}