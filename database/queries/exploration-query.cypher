// Query para prevenir fatiga y fomentar exploración
MATCH (u:Usuario)-[h:HA_JUGADO]->(j:Juego)
WHERE h.fecha >= date() - duration('P30D')
WITH u, collect(j.genero) AS generos_recientes
MATCH (nuevo:Juego)
WHERE none(g IN nuevo.genero WHERE g IN generos_recientes)
  AND nuevo.duracion_minima <= $tiempo_disponible
WITH nuevo, rand() AS r
ORDER BY r
LIMIT 3
RETURN nuevo.nombre AS juego,
       nuevo.descripcion AS descripcion,
       "exploración" AS tipo_recomendacion