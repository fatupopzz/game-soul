// Crear o actualizar un juego con el nuevo modelo
MERGE (j:Juego {id: $id})
ON CREATE SET 
    j.nombre = $nombre,
    j.descripcion = $descripcion
ON MATCH SET 
    j.nombre = $nombre,
    j.descripcion = $descripcion
    
// Eliminar relaciones existentes
WITH j
OPTIONAL MATCH (j)-[rg:TIENE_GENERO]->()
DELETE rg

WITH j
OPTIONAL MATCH (j)-[rc:TIENE_CARACTERISTICA]->()
DELETE rc

WITH j
OPTIONAL MATCH (j)-[rd:TIENE_DURACION]->()
DELETE rd

// Crear nuevas relaciones
WITH j
UNWIND $generos AS genero
MATCH (g:Genero {nombre: genero})
MERGE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g)

WITH j
UNWIND $caracteristicas AS caracteristica
MATCH (c:Caracteristica {nombre: caracteristica})
MERGE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c)

WITH j
MATCH (r:RangoDuracion)
WHERE r.min <= $duracion_minima AND $duracion_minima < r.max
MERGE (j)-[:TIENE_DURACION]->(r)

RETURN j.id AS id, j.nombre AS nombre, j.descripcion AS descripcion