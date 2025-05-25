package com.gamesoul.controller;

import java.util.List;
import java.util.Map;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.gamesoul.model.dto.GameRecommendation;
import com.gamesoul.model.dto.QuestionnaireRequest;
import com.gamesoul.model.dto.UserProfile;
import com.gamesoul.service.EmotionAnalysisService;
import com.gamesoul.service.RecommendationService;
import com.gamesoul.service.UserService;

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
    
    @PostMapping("/feedback")
public ResponseEntity<?> saveFeedback(@RequestBody Map<String, Object> feedback) {
    System.out.println("🔥 FEEDBACK RECIBIDO: " + feedback);
    
    try {
        String userId = (String) feedback.get("userId");
        String gameId = (String) feedback.get("gameId");
        Boolean liked = (Boolean) feedback.get("liked");
        
        System.out.println("📊 Procesando: usuario=" + userId + ", juego=" + gameId + ", like=" + liked);
        
        // NUEVO: Asegurar que existen usuarios seed antes de procesar feedback
        userService.ensureSeedUsers();
        
        // Guardar feedback
        userService.saveFeedback(userId, gameId, liked);
        
        // Procesar feedback social (ahora con usuarios seed)
        userService.processSocialFeedback(userId, gameId, liked);
        
        System.out.println("✅ Feedback procesado correctamente");
        
        return ResponseEntity.ok(Map.of(
            "status", "success",
            "message", "Feedback guardado y sistema social actualizado",
            "userId", userId,
            "gameId", gameId,
            "liked", liked
        ));
    } catch (Exception e) {
        System.out.println("❌ ERROR: " + e.getMessage());
        e.printStackTrace();
        return ResponseEntity.badRequest().body(Map.of(
            "status", "error",  
            "message", "Error: " + e.getMessage()
        ));
    }
}
    
    
    // Nuevo endpoint para recomendaciones mixtas
    @GetMapping("/recommendations/mixed/{userId}")
    public ResponseEntity<List<GameRecommendation>> getMixedRecommendations(@PathVariable String userId) {
        System.out.println("🔀 Solicitud de recomendaciones mixtas para: " + userId);
        try {
            List<GameRecommendation> recommendations = 
                recommendationService.getMixedRecommendations(userId);
            System.out.println("✅ Devolviendo " + recommendations.size() + " recomendaciones mixtas");
            return ResponseEntity.ok(recommendations);
        } catch (Exception e) {
            System.out.println("❌ Error obteniendo recomendaciones mixtas: " + e.getMessage());
            e.printStackTrace();
            return ResponseEntity.notFound().build();
        }
    }
    @GetMapping("/recommendations/social/{userId}")
public ResponseEntity<List<GameRecommendation>> getSocialRecommendations(@PathVariable String userId) {
    System.out.println("👥 Solicitud de recomendaciones sociales para: " + userId);
    try {
        List<GameRecommendation> recommendations = 
            recommendationService.getSocialRecommendations(userId);
        System.out.println("✅ Devolviendo " + recommendations.size() + " recomendaciones sociales");
        return ResponseEntity.ok(recommendations);
    } catch (Exception e) {
        System.out.println("❌ Error obteniendo recomendaciones sociales: " + e.getMessage());
        e.printStackTrace();
        return ResponseEntity.notFound().build();
    }
}
}