// ===== COMPLETAR SISTEMA GAME SOUL SIN MODIFICAR DATOS EXISTENTES =====
// Este script solo AGREGA lo que falta, no modifica nada existente

// ===== 1. VERIFICAR ESTADO ACTUAL =====
MATCH (j:Juego) WITH count(j) as juegos_existentes
MATCH (e:Emocion) WITH juegos_existentes, count(e) as emociones_existentes
MATCH (g:Genero) WITH juegos_existentes, emociones_existentes, count(g) as generos_existentes
MATCH (c:Caracteristica) WITH juegos_existentes, emociones_existentes, generos_existentes, count(c) as caracteristicas_existentes

RETURN 
  "🎮 SISTEMA GAME SOUL COMPLETADO:" as titulo,
  juegos + " juegos totales" as juegos_status,
  emociones + " emociones disponibles" as emociones_status,
  generos + " géneros disponibles" as generos_status,
  caracteristicas + " características disponibles" as caracteristicas_status,
  usuarios + " usuarios de ejemplo" as usuarios_status,
  rel_emociones + " relaciones emocionales" as emociones_rel_status,
  rel_generos + " relaciones de género" as generos_rel_status,
  rel_caracteristicas + " relaciones de características" as caracteristicas_rel_status,
  "✅ Sistema funcional y listo para recomendaciones" as estado;

// ===== 10. EJEMPLO DE FUNCIONAMIENTO =====
// Mostrar cómo funciona el sistema con una consulta de prueba

MATCH (u:Usuario {id: "user1"})-[:ESTADO_EMOCIONAL]->(estado:Emocion)
MATCH (j:Juego)-[r:RESUENA_CON]->(estado)
WITH j, r.intensidad as resonancia, estado.tipo as emocion_usuario
ORDER BY resonancia DESC
LIMIT 3
RETURN 
  "🧪 PRUEBA DEL SISTEMA:" as titulo,
  "Usuario relajado debería recibir:" as descripcion,
  collect({juego: j.nombre, resonancia: resonancia, emocion: emocion_usuario}) as recomendaciones_ejemplo;"📊 ESTADO INICIAL:" as titulo,
  juegos_existentes + " juegos" as juegos,
  emociones_existentes + " emociones" as emociones,
  generos_existentes + " géneros" as generos,
  caracteristicas_existentes + " características" as caracteristicas;

// ===== 2. CREAR NODOS BASE FALTANTES (solo si no existen) =====

// Emociones
MERGE (e1:Emocion {tipo: "alegre"});
MERGE (e2:Emocion {tipo: "relajante"});
MERGE (e3:Emocion {tipo: "melancólico"});
MERGE (e4:Emocion {tipo: "exploración"});
MERGE (e5:Emocion {tipo: "desafiante"});
MERGE (e6:Emocion {tipo: "contemplativo"});
MERGE (e7:Emocion {tipo: "social"});
MERGE (e8:Emocion {tipo: "competitivo"});
MERGE (e9:Emocion {tipo: "creativo"});

// Géneros
MERGE (g1:Genero {nombre: "RPG"});
MERGE (g2:Genero {nombre: "Acción"});
MERGE (g3:Genero {nombre: "Aventura"});
MERGE (g4:Genero {nombre: "Simulación"});
MERGE (g5:Genero {nombre: "Estrategia"});
MERGE (g6:Genero {nombre: "Puzzle"});
MERGE (g7:Genero {nombre: "Sandbox"});
MERGE (g8:Genero {nombre: "Plataformas"});
MERGE (g9:Genero {nombre: "Deportes"});

// Características
MERGE (c1:Caracteristica {nombre: "relajante"});
MERGE (c2:Caracteristica {nombre: "desafiante"});
MERGE (c3:Caracteristica {nombre: "creativo"});
MERGE (c4:Caracteristica {nombre: "competitivo"});
MERGE (c5:Caracteristica {nombre: "social"});
MERGE (c6:Caracteristica {nombre: "historia_rica"});
MERGE (c7:Caracteristica {nombre: "exploración"});
MERGE (c8:Caracteristica {nombre: "multijugador"});
MERGE (c9:Caracteristica {nombre: "mundo_abierto"});
MERGE (c10:Caracteristica {nombre: "violento"});
MERGE (c11:Caracteristica {nombre: "educativo"});

// ===== 3. AGREGAR RELACIONES EMOCIONALES (solo donde no existan) =====

