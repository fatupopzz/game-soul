// Añadir un nuevo género
MERGE (g:Genero {nombre: $nombre})
RETURN g.nombre AS genero_creado