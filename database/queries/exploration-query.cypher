// Query para prevenir fatiga y fomentar exploración
MATCH (u:Usuario {id: $usuario_id})-[h:HA_JUGADO]->(j:Juego)-[:TIENE_GENERO]->(g:Genero)
WHERE h.fecha >= date() - duration('P30D')
WITH u, collect(g.nombre) AS generos_recientes

// Encontrar juegos de géneros diferentes
MATCH (nuevo:Juego)-[:TIENE_GENERO]->(g:Genero)
WHERE NOT g.nombre IN generos_recientes

// Filtrar por tiempo disponible
MATCH (nuevo)-[:TIENE_DURACION]->(rd:RangoDuracion)
WHERE rd.max >= $tiempo_disponible

// Encontrar juegos con características interesantes 
MATCH (nuevo)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
WHERE NOT EXISTS {
    MATCH (u)-[h:HA_JUGADO]->(j)-[:TIENE_CARACTERISTICA]->(c)
    WHERE h.fecha >= date() - duration('P30D')
}

// Dar prioridad a juegos con emociones similares a las que al usuario le gustan
WITH nuevo, rand() AS puntuacion_exploracion
ORDER BY puntuacion_exploracion DESC
LIMIT 3

// Retornar recomendaciones exploratorias
RETURN nuevo.nombre AS juego,
       nuevo.descripcion AS descripcion,
       "exploración" AS tipo_recomendacion