// Actualización de resonancia basada en feedback


MATCH (u:Usuario)-[r:RESUENA_CON]->(j:Juego)
WHERE u.id = $usuario_id AND j.nombre = $juego_nombre
SET r.intensidad = r.intensidad + $delta_satisfaccion,
    r.ultima_actualizacion = datetime(),
    r.feedback_positivo = r.feedback_positivo + CASE WHEN $satisfaccion > 0 THEN 1 ELSE 0 END,
    r.feedback_negativo = r.feedback_negativo + CASE WHEN $satisfaccion < 0 THEN 1 ELSE 0 END
RETURN r.intensidad AS nueva_intensidad,
       r.feedback_positivo AS positivos,
       r.feedback_negativo AS negativos