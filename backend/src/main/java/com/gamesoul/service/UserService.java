//------------------------------------
// Este se usará para manejar la lógica de negocio relacionada con los usuarios.
// Aquí se guardarán los perfiles de usuario, feedback de juegos y se procesará la retroalimentación social.
// También se crearán usuarios "seed" para mejorar las recomendaciones iniciales.
//-------------------------------------------


package com.gamesoul.service;

import java.util.Map;

import org.neo4j.driver.Driver;
import org.neo4j.driver.Session;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import com.gamesoul.model.dto.UserProfile;

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
    
    public void saveFeedback(String userId, String gameId, Boolean liked) {
        String feedbackQuery = """
            // Crear usuario si no existe
            MERGE (u:Usuario {id: $userId})
            ON CREATE SET u.nombre = $userId, u.estado = 'activo', u.fecha_registro = datetime()
            
            // Crear juego si no existe
            MERGE (j:Juego {id: $gameId})
            ON CREATE SET j.nombre = $gameId, j.descripcion = 'Juego generado automáticamente'
            
            // Crear relación de feedback
            MERGE (u)-[f:HA_JUGADO]->(j)
            SET f.liked = $liked,
                f.fecha = datetime(),
                f.peso = CASE WHEN $liked THEN 1.0 ELSE -0.5 END
            
            RETURN u.id as usuario_id, j.id as juego_id
            """;
        
        String createEmotionalStateQuery = """
            // Crear estado emocional automático si no existe
            MATCH (u:Usuario {id: $userId})
            WHERE NOT EXISTS((u)-[:ESTADO_EMOCIONAL]->())
            
            // Determinar emoción basada en el juego
            OPTIONAL MATCH (j:Juego {id: $gameId})-[r:RESUENA_CON]->(e:Emocion)
            WITH u, e, r.intensidad as intensidad
            ORDER BY intensidad DESC
            LIMIT 1
            
            // Si el juego tiene emoción, usarla; sino, usar 'alegre' por defecto
            WITH u, COALESCE(e, null) as emocion_juego
            
            OPTIONAL MATCH (emocion_default:Emocion {tipo: 'alegre'})
            WITH u, COALESCE(emocion_juego, emocion_default) as emocion_final
            
            MERGE (u)-[estado:ESTADO_EMOCIONAL]->(emocion_final)
            SET estado.intensidad = 0.7,
                estado.fecha = datetime(),
                estado.origen = 'auto_generado'
            
            RETURN u.id as usuario, emocion_final.tipo as emocion_asignada
            """;
        
        try (Session session = neo4jDriver.session()) {
            // Guardar feedback
            var result1 = session.run(feedbackQuery, Map.of(
                "userId", userId,
                "gameId", gameId,
                "liked", liked
            ));
            
            if (result1.hasNext()) {
                var record = result1.next();
                System.out.println("Feedback guardado: " + record.get("usuario_id").asString() + 
                                 " -> " + record.get("juego_id").asString() + " = " + liked);
            }
            
            // Crear estado emocional automático
            var result2 = session.run(createEmotionalStateQuery, Map.of("userId", userId, "gameId", gameId));
            
            if (result2.hasNext()) {
                var record = result2.next();
                System.out.println("🎭 Estado emocional asignado: " + record.get("usuario").asString() + 
                                 " -> " + record.get("emocion_asignada").asString());
            }
            
        } catch (Exception e) {
            System.out.println(" Error guardando feedback: " + e.getMessage());
            throw e;
        }
    }


