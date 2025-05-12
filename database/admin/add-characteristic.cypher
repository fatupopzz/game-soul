// Añadir una nueva característica
MERGE (c:Caracteristica {nombre: $nombre})
RETURN c.nombre AS caracteristica_creada