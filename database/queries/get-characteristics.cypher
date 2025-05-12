// Obtener todas las características
MATCH (c:Caracteristica)
RETURN c.nombre AS nombre
ORDER BY nombre