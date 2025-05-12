// Actualización de resonancia basada en feedback
MATCH (u:Usuario {id: $usuario_id})
MATCH (j:Juego {id: $juego_id})
MERGE (u)-[r:RESUENA_CON]->(j)
ON CREATE SET r.intensidad = 0.5
SET r.intensidad = r.intensidad + $delta_satisfaccion,
    r.ultima_actualizacion = datetime()

// Actualizar resonancia indirecta para géneros
WITH u, j, $delta_satisfaccion as delta
MATCH (j)-[:TIENE_GENERO]->(g:Genero)
MERGE (u)-[rg:AFINIDAD_CON]->(g)
ON CREATE SET rg.intensidad = 0.5
SET rg.intensidad = rg.intensidad + (delta * 0.3),
    rg.ultima_actualizacion = datetime()

// Actualizar resonancia indirecta para características
WITH u, j, $delta_satisfaccion as delta
MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
MERGE (u)-[rc:AFINIDAD_CON]->(c)
ON CREATE SET rc.intensidad = 0.5
SET rc.intensidad = rc.intensidad + (delta * 0.2),
    rc.ultima_actualizacion = datetime()

RETURN "Feedback procesado correctamente" as mensaje