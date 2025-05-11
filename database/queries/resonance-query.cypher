// Query principal de resonancia emocional
MATCH (u:Usuario)-[e:ESTADO_EMOCIONAL]->(estado:Emocion)
MATCH (j:Juego)-[r:RESUENA_CON]->(estado)
WHERE j.duracion_minima <= $tiempo_disponible
  AND NOT any(caracteristica IN j.caracteristicas 
      WHERE caracteristica IN $dealbreakers)
WITH j, 
     collect(r.intensidad) AS resonancias,
     collect(estado.tipo) AS emociones_coincidentes
WITH j, 
     reduce(s = 0.0, x IN resonancias | s + x) AS puntuacion_total,
     emociones_coincidentes
ORDER BY puntuacion_total DESC
LIMIT 3
RETURN j.nombre AS juego, 
       j.descripcion AS descripcion,
       puntuacion_total AS resonancia,
       emociones_coincidentes