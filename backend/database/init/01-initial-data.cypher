// Crear emociones base
CREATE (e1:Emocion {tipo: "alegre", descripcion: "Experiencias divertidas y positivas"});
CREATE (e2:Emocion {tipo: "relajante", descripcion: "Experiencias calmadas y sin estrés"});
CREATE (e3:Emocion {tipo: "melancólico", descripcion: "Experiencias emotivas y nostálgicas"});
CREATE (e4:Emocion {tipo: "exploración", descripcion: "Experiencias de descubrimiento"});
CREATE (e5:Emocion {tipo: "desafiante", descripcion: "Experiencias que prueban habilidades"});
CREATE (e6:Emocion {tipo: "contemplativo", descripcion: "Experiencias reflexivas"});
CREATE (e7:Emocion {tipo: "social", descripcion: "Experiencias de conexión con otros"});
CREATE (e8:Emocion {tipo: "competitivo", descripcion: "Experiencias de competición"});
CREATE (e9:Emocion {tipo: "creativo", descripcion: "Experiencias de expresión y creación"});

// Crear algunos juegos de ejemplo
CREATE (j1:Juego {
  id: "stardew-valley",
  nombre: "Stardew Valley", 
  descripcion: "Simulador de granja relajante con elementos sociales",
  genero: "Simulación",
  duracion_estimada: 120
});

CREATE (j2:Juego {
  id: "dark-souls",
  nombre: "Dark Souls", 
  descripcion: "RPG de acción extremadamente desafiante",
  genero: "RPG",
  duracion_estimada: 240
});

CREATE (j3:Juego {
  id: "minecraft",
  nombre: "Minecraft", 
  descripcion: "Juego de construcción y exploración creativo",
  genero: "Sandbox",
  duracion_estimada: 180
});

// Crear relaciones juego-emoción
MATCH (j1:Juego {id: "stardew-valley"}), (e:Emocion {tipo: "relajante"})
CREATE (j1)-[:RESUENA_CON {intensidad: 0.9}]->(e);

MATCH (j1:Juego {id: "stardew-valley"}), (e:Emocion {tipo: "social"})
CREATE (j1)-[:RESUENA_CON {intensidad: 0.7}]->(e);

MATCH (j2:Juego {id: "dark-souls"}), (e:Emocion {tipo: "desafiante"})
CREATE (j2)-[:RESUENA_CON {intensidad: 0.95}]->(e);

MATCH (j3:Juego {id: "minecraft"}), (e:Emocion {tipo: "creativo"})
CREATE (j3)-[:RESUENA_CON {intensidad: 0.9}]->(e);

MATCH (j3:Juego {id: "minecraft"}), (e:Emocion {tipo: "exploración"})
CREATE (j3)-[:RESUENA_CON {intensidad: 0.8}]->(e);
