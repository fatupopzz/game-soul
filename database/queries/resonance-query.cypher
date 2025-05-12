// Query principal de resonancia emocional
MATCH (u:Usuario)-[e:ESTADO_EMOCIONAL]->(estado:Emocion)
MATCH (j:Juego)-[r:RESUENA_CON]->(estado)

// Filtrar por duración usando la relación a RangoDuracion
WITH u, j, estado, r
MATCH (j)-[:TIENE_DURACION]->(rd:RangoDuracion)
WHERE rd.max >= $tiempo_disponible

// Filtrar por características excluidas ("dealbreakers")
WITH u, j, estado, r
WHERE NOT EXISTS {
    MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
    WHERE c.nombre IN $dealbreakers
}

// Calcular puntuación básica de resonancia emocional directa
WITH j, 
     collect(r.intensidad) AS resonancias_directas,
     collect(estado.tipo) AS emociones_coincidentes
WITH j, 
     reduce(s = 0.0, x IN resonancias_directas | s + x) AS puntuacion_directa,
     emociones_coincidentes

// Agregar puntuación por resonancia indirecta a través de géneros
OPTIONAL MATCH (j)-[:TIENE_GENERO]->(g:Genero)-[rg:RELACIONADO_CON]->(e:Emocion)
WHERE e.tipo IN emociones_coincidentes
WITH j, 
     puntuacion_directa,
     emociones_coincidentes,
     sum(coalesce(rg.intensidad * 0.5, 0)) AS puntuacion_genero

// Agregar puntuación por resonancia indirecta a través de características
OPTIONAL MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)-[rc:RELACIONADO_CON]->(e:Emocion)
WHERE e.tipo IN emociones_coincidentes
WITH j, 
     puntuacion_directa,
     puntuacion_genero,
     emociones_coincidentes,
     sum(coalesce(rc.intensidad * 0.3, 0)) AS puntuacion_caracteristica

// Calcular puntuación total ponderada
WITH j, 
     puntuacion_directa,
     puntuacion_genero,
     puntuacion_caracteristica,
     emociones_coincidentes,
     (puntuacion_directa * 1.0 + puntuacion_genero * 0.5 + puntuacion_caracteristica * 0.3) AS puntuacion_total

// Ordenar y limitar resultados
ORDER BY puntuacion_total DESC
LIMIT 3

// Retornar resultados con detalles
RETURN j.nombre AS juego, 
       j.descripcion AS descripcion,
       puntuacion_total AS resonancia,
       emociones_coincidentes