// Factorio
MATCH (j:Juego {nombre: "Factorio"})
MATCH (e:Emocion {tipo: "creativo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.9, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Factorio"})
MATCH (e:Emocion {tipo: "contemplativo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

// Hades
MATCH (j:Juego {nombre: "Hades"})
MATCH (e:Emocion {tipo: "desafiante"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.85, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Hades"})
MATCH (e:Emocion {tipo: "alegre"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.7, fecha: datetime()}]->(e);

// Among Us
MATCH (j:Juego {nombre: "Among Us"})
MATCH (e:Emocion {tipo: "social"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.9, fecha: datetime()}]->(e);

// Disco Elysium
MATCH (j:Juego {nombre: "Disco Elysium"})
MATCH (e:Emocion {tipo: "contemplativo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.95, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Disco Elysium"})
MATCH (e:Emocion {tipo: "melancólico"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

// Monster Hunter World
MATCH (j:Juego {nombre: "Monster Hunter World"})
MATCH (e:Emocion {tipo: "desafiante"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Monster Hunter World"})
MATCH (e:Emocion {tipo: "social"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.7, fecha: datetime()}]->(e);

// Rocket League
MATCH (j:Juego {nombre: "Rocket League"})
MATCH (e:Emocion {tipo: "competitivo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.9, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Rocket League"})
MATCH (e:Emocion {tipo: "alegre"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

// Subnautica
MATCH (j:Juego {nombre: "Subnautica"})
MATCH (e:Emocion {tipo: "exploración"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.9, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Subnautica"})
MATCH (e:Emocion {tipo: "contemplativo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.7, fecha: datetime()}]->(e);

// Slay the Spire
MATCH (j:Juego {nombre: "Slay the Spire"})
MATCH (e:Emocion {tipo: "contemplativo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Slay the Spire"})
MATCH (e:Emocion {tipo: "desafiante"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.7, fecha: datetime()}]->(e);

// Final Fantasy XIV
MATCH (j:Juego {nombre: "Final Fantasy XIV"})
MATCH (e:Emocion {tipo: "social"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Final Fantasy XIV"})
MATCH (e:Emocion {tipo: "contemplativo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.7, fecha: datetime()}]->(e);

// Satisfactory
MATCH (j:Juego {nombre: "Satisfactory"})
MATCH (e:Emocion {tipo: "creativo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.85, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Satisfactory"})
MATCH (e:Emocion {tipo: "relajante"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.6, fecha: datetime()}]->(e);

// Elden Ring
MATCH (j:Juego {nombre: "Elden Ring"})
MATCH (e:Emocion {tipo: "desafiante"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.9, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Elden Ring"})
MATCH (e:Emocion {tipo: "exploración"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

// Fall Guys
MATCH (j:Juego {nombre: "Fall Guys"})
MATCH (e:Emocion {tipo: "alegre"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.9, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Fall Guys"})
MATCH (e:Emocion {tipo: "social"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.7, fecha: datetime()}]->(e);

// League of Legends
MATCH (j:Juego {nombre: "League of Legends"})
MATCH (e:Emocion {tipo: "competitivo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.95, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "League of Legends"})
MATCH (e:Emocion {tipo: "desafiante"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

// Cuphead
MATCH (j:Juego {nombre: "Cuphead"})
MATCH (e:Emocion {tipo: "desafiante"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.9, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "Cuphead"})
MATCH (e:Emocion {tipo: "alegre"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.6, fecha: datetime()}]->(e);

// God of War
MATCH (j:Juego {nombre: "God of War (2018)"})
MATCH (e:Emocion {tipo: "contemplativo"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8, fecha: datetime()}]->(e);

MATCH (j:Juego {nombre: "God of War (2018)"})
MATCH (e:Emocion {tipo: "desafiante"})
WHERE NOT EXISTS((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.7, fecha: datetime()}]->(e);

// ===== 4. AGREGAR RELACIONES DE GÉNERO (solo donde no existan) =====

// Factorio - Estrategia/Simulación
MATCH (j:Juego {nombre: "Factorio"}), (g:Genero {nombre: "Estrategia"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

MATCH (j:Juego {nombre: "Factorio"}), (g:Genero {nombre: "Simulación"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 0.8}]->(g);

// Hades - Acción/RPG
MATCH (j:Juego {nombre: "Hades"}), (g:Genero {nombre: "Acción"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

MATCH (j:Juego {nombre: "Hades"}), (g:Genero {nombre: "RPG"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 0.7}]->(g);

// Among Us - Puzzle
MATCH (j:Juego {nombre: "Among Us"}), (g:Genero {nombre: "Puzzle"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// Disco Elysium - RPG
MATCH (j:Juego {nombre: "Disco Elysium"}), (g:Genero {nombre: "RPG"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// Monster Hunter World - Acción/RPG
MATCH (j:Juego {nombre: "Monster Hunter World"}), (g:Genero {nombre: "Acción"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

MATCH (j:Juego {nombre: "Monster Hunter World"}), (g:Genero {nombre: "RPG"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 0.8}]->(g);

// Rocket League - Deportes
MATCH (j:Juego {nombre: "Rocket League"}), (g:Genero {nombre: "Deportes"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// Subnautica - Aventura
MATCH (j:Juego {nombre: "Subnautica"}), (g:Genero {nombre: "Aventura"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// Slay the Spire - Estrategia
MATCH (j:Juego {nombre: "Slay the Spire"}), (g:Genero {nombre: "Estrategia"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// Final Fantasy XIV - RPG
MATCH (j:Juego {nombre: "Final Fantasy XIV"}), (g:Genero {nombre: "RPG"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// Satisfactory - Simulación/Sandbox
MATCH (j:Juego {nombre: "Satisfactory"}), (g:Genero {nombre: "Simulación"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

MATCH (j:Juego {nombre: "Satisfactory"}), (g:Genero {nombre: "Sandbox"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 0.8}]->(g);

// Elden Ring - RPG/Acción
MATCH (j:Juego {nombre: "Elden Ring"}), (g:Genero {nombre: "RPG"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

MATCH (j:Juego {nombre: "Elden Ring"}), (g:Genero {nombre: "Acción"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 0.9}]->(g);

// Fall Guys - Plataformas
MATCH (j:Juego {nombre: "Fall Guys"}), (g:Genero {nombre: "Plataformas"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// League of Legends - Estrategia
MATCH (j:Juego {nombre: "League of Legends"}), (g:Genero {nombre: "Estrategia"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// Cuphead - Plataformas
MATCH (j:Juego {nombre: "Cuphead"}), (g:Genero {nombre: "Plataformas"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// God of War - Acción/Aventura
MATCH (j:Juego {nombre: "God of War (2018)"}), (g:Genero {nombre: "Acción"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

MATCH (j:Juego {nombre: "God of War (2018)"}), (g:Genero {nombre: "Aventura"})
WHERE NOT EXISTS((j)-[:TIENE_GENERO]->(g))
CREATE (j)-[:TIENE_GENERO {relevancia: 0.8}]->(g);

// ===== 5. AGREGAR CARACTERÍSTICAS CLAVE (solo donde no existan) =====

// Características más importantes para cada juego
MATCH (j:Juego {nombre: "Factorio"}), (c:Caracteristica {nombre: "creativo"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Among Us"}), (c:Caracteristica {nombre: "social"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Among Us"}), (c:Caracteristica {nombre: "multijugador"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Disco Elysium"}), (c:Caracteristica {nombre: "historia_rica"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Monster Hunter World"}), (c:Caracteristica {nombre: "desafiante"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 0.9}]->(c);

MATCH (j:Juego {nombre: "Monster Hunter World"}), (c:Caracteristica {nombre: "multijugador"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 0.9}]->(c);

MATCH (j:Juego {nombre: "Rocket League"}), (c:Caracteristica {nombre: "competitivo"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Rocket League"}), (c:Caracteristica {nombre: "multijugador"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Subnautica"}), (c:Caracteristica {nombre: "exploración"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Subnautica"}), (c:Caracteristica {nombre: "mundo_abierto"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 0.8}]->(c);

MATCH (j:Juego {nombre: "Final Fantasy XIV"}), (c:Caracteristica {nombre: "historia_rica"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Final Fantasy XIV"}), (c:Caracteristica {nombre: "social"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 0.9}]->(c);

MATCH (j:Juego {nombre: "Final Fantasy XIV"}), (c:Caracteristica {nombre: "multijugador"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Satisfactory"}), (c:Caracteristica {nombre: "creativo"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Elden Ring"}), (c:Caracteristica {nombre: "desafiante"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Elden Ring"}), (c:Caracteristica {nombre: "exploración"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 0.9}]->(c);

MATCH (j:Juego {nombre: "Elden Ring"}), (c:Caracteristica {nombre: "mundo_abierto"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "League of Legends"}), (c:Caracteristica {nombre: "competitivo"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "League of Legends"}), (c:Caracteristica {nombre: "multijugador"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "Cuphead"}), (c:Caracteristica {nombre: "desafiante"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

MATCH (j:Juego {nombre: "God of War (2018)"}), (c:Caracteristica {nombre: "historia_rica"})
WHERE NOT EXISTS((j)-[:TIENE_CARACTERISTICA]->(c))
CREATE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

// ===== 6. CREAR USUARIOS DE EJEMPLO (solo si no existen) =====
MERGE (u1:Usuario {id: "user1"})
ON CREATE SET
  u1.nombre = "Usuario Relajado",
  u1.estado = "activo",
  u1.fecha_registro = datetime();

MERGE (u2:Usuario {id: "user2"})
ON CREATE SET
  u2.nombre = "Usuario Competitivo",
  u2.estado = "activo",
  u2.fecha_registro = datetime();

MERGE (u3:Usuario {id: "user3"})
ON CREATE SET
  u3.nombre = "Usuario Explorador",
  u3.estado = "activo",
  u3.fecha_registro = datetime();

// Estados emocionales de usuarios
MATCH (u1:Usuario {id: "user1"}), (e:Emocion {tipo: "relajante"})
WHERE NOT EXISTS((u1)-[:ESTADO_EMOCIONAL]->(e))
CREATE (u1)-[:ESTADO_EMOCIONAL {fecha: datetime(), intensidad: 0.8}]->(e);

MATCH (u2:Usuario {id: "user2"}), (e:Emocion {tipo: "competitivo"})
WHERE NOT EXISTS((u2)-[:ESTADO_EMOCIONAL]->(e))
CREATE (u2)-[:ESTADO_EMOCIONAL {fecha: datetime(), intensidad: 0.9}]->(e);

MATCH (u3:Usuario {id: "user3"}), (e:Emocion {tipo: "exploración"})
WHERE NOT EXISTS((u3)-[:ESTADO_EMOCIONAL]->(e))
CREATE (u3)-[:ESTADO_EMOCIONAL {fecha: datetime(), intensidad: 0.85}]->(e);

// ===== 7. GENERAR RELACIONES AUTOMÁTICAS GÉNERO-EMOCIÓN =====
MATCH (g:Genero)<-[:TIENE_GENERO]-(j:Juego)-[r:RESUENA_CON]->(e:Emocion)
WITH g, e, avg(r.intensidad) AS promedio_intensidad, count(*) AS conteo
WHERE conteo > 0 AND NOT EXISTS((g)-[:RELACIONADO_CON]->(e))
CREATE (g)-[:RELACIONADO_CON {intensidad: promedio_intensidad, conteo: conteo, ultima_actualizacion: datetime()}]->(e);

// ===== 8. GENERAR RELACIONES AUTOMÁTICAS CARACTERÍSTICA-EMOCIÓN =====
MATCH (c:Caracteristica)<-[:TIENE_CARACTERISTICA]-(j:Juego)-[r:RESUENA_CON]->(e:Emocion)
WITH c, e, avg(r.intensidad) AS promedio_intensidad, count(*) AS conteo
WHERE conteo > 0 AND NOT EXISTS((c)-[:RELACIONADO_CON]->(e))
CREATE (c)-[:RELACIONADO_CON {intensidad: promedio_intensidad, conteo: conteo, ultima_actualizacion: datetime()}]->(e);

// ===== 9. VERIFICACIÓN FINAL =====
MATCH (j:Juego) WITH count(j) as juegos
MATCH (e:Emocion) WITH juegos, count(e) as emociones
MATCH (g:Genero) WITH juegos, emociones, count(g) as generos
MATCH (c:Caracteristica) WITH juegos, emociones, generos, count(c) as caracteristicas
MATCH (u:Usuario) WITH juegos, emociones, generos, caracteristicas, count(u) as usuarios
MATCH ()-[r:RESUENA_CON]->() WITH juegos, emociones, generos, caracteristicas, usuarios, count(r) as rel_emociones
MATCH ()-[r:TIENE_GENERO]->() WITH juegos, emociones, generos, caracteristicas, usuarios, rel_emociones, count(r) as rel_generos
MATCH ()-[r:TIENE_CARACTERISTICA]->() WITH juegos, emociones, generos, caracteristicas, usuarios, rel_emociones, rel_generos, count(r) as rel_caracteristicas

RETURN