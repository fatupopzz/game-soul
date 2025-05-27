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
    
    /**
     * Guarda el feedback del usuario respecto a un videojuego recomendado
     * 
     * @param feedback Mapa que conteiene userID, gameID, liked (booelan) y rating (entero)
     * @return Mensaje de éxito o error
     */
    @PostMapping("/feedback")
public ResponseEntity<?> saveFeedback(@RequestBody Map<String, Object> feedback) {
    System.out.println("FEEDBACK RECIBIDO: " + feedback);
    
    try {
        String userId = (String) feedback.get("userId");
        String gameId = (String) feedback.get("gameId");
        Boolean liked = (Boolean) feedback.get("liked");
        
        System.out.println("Procesando: usuario=" + userId + ", juego=" + gameId + ", like=" + liked);
        
        // NUEVO: Asegurar que existen usuarios seed antes de procesar feedback
        userService.ensureSeedUsers();
        
        // Guardar feedback
        userService.saveFeedback(userId, gameId, liked);
        
        // Procesar feedback social (ahora con usuarios seed)
        userService.processSocialFeedback(userId, gameId, liked);
        
        System.out.println("Feedback procesado correctamente");
        
        return ResponseEntity.ok(Map.of(
            "status", "success",
            "message", "Feedback guardado y sistema social actualizado",
            "userId", userId,
            "gameId", gameId,
            "liked", liked
        ));
    } catch (Exception e) {
        System.out.println(" ERROR: " + e.getMessage());
        e.printStackTrace();
        return ResponseEntity.badRequest().body(Map.of(
            "status", "error",  
            "message", "Error: " + e.getMessage()
        ));
    }
}
    
    
    /**
     * Obtiene recoemndaciones mixtas y para un usuario, combinando diversos factores como perfil emocional 
     * retroalimentación previa, y datos sociales
     * 
     * @param userId ID del usuario
     * @return Lista de recomendaciones mixtas 
     */
    @GetMapping("/recommendations/mixed/{userId}")
    public ResponseEntity<List<GameRecommendation>> getMixedRecommendations(@PathVariable String userId) {
        System.out.println("Solicitud de recomendaciones mixtas para: " + userId);
        try {
            List<GameRecommendation> recommendations = 
                recommendationService.getMixedRecommendations(userId);
            System.out.println(" Devolviendo " + recommendations.size() + " recomendaciones mixtas");
            return ResponseEntity.ok(recommendations);
        } catch (Exception e) {
            System.out.println("Error obteniendo recomendaciones mixtas: " + e.getMessage());
            e.printStackTrace();
            return ResponseEntity.notFound().build();
        }
    }

    /**
     * Obtiene recomendaciones sociales para un usuario, basdas en comportamientos  de otros usuarios
     * con perfiles o gustos similares
     * 
     * @param userId ID del usuario
     * @return Lista de recomendaciones basadas en datos sociales
     */
    @GetMapping("/recommendations/social/{userId}")
public ResponseEntity<List<GameRecommendation>> getSocialRecommendations(@PathVariable String userId) {
    System.out.println("Solicitud de recomendaciones sociales para: " + userId);
    try {
        List<GameRecommendation> recommendations = 
            recommendationService.getSocialRecommendations(userId);
        System.out.println("Devolviendo " + recommendations.size() + " recomendaciones sociales");
        return ResponseEntity.ok(recommendations);
    } catch (Exception e) {
        System.out.println("Error obteniendo recomendaciones sociales: " + e.getMessage());
        e.printStackTrace();
        return ResponseEntity.notFound().build();
    }
}
}