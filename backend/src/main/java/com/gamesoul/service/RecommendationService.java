package com.gamesoul.service;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import org.neo4j.driver.Driver;
import org.neo4j.driver.Record;
import org.neo4j.driver.Result;
import org.neo4j.driver.Session;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.gamesoul.model.dto.GameRecommendation;

@Service
public class RecommendationService {
    
    @Autowired
    private Driver neo4jDriver;
    
    public List<GameRecommendation> getRecommendationsForUser(String userId) {
        List<GameRecommendation> recommendations = new ArrayList<>();
        
        String query = """
            MATCH (u:Usuario {id: $userId})-[:ESTADO_EMOCIONAL]->(e:Emocion)
            MATCH (j:Juego)-[r:RESUENA_CON]->(e)
            RETURN j.id as id, j.nombre as name, j.descripcion as description, 
                   r.intensidad as matchScore,
                   e.tipo as emotion
            ORDER BY r.intensidad DESC
            LIMIT 5
            """;
        
        try (Session session = neo4jDriver.session()) {
            Result result = session.run(query, Map.of("userId", userId));
            
            while (result.hasNext()) {
                Record record = result.next();
                GameRecommendation rec = new GameRecommendation(
                    record.get("id").asString(),
                    record.get("name").asString(),
                    record.get("description").asString(),
                    record.get("matchScore").asDouble()
                );
                
                String emotion = record.get("emotion").asString();
                rec.setReasons(List.of("Resuena con tu emoción: " + emotion));
                
                recommendations.add(rec);
            }
        }
        
        return recommendations;
    }
    
    public List<GameRecommendation> getRecommendationsForEmotion(String emotion) {
        List<GameRecommendation> recommendations = new ArrayList<>();
        
        String query = """
            MATCH (j:Juego)-[r:RESUENA_CON]->(e:Emocion {tipo: $emotion})
            RETURN j.id as id, j.nombre as name, j.descripcion as description, 
                   r.intensidad as matchScore
            ORDER BY r.intensidad DESC
            LIMIT 5
            """;
        
        try (Session session = neo4jDriver.session()) {
            Result result = session.run(query, Map.of("emotion", emotion));
            
            while (result.hasNext()) {
                Record record = result.next();
                GameRecommendation rec = new GameRecommendation(
                    record.get("id").asString(),
                    record.get("name").asString(),
                    record.get("description").asString(),
                    record.get("matchScore").asDouble()
                );
                
                rec.setReasons(List.of("Perfecto para cuando te sientes: " + emotion));
                recommendations.add(rec);
            }
        }
        
        return recommendations;
    }

    public List<GameRecommendation> getSocialRecommendations(String userId) {
    String query = """
        MATCH (u:Usuario {id: $userId})-[:SIMILAR_A]->(similar:Usuario)
        MATCH (similar)-[r:HA_JUGADO]->(recomendado:Juego)
        WHERE r.liked = true
          AND NOT EXISTS((u)-[:HA_JUGADO]->(recomendado))
        WITH recomendado, 
             count(similar) as popularidad,
             collect(similar.nombre)[0..3] as recomendado_por
        ORDER BY popularidad DESC
        LIMIT 3
        RETURN recomendado.id as id,
               recomendado.nombre as name,
               recomendado.descripcion as description,
               popularidad * 0.2 as matchScore,
               recomendado_por
        """;
    
    List<GameRecommendation> recommendations = new ArrayList<>();
    
    try (Session session = neo4jDriver.session()) {
        Result result = session.run(query, Map.of("userId", userId));
        
        while (result.hasNext()) {
            Record record = result.next();
            GameRecommendation rec = new GameRecommendation(
                record.get("id").asString(),
                record.get("name").asString(),
                record.get("description").asString(),
                record.get("matchScore").asDouble()
            );
            
            rec.setReasons(List.of("Usuarios como tú también jugaron esto"));
            recommendations.add(rec);
        }
    }
    
    return recommendations;
}

public List<GameRecommendation> getMixedRecommendations(String userId) {
    // Combinar recomendaciones emocionales y sociales
    List<GameRecommendation> emotional = getRecommendationsForUser(userId);
    List<GameRecommendation> social = getSocialRecommendations(userId);
    
    List<GameRecommendation> mixed = new ArrayList<>();
    mixed.addAll(emotional);
    mixed.addAll(social);
    
    return mixed.stream()
        .sorted((a, b) -> Double.compare(b.getMatchScore(), a.getMatchScore()))
        .limit(5)
        .collect(Collectors.toList());
}
}
