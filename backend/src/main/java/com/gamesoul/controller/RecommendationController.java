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

/**
 * Controlador que gestiona las operaciones relacionadas con recomendaciones de videojuegos
 * basadas en el análisis emocional del usuario
 */

@RestController
@RequestMapping("")
@CrossOrigin(origins = "http://localhost:3000") //Permite peticiones desde el frontend local
public class RecommendationController {
    
    @Autowired
    private RecommendationService recommendationService;
    
    @Autowired
    private UserService userService;
    
    @Autowired
    private EmotionAnalysisService emotionAnalysisService;
    
    /**
     * Procesa las respuestas del cuestionario emocional del usuario 
     * crea un perfil basado en dischas respuestas y genera recomendaciones iniciales. 
     * 
     * @param request Objeto que contiene las respuestas y el ID del usuario
     * @return ResponseEntity con perfil generado y recomendaciones iniciales 
     */
    @PostMapping("/questionnaire")
    public ResponseEntity<?> processQuestionnaire(@RequestBody QuestionnaireRequest request) {
        try {
            // 1. Analizar respuestas y crear perfil emocional
            UserProfile profile = emotionAnalysisService.analyzeQuestionnaire(request.getAnswers());
            profile.setUserId(request.getUserId());
            
            // 2. Guardar el perfil del usuario en la base de datos (Neo4j)
            userService.saveUserProfile(request.getUserId(), profile);
            
            // 3. Obtener recomendaciones de videojuegos basadas en el perfil generado
            List<GameRecommendation> recommendations = 
                recommendationService.getRecommendationsForUser(request.getUserId());
            
            // 4. Responder con el perfil y las recomendaciones
            return ResponseEntity.ok(Map.of(
                "status", "success",
                "message", "Cuestionario procesado correctamente",
                "profile", profile,
                "recommendations", recommendations
            ));
            
        } catch (Exception e) {
            //Manejo de errores
            return ResponseEntity.badRequest().body(Map.of(
                "status", "error",
                "message", "Error procesando cuestionario: " + e.getMessage()
            ));
        }
    }
    
    /**
     * Obtiene recomendaciones de videojuegos para un usuario específico
     * 
     * @param userId del uusuario
     * @return Lista de recomendaciones personalizadas
     */
    @GetMapping("/recommendations/{userId}")
    public ResponseEntity<List<GameRecommendation>> getRecommendations(@PathVariable String userId) {
        try {
            List<GameRecommendation> recommendations = 
                recommendationService.getRecommendationsForUser(userId);
            return ResponseEntity.ok(recommendations);
        } catch (Exception e) {
            // Retorna 404 si no se encuentran recomendaciones 
            return ResponseEntity.notFound().build();
        }
    }
    
    /**
     * Obtiene recomendaciones basadas en una emoción específica. 
     * 
     * @param emotion Nombre de la emoción (ej. "feliz", "triste", etc.)
     * @return Lista de juegos recoemndados para esa emoción
     */
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
