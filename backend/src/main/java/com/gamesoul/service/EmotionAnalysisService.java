package com.gamesoul.service;

import com.gamesoul.model.dto.UserProfile;
import org.springframework.stereotype.Service;

import java.util.HashMap;
import java.util.Map;

/**
 * Servicio encargado de analizar las respuestas de un cuestionario emocional 
 * para generar un perfil de usuario que permita personalizar recomendaciones de juegos.
 */
@Service
public class EmotionAnalysisService {
    
    /**
     * Analiza las respuestas del cuestionario y construye un perfil emocional del usuario. 
     * 
     * @param answers Mapa con las respuestas del cuestionario, donde la clave es la pregunta y el valor para la respuesta 
     * @return Un objeto UserProfile que representa las emociones dominantes del usuario.
     */
    public UserProfile analyzeQuestionnaire(Map<String, String> answers) {
        Map<String, Double> emotionWeights = new HashMap<>();
        
        // Procesar cada respuesta del cuestionario
        for (Map.Entry<String, String> entry : answers.entrySet()) {
            String question = entry.getKey();
            String answer = entry.getValue();
            
            processAnswer(question, answer, emotionWeights);
        }
        
        // Normalizar pesos emocionales
        normalizeEmotions(emotionWeights);
        
        // Encontrar emoción dominante
        String dominantEmotion = findDominantEmotion(emotionWeights);
        
        // Determinar preferencia de tiempo
        String timePreference = answers.getOrDefault("tiempo_disponible", "medio");
        
        // Crea y retorna un perfil de usuario temporal (el ID puede ser reemplazado más adelante)
        return new UserProfile("temp", dominantEmotion, timePreference, emotionWeights);
    }
    

    /**
     * Procesa una respuesta y actualiza los pesos emocionales correspondientes 
     * 
     * @param question          Preguntas del cuestionario
     * @param answer            Respuesta seleccionada
     * @param emotionWeights    Mapa donde se acumulan los pesos emocionales
     */
    private void processAnswer(String question, String answer, Map<String, Double> emotionWeights) {
        switch (question) {
            case "tipo_experiencia":
                switch (answer) {
                    case "relajante" -> addEmotion(emotionWeights, "relajante", 0.9);
                    case "emocion" -> addEmotion(emotionWeights, "desafiante", 0.7);
                    case "desafio" -> addEmotion(emotionWeights, "desafiante", 0.9);
                    case "exploracion" -> addEmotion(emotionWeights, "exploración", 0.9);
                    case "conexion" -> addEmotion(emotionWeights, "social", 0.9);
                }
                break;
                
            case "estado_animo":
                switch (answer) {
                    case "tranquilo" -> addEmotion(emotionWeights, "relajante", 0.8);
                    case "energico" -> addEmotion(emotionWeights, "competitivo", 0.7);
                    case "curioso" -> addEmotion(emotionWeights, "exploración", 0.8);
                    case "nostalgico" -> addEmotion(emotionWeights, "melancólico", 0.8);
                    case "estresado" -> addEmotion(emotionWeights, "relajante", 0.9);
                }
                break;
                
            case "actividad_preferida":
                switch (answer) {
                    case "construir" -> addEmotion(emotionWeights, "creativo", 0.9);
                    case "competir" -> addEmotion(emotionWeights, "competitivo", 0.9);
                    case "descubrir" -> addEmotion(emotionWeights, "exploración", 0.9);
                    case "historia" -> addEmotion(emotionWeights, "contemplativo", 0.7);
                }
                break;
                
            case "meta_emocional":
                switch (answer) {
                    case "calma" -> addEmotion(emotionWeights, "relajante", 0.9);
                    case "satisfaccion" -> addEmotion(emotionWeights, "desafiante", 0.9);
                    case "asombro" -> addEmotion(emotionWeights, "exploración", 0.8);
                    case "diversion" -> addEmotion(emotionWeights, "alegre", 0.9);
                    case "conexion" -> addEmotion(emotionWeights, "contemplativo", 0.8);
                }
                break;
        }
    }
    
    /**
     * Suma un valor al peso de una emoción específica. 
     * Si la emocioón no existe en el mapa, se inicializa con el valor proporcionado
     * 
     * @param emotionWeights    Mapa de pesos emocionales
     * @param emotion           Emoción a actualizar 
     * @param weight            Peso a agregar 
     */
    private void addEmotion(Map<String, Double> emotionWeights, String emotion, double weight) {
        emotionWeights.merge(emotion, weight, Double::sum);
    }
    
    /**
     * Normaliza todos los valores del mapa de emociones para que sumen 1
     * 
     * @param emotionWeights Mapa de pesos emocionales a normalizar
     */
    private void normalizeEmotions(Map<String, Double> emotionWeights) {
        double sum = emotionWeights.values().stream().mapToDouble(Double::doubleValue).sum();
        if (sum > 0) {
            emotionWeights.replaceAll((k, v) -> v / sum);
        }
    }
    
    /**
     * Encuentra la emoción con el valor más alto en el mapa de emociones. 
     * 
     * @param emotionWeights Mapa con las emociones y sus pesos normalizados
     * @return Nombre de la emoción dominante. Por defecto retorna "relajante" si no hay datos. 
     */
    private String findDominantEmotion(Map<String, Double> emotionWeights) {
        return emotionWeights.entrySet().stream()
                .max(Map.Entry.comparingByValue()) //Encunetra la emoción con mayor peso 
                .map(Map.Entry::getKey) //Retorna el nombre de la emoción
                .orElse("relajante"); //Valor por defecto 
    }
}
