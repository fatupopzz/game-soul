// Crear y actualizar relaciones entre géneros y emociones
MATCH (g:Genero)<-[:TIENE_GENERO]-(j:Juego)-[r:RESUENA_CON]->(e:Emocion)
WITH g, e, avg(r.intensidad) AS promedio_intensidad, count(*) AS conteo
WHERE conteo > 0
MERGE (g)-[rel:RELACIONADO_CON]->(e)
ON CREATE SET 
  rel.intensidad = promedio_intensidad,
  rel.conteo = conteo
ON MATCH SET
  rel.intensidad = (rel.intensidad * rel.conteo + promedio_intensidad * conteo) / (rel.conteo + conteo),
  rel.conteo = rel.conteo + conteo,
  rel.ultima_actualizacion = datetime();

// Crear y actualizar relaciones entre características y emociones
MATCH (c:Caracteristica)<-[:TIENE_CARACTERISTICA]-(j:Juego)-[r:RESUENA_CON]->(e:Emocion)
WITH c, e, avg(r.intensidad) AS promedio_intensidad, count(*) AS conteo
WHERE conteo > 0
MERGE (c)-[rel:RELACIONADO_CON]->(e)
ON CREATE SET 
  rel.intensidad = promedio_intensidad,
  rel.conteo = conteo
ON MATCH SET
  rel.intensidad = (rel.intensidad * rel.conteo + promedio_intensidad * conteo) / (rel.conteo + conteo),
  rel.conteo = rel.conteo + conteo,
  rel.ultima_actualizacion = datetime();

RETURN "Mantenimiento de relaciones completado" AS mensaje