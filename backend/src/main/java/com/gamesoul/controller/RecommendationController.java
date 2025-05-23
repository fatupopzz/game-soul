package com.gamesoul.controller;

import com.gamesoul.model.dto.*;
import com.gamesoul.service.EmotionAnalysisService;
import com.gamesoul.service.RecommendationService;
import com.gamesoul.service.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.Map;

@RestController
@RequestMapping("")
@CrossOrigin(origins = "http://localhost:3000")
public class RecommendationController {
    
    @Autowired
    private RecommendationService recommendationService;
    
    @Autowired
    private UserService userService;
    
    @Autowired
    private EmotionAnalysisService emotionAnalysisService;
    
    // Procesar cuestionario y crear perfil
    @PostMapping("/questionnaire")
    public ResponseEntity<?> processQuestionnaire(@RequestBody QuestionnaireRequest request) {
        try {
            // 1. Analizar respuestas y crear perfil emocional
            UserProfile profile = emotionAnalysisService.analyzeQuestionnaire(request.getAnswers());
            profile.setUserId(request.getUserId());
            
            // 2. Guardar usuario en Neo4j
            userService.saveUserProfile(request.getUserId(), profile);
            
            // 3. Obtener recomendaciones iniciales
            List<GameRecommendation> recommendations = 
                recommendationService.getRecommendationsForUser(request.getUserId());
            
            return ResponseEntity.ok(Map.of(
                "status", "success",
                "message", "Cuestionario procesado correctamente",
                "profile", profile,
                "recommendations", recommendations
            ));
            
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "status", "error",
                "message", "Error procesando cuestionario: " + e.getMessage()
            ));
        }
    }
    
    // Obtener recomendaciones para un usuario
    @GetMapping("/recommendations/{userId}")
    public ResponseEntity<List<GameRecommendation>> getRecommendations(@PathVariable String userId) {
        try {
            List<GameRecommendation> recommendations = 
                recommendationService.getRecommendationsForUser(userId);
            return ResponseEntity.ok(recommendations);
        } catch (Exception e) {
            return ResponseEntity.notFound().build();
        }
    }
    
    // Obtener recomendaciones por emoción directa
    @GetMapping("/recommendations/emotion/{emotion}")
    public ResponseEntity<List<GameRecommendation>> getRecommendationsByEmotion(@PathVariable String emotion) {
        try {
            List<GameRecommendation> recommendations = 
                recommendationService.getRecommendationsForEmotion(emotion);
            return ResponseEntity.ok(recommendations);
        } catch (Exception e) {
            return ResponseEntity.notFound().build();
        }
    }
    
    // Guardar feedback
    @PostMapping("/feedback")
    public ResponseEntity<?> saveFeedback(@RequestBody Map<String, Object> feedback) {
        try {
            String userId = (String) feedback.get("userId");
            String gameId = (String) feedback.get("gameId");
            Boolean liked = (Boolean) feedback.get("liked");
            Integer rating = (Integer) feedback.get("rating");
            
            userService.saveFeedback(userId, gameId, liked, rating);
            
            return ResponseEntity.ok(Map.of(
                "status", "success",
                "message", "Feedback guardado correctamente"
            ));
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "status", "error",
                "message", "Error guardando feedback: " + e.getMessage()
            ));
        }
    }
}
