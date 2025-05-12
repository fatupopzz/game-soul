use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Genera un ID único para usar en la aplicación
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// Obtiene la fecha y hora actual en formato UTC
pub fn get_current_datetime() -> DateTime<Utc> {
    Utc::now()
}

/// Convierte minutos a un formato más legible
pub fn format_minutes_to_human_readable(minutes: i32) -> String {
    if minutes < 60 {
        format!("{} minutos", minutes)
    } else if minutes < 120 {
        format!("1 hora y {} minutos", minutes - 60)
    } else {
        let hours = minutes / 60;
        let remaining_minutes = minutes % 60;
        
        if remaining_minutes == 0 {
            format!("{} horas", hours)
        } else {
            format!("{} horas y {} minutos", hours, remaining_minutes)
        }
    }
}

/// Normaliza un nombre para uso en IDs (sin espacios, minúsculas)
pub fn normalize_name_for_id(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
        .replace(" ", "_")
}

/// Trunca un texto a una longitud máxima y añade "..." si es necesario
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let mut truncated = text.chars().take(max_length - 3).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}