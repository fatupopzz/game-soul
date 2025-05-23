package com.gamesoul.service;

import com.gamesoul.model.dto.UserProfile;
import org.springframework.stereotype.Service;

import java.util.HashMap;
import java.util.Map;

@Service
public class EmotionAnalysisService {
    
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
        
        return new UserProfile("temp", dominantEmotion, timePreference, emotionWeights);
    }
    
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
    
    private void addEmotion(Map<String, Double> emotionWeights, String emotion, double weight) {
        emotionWeights.merge(emotion, weight, Double::sum);
    }
    
    private void normalizeEmotions(Map<String, Double> emotionWeights) {
        double sum = emotionWeights.values().stream().mapToDouble(Double::doubleValue).sum();
        if (sum > 0) {
            emotionWeights.replaceAll((k, v) -> v / sum);
        }
    }
    
    private String findDominantEmotion(Map<String, Double> emotionWeights) {
        return emotionWeights.entrySet().stream()
                .max(Map.Entry.comparingByValue())
                .map(Map.Entry::getKey)
                .orElse("relajante");
    }
}
