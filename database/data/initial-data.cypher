// Crear usuarios de ejemplo (usando MERGE para evitar duplicados)

MERGE (u1:Usuario {
  id: "user1"
})
ON CREATE SET
  u1.nombre = "Usuario Prueba",
  u1.estado = "relajado";

MERGE (u2:Usuario {
  id: "user2"
})
ON CREATE SET
  u2.nombre = "Usuario Aventurero",
  u2.estado = "aventurero";

// Crear juegos de ejemplo (usando MERGE para evitar duplicados)

MERGE (j1:Juego {
  id: "game1"
})
ON CREATE SET
  j1.nombre = "Stardew Valley",
  j1.descripcion = "Un juego relajante de simulación de granja",
  j1.duracion_minima = 30,
  j1.duracion_maxima = 200,
  j1.genero = "simulación",
  j1.caracteristicas = ["relajante", "creativo", "social"];

MERGE (j2:Juego {
  id: "game2"
})
ON CREATE SET
  j2.nombre = "Minecraft",
  j2.descripcion = "Un mundo de exploración y creatividad",
  j2.duracion_minima = 15,
  j2.duracion_maxima = 999,
  j2.genero = "sandbox",
  j2.caracteristicas = ["creativo", "exploración", "desafiante"];

// Crear relaciones de resonancia (con verificación previa)
MATCH (j:Juego {nombre: "Stardew Valley"}), (e:Emocion {tipo: "relajante"})
WHERE NOT EXISTS ((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.9}]->(e);

MATCH (j:Juego {nombre: "Minecraft"}), (e:Emocion {tipo: "creativo"})
WHERE NOT EXISTS ((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.85}]->(e);

MATCH (j:Juego {nombre: "Minecraft"}), (e:Emocion {tipo: "exploración"})
WHERE NOT EXISTS ((j)-[:RESUENA_CON]->(e))
CREATE (j)-[:RESUENA_CON {intensidad: 0.8}]->(e);

// Crear relaciones de usuario (con verificación previa)
MATCH (u:Usuario {id: "user1"}), (e:Emocion {tipo: "relajante"})
WHERE NOT EXISTS ((u)-[:ESTADO_EMOCIONAL]->(e))
CREATE (u)-[:ESTADO_EMOCIONAL {fecha: datetime()}]->(e);

MATCH (u:Usuario {id: "user2"}), (e:Emocion {tipo: "exploración"})
WHERE NOT EXISTS ((u)-[:ESTADO_EMOCIONAL]->(e))
CREATE (u)-[:ESTADO_EMOCIONAL {fecha: datetime()}]->(e);