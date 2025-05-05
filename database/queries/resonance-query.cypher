// Query principal de resonancia emocional

MATCH (u:Usuario)-[e:ESTADO_EMOCIONAL]->(estado:Emocion)
MATCH (j:Juego)-[r:RESUENA_CON]->(estado)
WHERE estado.tipo IN $emociones_usuario
  AND j.duracion_minima <= $tiempo_disponible
  AND NOT any(caracteristica IN j.caracteristicas 
      WHERE caracteristica IN $dealbreakers)
WITH j, 
     collect(r.intensidad) AS resonancias,
     collect(estado.tipo) AS emociones_coincidentes,
     size([x IN j.caracteristicas WHERE x IN $preferencias]) AS preferencias_match
WITH j, 
     reduce(s = 0.0, x IN resonancias | s + x) AS puntuacion_total,
     emociones_coincidentes,
     preferencias_match
// Aplicar bonus por preferencias
WITH j, 
     puntuacion_total + (preferencias_match * 0.5) AS puntuacion_final,
     emociones_coincidentes
ORDER BY puntuacion_final DESC
LIMIT 5
RETURN j.nombre AS juego,
       j.descripcion AS descripcion,
       j.imagen_url AS imagen,
       puntuacion_final AS resonancia,
       emociones_coincidentes AS emociones_match