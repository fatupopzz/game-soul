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

/**
 * Servicio encargado de genrar recomendciones de juegos para los usuarios 
 * en base a distintos criterios como emociones y preferencias de usuarios similares.
 */
@Service
public class RecommendationService {
    
    @Autowired
    private Driver neo4jDriver;
    
    // MÉTODO EXISTENTE - recomendaciones emocionales
    /**
     * Genera una lsita de juegos recomendados en base al estado emocional del usuario. 
     * 
     * @param userId ID del usuaio al que se le generarían las recomendaciones.
     * @return Lista de recomendaciones emocionales (máximo 5).
     */
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
    
    // MÉTODO EXISTENTE - recomendaciones por emoción
    /**
     * Genera una lista de juegos recomendados en base a un tipo de emoción específico.
     * 
     * @param emotion Tipo ed emoción (ej. "felicidad", "tristeza").
     * @return Lista de recomendaciones por emoción (máximo 5).
     */
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

    // NUEVO MÉTODO - AGREGAR AQUÍ
    /**
     * Genera una lista de recomendaciones basadas en juegos que han sido jugados y gustados por usuarios similares, 
     * pero que el usuario actual no ha jugado
     * 
     * @param userId ID del usuario para el que se generarán recomendaciones sociales.
     * @return Lista de reocmendaciones sociales (máximo 5).
     */
    public List<GameRecommendation> getSocialRecommendations(String userId) {
        System.out.println("👥 Buscando recomendaciones sociales para: " + userId);
        
        String query = """
            MATCH (u:Usuario {id: $userId})-[:SIMILAR_A]->(similar:Usuario)
            MATCH (similar)-[r:HA_JUGADO]->(recomendado:Juego)
            WHERE r.liked = true
              AND NOT EXISTS((u)-[:HA_JUGADO]->(recomendado))
            WITH recomendado, 
                 count(similar) as popularidad,
                 collect(similar.id)[0..3] as recomendado_por
            ORDER BY popularidad DESC
            LIMIT 5
            RETURN recomendado.id as id,
                   recomendado.nombre as name,
                   recomendado.descripcion as description,
                   popularidad * 0.2 as matchScore,
                   recomendado_por,
                   popularidad
            """;
        
        List<GameRecommendation> recommendations = new ArrayList<>();
        
        try (Session session = neo4jDriver.session()) {
            var result = session.run(query, Map.of("userId", userId));
            
            while (result.hasNext()) {
                var record = result.next();
                GameRecommendation rec = new GameRecommendation(
                    record.get("id").asString(),
                    record.get("name").asString(),
                    record.get("description").asString(),
                    record.get("matchScore").asDouble()
                );
                
                rec.setReasons(List.of("👥 Usuarios como tú también jugaron esto"));
                recommendations.add(rec);
                
                System.out.println("🎮 Recomendación social: " + rec.getName() + 
                                 " (popularidad: " + record.get("popularidad").asInt() + ")");
            }
            
            System.out.println("📋 Total recomendaciones sociales: " + recommendations.size());
            
        } catch (Exception e) {
            System.out.println("❌ Error obteniendo recomendaciones sociales: " + e.getMessage());
            e.printStackTrace();
        }
        
        return recommendations;
    }

    // MÉTODO MIXTO - AGREGAR TAMBIÉN
    /**
     * Combina las recomendaciones emocionales y sociales en una sola lista, ordenadas por puntaje, 
     * y devuelve las 5 mejores recomendaciones. 
     * 
     * @param userId ID del usuario para el que se generarán recomendaciones mixtas. 
     * @return Lista combinada de recomendaciones emocionales y sociales. 
     */
    public List<GameRecommendation> getMixedRecommendations(String userId) {
        System.out.println("🔀 Obteniendo recomendaciones mixtas para: " + userId);
        
        // Combinar recomendaciones emocionales y sociales
        List<GameRecommendation> emotional = getRecommendationsForUser(userId);
        List<GameRecommendation> social = getSocialRecommendations(userId);
        
        System.out.println("💝 Recomendaciones emocionales: " + emotional.size());
        System.out.println("👥 Recomendaciones sociales: " + social.size());
        
        List<GameRecommendation> mixed = new ArrayList<>();
        mixed.addAll(emotional);
        mixed.addAll(social);
        
        return mixed.stream()
            .sorted((a, b) -> Double.compare(b.getMatchScore(), a.getMatchScore()))
            .limit(5)
            .collect(Collectors.toList());
    }
}