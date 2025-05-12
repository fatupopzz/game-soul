// Obtener todos los géneros
MATCH (g:Genero)
RETURN g.nombre AS nombre
ORDER BY nombre