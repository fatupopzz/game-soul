#  Game Soul - Sistema de Recomendaciones Emocional

*Un sistema inteligente que recomienda videojuegos basándose en tu estado emocional actual*

##  Nuestro Equipo
- **Ismalej** - Desarrollo Backend y Base de Datos
- **Adrina** - Desarrollo Frontend y UX  
- **Fatima** - Integración y Testing

---

##  ¿Qué es Game Soul?

Game Soul es nuestra solución para el problema de "no sé qué jugar hoy". Utilizamos un grafo de conocimiento en Neo4j para mapear las relaciones entre videojuegos y emociones, creando recomendaciones personalizadas que realmente conecten con cómo te sientes.

### Que hace?
- **Resonancia emocional**: Los juegos están conectados a emociones específicas con diferentes intensidades
- **Aprendizaje adaptativo**: El sistema mejora sus recomendaciones basándose en tu feedback
- **Exploración inteligente**: Previene la fatiga sugiriendo géneros que no has jugado recientemente

---

## Arquitectura del Proyecto

```
Game Soul/
├── backend/                 # API REST con Spring Boot
│   ├── src/main/java/      # Código Java
│   ├── Dockerfile          # Contenedor del backend
│   └── pom.xml             # Dependencias Maven
├── database/               # Scripts y consultas Neo4j
│   ├── queries/           # Consultas principales del sistema
│   ├── schema/            # Definición de constraints
│   └── data/              # Datos iniciales
├── frontend/              # Interfaz de usuario (React/Svelte)
└── docker-compose.yml     # Orquestación de servicios
```

---

##  Configuración Inicial

### Paso 1: Prerrequisitos
Asegúrense de tener instalado:
- **Java 17+** (para el backend)
- **Maven 3.6+** (para compilar)
- **Docker Desktop** (la forma más fácil de usar Neo4j)
- **Node.js 18+** (para el frontend)

### Paso 2: Levantar Neo4j
```bash
# En la raíz del proyecto
docker-compose up -d neo4j
```

Esto iniciará Neo4j en:
- Browser: http://localhost:7474
- Usuario: `neo4j`
- Contraseña: `password`

