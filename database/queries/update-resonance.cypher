// Actualización de resonancia basada en feedback


MATCH (u:Usuario)-[r:RESUENA_CON]->(j:Juego)
WHERE u.id = $usuario_id AND j.nombre = $juego_nombre
SET r.intensidad = r.intensidad + $delta_satisfaccion,
    r.ultima_actualizacion = datetime()
RETURN r.intensidad as nueva_intensidad