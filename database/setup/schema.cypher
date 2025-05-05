// Crear constraints
CREATE CONSTRAINT game_name_unique IF NOT EXISTS
FOR (g:Juego) REQUIRE g.nombre IS UNIQUE;

CREATE CONSTRAINT user_id_unique IF NOT EXISTS
FOR (u:Usuario) REQUIRE u.id IS UNIQUE;

CREATE CONSTRAINT emotion_type_unique IF NOT EXISTS
FOR (e:Emocion) REQUIRE e.tipo IS UNIQUE;

// Crear índices
CREATE INDEX resonance_intensity IF NOT EXISTS
FOR ()-[r:RESUENA_CON]-() ON (r.intensidad);

CREATE INDEX game_duration IF NOT EXISTS
FOR (g:Juego) ON (g.duracion_minima, g.duracion_maxima);

// Crear nodos base de emociones
MERGE (e1:Emocion {tipo: "alegre"});
MERGE (e2:Emocion {tipo: "relajante"});
MERGE (e3:Emocion {tipo: "melancólico"});
MERGE (e4:Emocion {tipo: "exploración"});
MERGE (e5:Emocion {tipo: "desafiante"});
MERGE (e6:Emocion {tipo: "contemplativo"});
MERGE (e7:Emocion {tipo: "social"});
MERGE (e8:Emocion {tipo: "competitivo"});
MERGE (e9:Emocion {tipo: "creativo"});