### Paso 3: Configurar la Base de Datos
1. Abre Neo4j Browser (http://localhost:7474)
2. Conéctate con las credenciales de arriba
3. Copia y pega el contenido de `database/schema/constraints.cypher`
4. Ejecuta la consulta (Ctrl+Enter)
5. Copia y pega el contenido de `database/data/initial-data.cypher`
6. Ejecuta la consulta

### Paso 4: Levantar el Backend
```bash
cd backend
mvn clean install
mvn spring-boot:run
```

El backend estará disponible en: http://localhost:8080/api

### Paso 5: Probar que Todo Funciona
Visita: http://localhost:8080/api/test/hello

Deberías ver un mensaje de confirmación del backend.

---

##  Funcionalidades Principales

### 1. Recomendación por Resonancia Emocional
El corazón del sistema. Conecta el estado emocional del usuario con juegos que "resuenan" con esa emoción.

**Archivo**: `database/queries/resonance-query.cypher`

### 2. Exploración Inteligente
Previene la fatiga sugiriendo juegos de géneros que no has jugado recientemente.

**Archivo**: `database/queries/exploration-query.cypher`

### 3. Aprendizaje por Feedback
Mejora las recomendaciones basándose en qué tan satisfecho quedaste con un juego.

**Archivo**: `database/queries/update-resonance.cypher`

---

##  Tareas Comunes de Desarrollo

### Agregar un Nuevo Juego
1. Abre Neo4j Browser
2. Usa el script `database/admin/add-game.cypher`
3. Proporciona los parámetros necesarios:
   ```cypher
   // Ejemplo
   :param id => "zelda-botw"
   :param nombre => "The Legend of Zelda: Breath of the Wild"
   :param descripcion => "Aventura épica en mundo abierto"
   :param duracion_minima => 60
   :param generos => ["aventura", "acción", "mundo abierto"]
   :param caracteristicas => ["exploración", "libertad", "contemplativo"]
   :param emociones => {exploración: 0.9, contemplativo: 0.8, aventurero: 0.85}
   ```

### Ver los Datos en Neo4j
```cypher
// Ver todos los juegos
MATCH (j:Juego) RETURN j

// Ver relaciones entre juegos y emociones
MATCH (j:Juego)-[r:RESUENA_CON]->(e:Emocion) 
RETURN j.nombre, e.tipo, r.intensidad

// Ver el grafo completo (¡cuidado, puede ser mucho!)
MATCH (n)-[r]->(m) RETURN n, r, m LIMIT 100
```

### Probar las Consultas Principales
```cypher
// Recomendación emocional (simula usuario relajado con 60 min disponibles)
:param usuario_id => "user1"
:param tiempo_disponible => 60
:param dealbreakers => ["violento", "competitivo"]

// Luego ejecuta el contenido de resonance-query.cypher
```

---

##  Solución de Problemas Comunes

### "No puedo conectar a Neo4j"
- Verifica que Docker esté corriendo: `docker ps`
- Reinicia el contenedor: `docker-compose restart neo4j`
- Espera un momento, Neo4j tarda en inicializar

### "El backend no encuentra Neo4j"
- Verifica la configuración en `backend/src/main/resources/application.yml`
- Asegúrate de que Neo4j esté en el puerto 7687

### "Las consultas no devuelven resultados"
- Verifica que los datos iniciales se hayan cargado correctamente
- Ejecuta: `MATCH (n) RETURN count(n)` para ver si hay nodos

### "Error de compilación en Maven"
- Verifica la versión de Java: `java -version`
- Limpia el cache: `mvn clean`

---

##  Estructura de la Base de Datos

### Nodos Principales
- **Usuario**: Representa a cada usuario del sistema
- **Juego**: Catálogo de videojuegos con sus propiedades
- **Emocion**: Estados emocionales base del sistema
- **Genero**: Categorías de juegos (RPG, Acción, etc.)
- **Caracteristica**: Atributos específicos (relajante, desafiante, etc.)

### Relaciones Clave
- **RESUENA_CON**: Un juego resuena con una emoción (tiene intensidad)
- **ESTADO_EMOCIONAL**: El estado actual de un usuario
- **HA_JUGADO**: Historial de juegos de un usuario
- **TIENE_GENERO/CARACTERISTICA**: Propiedades de los juegos

---

##  Frontend - Próximos Pasos

El frontend está configurado para ser desarrollado en React o Svelte (¡tu elección!). 

### Para React:
```bash
cd frontend
npm create react-app . --template typescript
npm install axios tailwindcss
```

### Para Svelte:
```bash
cd frontend
npm create svelte@latest .
npm install axios
```

### APIs Disponibles
- `GET /api/test/hello` - Verificar que el backend funciona
- Próximamente: endpoints para recomendaciones, usuarios, etc.

---


##  Cómo Colaborar

1. **Actualiza tu rama local**:
   ```bash
   git pull origin main
   ```

2. **Crea tu rama de feature**:
   ```bash
   git checkout -b feature/nueva-funcionalidad
   ```

3. **Haz commits descriptivos**:
   ```bash
   git commit -m "feat: agregar endpoint para buscar juegos por género"
   ```

4. **Sube tu rama y crea PR**:
   ```bash
   git push origin feature/nueva-funcionalidad
   ```


##  ¿Necesitas Ayuda?

- **Problemas con Neo4j**: Revisa la sección de troubleshooting o consulta la documentación oficial
- **Dudas de Spring Boot**: La documentación está en el directorio `backend/docs/`
- **Issues del proyecto**: Usa el sistema de issues de Git para reportar bugs

---
