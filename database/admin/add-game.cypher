// Script para añadir un nuevo juego con el modelo de atributos como nodos
// Parámetros esperados:
// $id - Identificador único del juego (ej: "game3")
// $nombre - Nombre del juego (ej: "The Legend of Zelda: Breath of the Wild")
// $descripcion - Descripción del juego
// $duracion_minima - Duración mínima estimada en minutos (ej: 60)
// $generos - Array de géneros (ej: ["aventura", "acción", "mundo abierto"])
// $caracteristicas - Array de características (ej: ["exploración", "desafiante", "historia"])
// $emociones - Mapa de emociones y sus intensidades (ej: {aventurero: 0.9, desafiante: 0.8, contemplativo: 0.7})

// 1. Crear el nodo del juego
MERGE (j:Juego {id: $id})
ON CREATE SET 
  j.nombre = $nombre,
  j.descripcion = $descripcion
ON MATCH SET 
  j.nombre = $nombre,
  j.descripcion = $descripcion;

// 2. Asignar géneros al juego (creándolos si no existen)
WITH j
UNWIND $generos AS genero_nombre
MERGE (g:Genero {nombre: genero_nombre})
MERGE (j)-[:TIENE_GENERO {relevancia: 1.0}]->(g);

// 3. Asignar características al juego (creándolas si no existen)
WITH j
UNWIND $caracteristicas AS caracteristica_nombre
MERGE (c:Caracteristica {nombre: caracteristica_nombre})
MERGE (j)-[:TIENE_CARACTERISTICA {relevancia: 1.0}]->(c);

// 4. Asignar rango de duración apropiado
WITH j
MATCH (r:RangoDuracion)
WHERE r.min <= $duracion_minima AND $duracion_minima < r.max
MERGE (j)-[:TIENE_DURACION]->(r);

// 5. Crear relaciones de resonancia emocional
WITH j
UNWIND keys($emociones) AS emocion_tipo
MATCH (e:Emocion {tipo: emocion_tipo})
MERGE (j)-[res:RESUENA_CON]->(e)
SET res.intensidad = $emociones[emocion_tipo],
    res.ultima_actualizacion = datetime();

// 6. Actualizar relaciones entre géneros y emociones
WITH j
MATCH (j)-[:TIENE_GENERO]->(g:Genero)
MATCH (j)-[res:RESUENA_CON]->(e:Emocion)
MERGE (g)-[rel:RELACIONADO_CON]->(e)
ON CREATE SET 
  rel.intensidad = res.intensidad * 0.8,
  rel.conteo = 1
ON MATCH SET
  rel.intensidad = (rel.intensidad * rel.conteo + res.intensidad * 0.8) / (rel.conteo + 1),
  rel.conteo = rel.conteo + 1,
  rel.ultima_actualizacion = datetime();

// 7. Actualizar relaciones entre características y emociones
WITH j
MATCH (j)-[:TIENE_CARACTERISTICA]->(c:Caracteristica)
MATCH (j)-[res:RESUENA_CON]->(e:Emocion)
MERGE (c)-[rel:RELACIONADO_CON]->(e)
ON CREATE SET 
  rel.intensidad = res.intensidad * 0.6,
  rel.conteo = 1
ON MATCH SET
  rel.intensidad = (rel.intensidad * rel.conteo + res.intensidad * 0.6) / (rel.conteo + 1),
  rel.conteo = rel.conteo + 1,
  rel.ultima_actualizacion = datetime();

// 8. Retornar información del juego agregado
MATCH (j:Juego {id: $id})
RETURN j.id AS id, 
       j.nombre AS nombre, 
       j.descripcion AS descripcion,
       [(j)-[:TIENE_GENERO]->(g) | g.nombre] AS generos,
       [(j)-[:TIENE_CARACTERISTICA]->(c) | c.nombre] AS caracteristicas,
       [(j)-[:TIENE_DURACION]->(r) | r.nombre] AS rango_duracion,
       [(j)-[res:RESUENA_CON]->(e) | {emocion: e.tipo, intensidad: res.intensidad}] AS resonancias;