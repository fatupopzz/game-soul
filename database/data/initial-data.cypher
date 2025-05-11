// Crear usuarios de ejemplo


CREATE (u1:Usuario {
  id: "user1",
  nombre: "Usuario Prueba",
  estado: "relajado"
});

CREATE (u2:Usuario {
  id: "user2",
  nombre: "Usuario Aventurero",
  estado: "aventurero"
});

// Crear juegos de ejemplo


CREATE (j1:Juego {
  id: "game1",
  nombre: "Stardew Valley",
  descripcion: "Un juego relajante de simulación de granja",
  duracion_minima: 30,
  duracion_maxima: 200,
  genero: "simulación",
  caracteristicas: ["relajante", "creativo", "social"]
});

CREATE (j2:Juego {
  id: "game2",
  nombre: "Minecraft",
  descripcion: "Un mundo de exploración y creatividad",
  duracion_minima: 15,
  duracion_maxima: 999,
  genero: "sandbox",
  caracteristicas: ["creativo", "exploración", "desafiante"]
});

// Crear relaciones de resonancia
MATCH (j:Juego {nombre: "Stardew Valley"}), (e:Emocion {tipo: "relajante"})
CREATE (j)-[:RESUENA_CON {intensidad: 0.9}]->(e);

MATCH (j:Juego {nombre: "Minecraft"}), (e:Emocion {tipo: "creativo"})
CREATE (j)-[:RESUENA_CON {intensidad: 0.85}]->(e);

MATCH (j:Juego {nombre: "Minecraft"}), (e:Emocion {tipo: "exploración"})
CREATE (j)-[:RESUENA_CON {intensidad: 0.8}]->(e);

// Crear relaciones de usuario
MATCH (u:Usuario {id: "user1"}), (e:Emocion {tipo: "relajante"})
CREATE (u)-[:ESTADO_EMOCIONAL {fecha: datetime()}]->(e);

MATCH (u:Usuario {id: "user2"}), (e:Emocion {tipo: "exploración"})
CREATE (u)-[:ESTADO_EMOCIONAL {fecha: datetime()}]->(e);