public void processSocialFeedback(String userId, String gameId, boolean liked) {
    System.out.println("Procesando feedback social para: " + userId + " -> " + gameId + " = " + liked);
    
    // 1. PRIMERO: Crear similitudes basadas en gustos comunes
    String createSimilaritiesQuery = """
        MATCH (u1:Usuario {id: $userId})-[r1:HA_JUGADO]->(j:Juego)<-[r2:HA_JUGADO]-(u2:Usuario)
        WHERE u1 <> u2 AND r1.liked = true AND r2.liked = true
        WITH u1, u2, count(j) as juegos_comunes, collect(j.id) as juegos_compartidos
        WHERE juegos_comunes >= 1
        MERGE (u1)-[s:SIMILAR_A]->(u2)
        SET s.similitud = juegos_comunes * 0.3,
            s.juegos_comunes = juegos_comunes,
            s.juegos_compartidos = juegos_compartidos,
            s.ultima_actualizacion = datetime()
        RETURN u1.id as usuario1, u2.id as usuario2, juegos_comunes
        """;
    
    // 2. SEGUNDO: Si no hay suficientes similitudes, crear con usuarios seed
    String createSeedSimilaritiesQuery = """
        MATCH (tu:Usuario {id: $userId})
        MATCH (seed:Usuario)
        WHERE seed.id IN ['user2', 'similar_user_001', 'ana_relajada_1640000001']
          AND NOT EXISTS((tu)-[:SIMILAR_A]->(seed))
          AND tu <> seed
        WITH tu, seed, rand() AS randomScore
        WHERE randomScore > 0.7
        MERGE (tu)-[s:SIMILAR_A]->(seed)
        SET s.similitud = 0.4,
            s.juegos_comunes = 1,
            s.juegos_compartidos = [$gameId],
            s.ultima_actualizacion = datetime(),
            s.tipo = 'seed'
        RETURN tu.id as usuario, seed.id as usuario_seed
        """;
    
    try (Session session = neo4jDriver.session()) {
        // Ejecutar primera consulta
        var result1 = session.run(createSimilaritiesQuery, Map.of("userId", userId));
        
        int similitudesNaturales = 0;
        while (result1.hasNext()) {
            var record = result1.next();
            similitudesNaturales++;
            System.out.println("Similitud natural: " + record.get("usuario1").asString() + 
                             " <-> " + record.get("usuario2").asString() + 
                             " (" + record.get("juegos_comunes").asInt() + " juegos)");
        }
        
        System.out.println("📊 Similitudes naturales creadas: " + similitudesNaturales);
        
        // Si no hay suficientes similitudes naturales, crear con usuarios seed
        if (similitudesNaturales == 0) {
            System.out.println("No hay similitudes naturales, creando con usuarios seed...");
            
            var result2 = session.run(createSeedSimilaritiesQuery, Map.of("userId", userId, "gameId", gameId));
            
            int similitudesSeed = 0;
            while (result2.hasNext()) {
                var record = result2.next();
                similitudesSeed++;
                System.out.println("Similitud seed: " + record.get("usuario").asString() + 
                                 " <-> " + record.get("usuario_seed").asString());
            }
            
            System.out.println("Similitudes seed creadas: " + similitudesSeed);
        }
        
    } catch (Exception e) {
        System.out.println(" Error procesando feedback social: " + e.getMessage());
        e.printStackTrace();
    }
}

// AGREGAR método para crear usuarios seed automáticamente
public void ensureSeedUsers() {
    System.out.println("Verificando usuarios seed...");
    
    String createSeedUsersQuery = """
        // Crear usuario seed si no existe
        MERGE (seed1:Usuario {id: 'seed_relajado'})
        ON CREATE SET 
          seed1.nombre = 'Usuario Relajado Seed',
          seed1.estado = 'seed',
          seed1.fecha_registro = datetime()
        
        // Darle likes a juegos relajantes
        WITH seed1
        MATCH (j1:Juego {nombre: 'Stardew Valley'})
        MERGE (seed1)-[:HA_JUGADO {liked: true, fecha: datetime(), peso: 1.0}]->(j1)
        
        WITH seed1
        MATCH (j2:Juego {nombre: 'Journey'})
        MERGE (seed1)-[:HA_JUGADO {liked: true, fecha: datetime(), peso: 1.0}]->(j2)
        
        WITH seed1
        MATCH (j3:Juego {nombre: 'Animal Crossing: New Horizons'})
        MERGE (seed1)-[:HA_JUGADO {liked: true, fecha: datetime(), peso: 1.0}]->(j3)
        
        // Crear estado emocional
        WITH seed1
        MATCH (e:Emocion {tipo: 'relajante'})
        MERGE (seed1)-[:ESTADO_EMOCIONAL {intensidad: 0.8, fecha: datetime()}]->(e)
        
        RETURN seed1.id as seed_creado
        """;
    
    String createSeed2Query = """
        // Crear segundo usuario seed
        MERGE (seed2:Usuario {id: 'seed_aventurero'})
        ON CREATE SET 
          seed2.nombre = 'Usuario Aventurero Seed',
          seed2.estado = 'seed',
          seed2.fecha_registro = datetime()
        
        // Darle likes a juegos aventureros
        WITH seed2
        MATCH (j1:Juego {nombre: 'Elden Ring'})
        MERGE (seed2)-[:HA_JUGADO {liked: true, fecha: datetime(), peso: 1.0}]->(j1)
        
        WITH seed2
        MATCH (j2:Juego {nombre: 'Dark Souls'})
        MERGE (seed2)-[:HA_JUGADO {liked: true, fecha: datetime(), peso: 1.0}]->(j2)
        
        WITH seed2
        MATCH (j3:Juego {nombre: 'Monster Hunter World'})
        MERGE (seed2)-[:HA_JUGADO {liked: true, fecha: datetime(), peso: 1.0}]->(j3)
        
        // Crear estado emocional
        WITH seed2
        MATCH (e:Emocion {tipo: 'desafiante'})
        MERGE (seed2)-[:ESTADO_EMOCIONAL {intensidad: 0.9, fecha: datetime()}]->(e)
        
        RETURN seed2.id as seed_creado
        """;
    
    try (Session session = neo4jDriver.session()) {
        session.run(createSeedUsersQuery);
        session.run(createSeed2Query);
        System.out.println("Usuarios seed verificados/creados");
    } catch (Exception e) {
        System.out.println("Error creando usuarios seed: " + e.getMessage());
    }
}
}