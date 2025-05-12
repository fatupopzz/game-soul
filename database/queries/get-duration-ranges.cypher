// Obtener todos los rangos de duración
MATCH (r:RangoDuracion)
RETURN r.nombre AS nombre, r.min AS min, r.max AS max, r.descripcion AS descripcion
ORDER BY r.min