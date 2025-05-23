package com.gamesoul.service;

import com.gamesoul.model.dto.UserProfile;
import org.neo4j.driver.Driver;
import org.neo4j.driver.Record;
import org.neo4j.driver.Result;
import org.neo4j.driver.Session;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Map;

@Service
public class UserService {
    
    @Autowired
    private Driver neo4jDriver;
    
    public void saveUserProfile(String userId, UserProfile profile) {
        String saveUserQuery = """
            MERGE (u:Usuario {id: $userId})
            SET u.nombre = $userId,
                u.emocion_dominante = $dominantEmotion,
                u.tiempo_preferido = $timePreference,
                u.fecha_cuestionario = datetime(),
                u.estado = "activo"
            
            WITH u
            MATCH (e:Emocion {tipo: $dominantEmotion})
            MERGE (u)-[r:ESTADO_EMOCIONAL]->(e)
            SET r.intensidad = $intensity, r.fecha = datetime()
            
            RETURN u.id as savedUser
            """;
        
        Double intensity = profile.getEmotionWeights().get(profile.getDominantEmotion());
        if (intensity == null) intensity = 1.0;
        
        try (Session session = neo4jDriver.session()) {
            session.run(saveUserQuery, Map.of(
                "userId", userId,
                "dominantEmotion", profile.getDominantEmotion(),
                "timePreference", profile.getTimePreference(),
                "intensity", intensity
            ));
            
            // Crear resonancias adicionales
            for (Map.Entry<String, Double> entry : profile.getEmotionWeights().entrySet()) {
                if (entry.getValue() > 0.2) {
                    createResonance(session, userId, entry.getKey(), entry.getValue());
                }
            }
        }
    }
    
    private void createResonance(Session session, String userId, String emotion, Double weight) {
        String resonanceQuery = """
            MATCH (u:Usuario {id: $userId})
            MATCH (e:Emocion {tipo: $emotion})
            MERGE (u)-[r:RESUENA_CON]->(e)
            SET r.intensidad = $weight, r.fecha = datetime()
            """;
        
        session.run(resonanceQuery, Map.of(
            "userId", userId,
            "emotion", emotion,
            "weight", weight
        ));
    }
    
    public void saveFeedback(String userId, String gameId, Boolean liked, Integer rating) {
        String feedbackQuery = """
            MERGE (u:Usuario {id: $userId})
            MERGE (j:Juego {id: $gameId})
            MERGE (u)-[f:HA_JUGADO]->(j)
            SET f.liked = $liked,
                f.rating = $rating,
                f.fecha = datetime()
            """;
        
        try (Session session = neo4jDriver.session()) {
            session.run(feedbackQuery, Map.of(
                "userId", userId,
                "gameId", gameId,
                "liked", liked,
                "rating", rating != null ? rating : 3
            ));
        }
    }
}
