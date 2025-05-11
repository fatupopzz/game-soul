# Game Soul - Documentación de Base de Datos

Esta carpeta contiene todos los scripts y consultas necesarios para configurar y utilizar la base de datos Neo4j para el sistema de recomendaciones emocional Game Soul.

## Estructura de Archivos

```
database/
├── data/                  # Datos de inicialización
│   └── initial-data.cypher  # Script para crear nodos y relaciones iniciales
│
├── queries/               # Consultas para diferentes funcionalidades
│   ├── exploration-query.cypher    # Consulta para recomendaciones exploratorias
│   ├── resonance-query.cypher      # Consulta principal de resonancia emocional
│   └── update-resonance.cypher     # Actualización de resonancia basada en feedback
│
├── schema/                # Definición del esquema
│   └── constraints.cypher   # Restricciones e índices de la base de datos
│
├── setup/                 # Scripts de configuración adicionales
│   └── schema.cypher        # Esquema base con emociones predefinidas
│
├── setup.sh               # Script de configuración para Unix/Linux/macOS
├── setup.ps1              # Script de configuración para Windows (PowerShell)
├── setup.bat              # Script de configuración alternativo para Windows
└── README.md              # Este archivo
```

## Modelo de Datos

El sistema utiliza un grafo con los siguientes nodos y relaciones:

### Nodos
- **Usuario**: Representa a un usuario del sistema
  - Propiedades: `id`, `nombre`, `estado`
  
- **Juego**: Representa un videojuego en el catálogo
  - Propiedades: `id`, `nombre`, `descripcion`, `duracion_minima`, `duracion_maxima`, `genero`, `caracteristicas`
  
- **Emocion**: Representa un estado emocional o experiencia de juego
  - Propiedades: `tipo` (por ejemplo: "relajante", "desafiante", "social")

### Relaciones
- **RESUENA_CON**: Conecta un juego con una emoción
  - Propiedades: `intensidad` (0.0 a 1.0)
  
- **ESTADO_EMOCIONAL**: Conecta un usuario con una emoción actual
  - Propiedades: `fecha`
  
- **HA_JUGADO**: Historial de juegos jugados por un usuario
  - Propiedades: `fecha`, `duracion`, `satisfaccion` (0 a 10)

## Configuración de la Base de Datos

### Requisitos previos
- Neo4j instalado (versión 4.4 o superior)
- Cypher-shell disponible en tu PATH (o ruta definida)

### Usando Docker (Recomendado)
La forma más fácil de configurar Neo4j es usando Docker:

1. Asegúrate de tener Docker y Docker Compose instalados
2. Desde la raíz del proyecto, ejecuta:
   ```
   docker-compose up -d
   ```
3. Esto iniciará Neo4j en el puerto 7474 (Browser) y 7687 (Bolt)

### Configuración manual

#### En sistemas Unix/Linux/macOS:
```bash
cd database
chmod +x setup.sh
./setup.sh
```

#### En Windows (PowerShell):
```powershell
cd database
.\setup.ps1
```

#### En Windows (CMD):
```cmd
cd database
setup.bat
```

#### Configuración manual sin scripts:
1. Accede a Neo4j Browser (http://localhost:7474)
2. Inicia sesión con las credenciales por defecto (neo4j/password)
3. Ejecuta secuencialmente:
   - El contenido de `schema/constraints.cypher`
   - El contenido de `data/initial-data.cypher`

## 📊 Consultas principales

### Recomendación por Resonancia Emocional
Esta consulta principal (`queries/resonance-query.cypher`) recomienda juegos que resuenan con el estado emocional actual del usuario:

```cypher
// Para ejecutar:
MATCH (u:Usuario)-[e:ESTADO_EMOCIONAL]->(estado:Emocion)
MATCH (j:Juego)-[r:RESUENA_CON]->(estado)
WHERE j.duracion_minima <= $tiempo_disponible
  AND NOT any(caracteristica IN j.caracteristicas 
      WHERE caracteristica IN $dealbreakers)
WITH j, 
     collect(r.intensidad) AS resonancias,
     collect(estado.tipo) AS emociones_coincidentes
WITH j, 
     reduce(s = 0.0, x IN resonancias | s + x) AS puntuacion_total,
     emociones_coincidentes
ORDER BY puntuacion_total DESC
LIMIT 3
RETURN j.nombre AS juego, 
       j.descripcion AS descripcion,
       puntuacion_total AS resonancia,
       emociones_coincidentes
```

### Recomendación para Exploración
Esta consulta (`queries/exploration-query.cypher`) recomienda juegos para prevenir la fatiga y fomentar exploración:

```cypher
// Para ejecutar:
MATCH (u:Usuario)-[h:HA_JUGADO]->(j:Juego)
WHERE h.fecha >= date() - duration('P30D')
WITH u, collect(j.genero) AS generos_recientes
MATCH (nuevo:Juego)
WHERE none(g IN nuevo.genero WHERE g IN generos_recientes)
  AND nuevo.duracion_minima <= $tiempo_disponible
WITH nuevo, rand() AS r
ORDER BY r
LIMIT 3
RETURN nuevo.nombre AS juego,
       nuevo.descripcion AS descripcion,
       "exploración" AS tipo_recomendacion
```

## Actualización de la Base de Datos

### Añadir un nuevo juego
```cypher
MERGE (j:Juego {id: "game_id"})
ON CREATE SET
  j.nombre = "Nombre del Juego",
  j.descripcion = "Descripción del juego",
  j.duracion_minima = 30,
  j.duracion_maxima = 120,
  j.genero = "género",
  j.caracteristicas = ["caract1", "caract2", "caract3"];

// Conectar con emociones
MATCH (j:Juego {id: "game_id"}), (e:Emocion {tipo: "tipo_emocion"})
CREATE (j)-[:RESUENA_CON {intensidad: 0.7}]->(e);
```

### Actualizar resonancia basada en feedback
```cypher
// Ejemplo de uso:
MATCH (u:Usuario)-[r:RESUENA_CON]->(j:Juego)
WHERE u.id = "user_id" AND j.nombre = "Nombre del Juego"
SET r.intensidad = r.intensidad + 0.1,
    r.ultima_actualizacion = datetime()
RETURN r.intensidad as nueva_intensidad
```

## Búsqueda y Exploración

Para explorar la base de datos visualmente:
1. Accede a Neo4j Browser: http://localhost:7474
2. Usa consultas como:
   ```cypher
   // Ver todos los juegos
   MATCH (j:Juego) RETURN j
   
   // Ver relaciones entre juegos y emociones
   MATCH (j:Juego)-[r:RESUENA_CON]->(e:Emocion) RETURN j, r, e
   
   // Ver usuarios y sus estados emocionales
   MATCH (u:Usuario)-[r:ESTADO_EMOCIONAL]->(e:Emocion) RETURN u, r, e
   ```

## Notas Importantes

1. **Backups**: Siempre realiza copias de seguridad antes de modificar el esquema
2. **Índices**: Los índices están configurados para optimizar consultas frecuentes
3. **Actualización de esquema**: Si necesitas añadir nuevas restricciones, hazlo en `schema/constraints.cypher`

## Contribuir

1. Mantén la estructura de carpetas
2. Documenta nuevas consultas
3. Usa MERGE en lugar de CREATE para evitar duplicados
4. Sigue las convenciones de nomenclatura existentes

