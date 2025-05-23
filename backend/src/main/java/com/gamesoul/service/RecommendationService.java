package com.gamesoul.service;

import com.gamesoul.model.dto.GameRecommendation;
import org.neo4j.driver.Driver;
import org.neo4j.driver.Record;
import org.neo4j.driver.Result;
import org.neo4j.driver.Session;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

@Service
public class RecommendationService {
    
    @Autowired
    private Driver neo4jDriver;
    
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
                recommendations.add(rec);
            }
        }
        
        return recommendations;
    }
}
