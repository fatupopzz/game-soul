# Game Soul - Sistema de Recomendaciones Emocional

> *Conectando emociones con experiencias de juego*

Un sistema inteligente que recomienda videojuegos basándose en tu estado emocional actual, utilizando grafos de conocimiento y algoritmos de resonancia emocional con recomendaciones sociales.

![Java](https://img.shields.io/badge/Java-17-orange?style=flat&logo=openjdk)
![Spring Boot](https://img.shields.io/badge/Spring%20Boot-3.2.0-green?style=flat&logo=spring)
![React](https://img.shields.io/badge/React-19.1-blue?style=flat&logo=react)
![Neo4j](https://img.shields.io/badge/Neo4j-Latest-red?style=flat&logo=neo4j)

---

## Requisitos del Sistema

### Software Necesario (INSTALAR ANTES DE CONTINUAR)
- **Java 17 o superior** - [Descargar aquí](https://adoptium.net/)
- **Maven 3.6 o superior** - [Descargar aquí](https://maven.apache.org/download.cgi)
- **Node.js 18 o superior** - [Descargar aquí](https://nodejs.org/)
- **Docker Desktop** - [Descargar aquí](https://www.docker.com/products/docker-desktop/)
- **Git** - [Descargar aquí](https://git-scm.com/)

### Verificar Instalaciones
Ejecutar estos comandos en terminal/cmd para verificar:
```bash
java -version     # Debe mostrar version 17+
mvn -version      # Debe mostrar version 3.6+
node -version     # Debe mostrar version 18+
npm -version      # Debe mostrar alguna version
docker --version  # Debe mostrar version instalada
git --version     # Debe mostrar version instalada
```

**Si algún comando falla, instalar el software faltante antes de continuar.**

---

## Instalación Completa (Paso a Paso)

### PASO 1: Descargar el Proyecto
```bash
git clone https://github.com/tu-usuario/game-soul.git
cd game-soul
```

### PASO 2: Levantar Neo4j (Base de Datos)
```bash
# Levantar Neo4j con Docker
docker-compose up -d neo4j

# Verificar que esté corriendo (debe aparecer neo4j)
docker ps
```

**IMPORTANTE**: Esperar 60 segundos para que Neo4j termine de inicializarse.

### PASO 3: Configurar la Base de Datos

#### 3.1 Acceder a Neo4j Browser
1. Abrir en navegador: **http://localhost:7474**
2. Conectar con:
   - **URI**: bolt://localhost:7687
   - **Usuario**: neo4j
   - **Contraseña**: password

#### 3.2 Crear Esquema (COPIAR Y PEGAR)
En Neo4j Browser, copiar y pegar este código, luego presionar **Ctrl+Enter**:

```cypher
// Crear constraints y indices
CREATE CONSTRAINT game_name_unique IF NOT EXISTS
FOR (g:Juego) REQUIRE g.nombre IS UNIQUE;

CREATE CONSTRAINT user_id_unique IF NOT EXISTS
FOR (u:Usuario) REQUIRE u.id IS UNIQUE;

CREATE CONSTRAINT emotion_type_unique IF NOT EXISTS
FOR (e:Emocion) REQUIRE e.tipo IS UNIQUE;

CREATE INDEX resonance_intensity IF NOT EXISTS
FOR ()-[r:RESUENA_CON]-() ON (r.intensidad);

// Crear emociones base
MERGE (e1:Emocion {tipo: "alegre"});
MERGE (e2:Emocion {tipo: "relajante"});
MERGE (e3:Emocion {tipo: "melancólico"});
MERGE (e4:Emocion {tipo: "exploración"});
MERGE (e5:Emocion {tipo: "desafiante"});
MERGE (e6:Emocion {tipo: "contemplativo"});
MERGE (e7:Emocion {tipo: "social"});
MERGE (e8:Emocion {tipo: "competitivo"});
MERGE (e9:Emocion {tipo: "creativo"});
```

#### 3.3 Cargar Datos Iniciales (COPIAR Y PEGAR)
En Neo4j Browser, copiar y pegar este código:

```cypher
// Crear juegos con sus propiedades emocionales
CREATE (j1:Juego {id: "stardew-valley", nombre: "Stardew Valley", descripcion: "Simulador de granja relajante con elementos sociales"});
CREATE (j2:Juego {id: "dark-souls", nombre: "Dark Souls", descripcion: "RPG extremadamente desafiante"});
CREATE (j3:Juego {id: "minecraft", nombre: "Minecraft", descripcion: "Juego de construcción y exploración creativo"});
CREATE (j4:Juego {id: "among-us", nombre: "Among Us", descripcion: "Juego social de deducción"});
CREATE (j5:Juego {id: "journey", nombre: "Journey", descripcion: "Experiencia contemplativa y artística"});
CREATE (j6:Juego {id: "animal-crossing", nombre: "Animal Crossing: New Horizons", descripcion: "Simulador de vida social relajante"});
CREATE (j7:Juego {id: "elden-ring", nombre: "Elden Ring", descripcion: "RPG de mundo abierto desafiante"});
CREATE (j8:Juego {id: "fall-guys", nombre: "Fall Guys", descripcion: "Juego competitivo divertido y colorido"});
CREATE (j9:Juego {id: "satisfactory", nombre: "Satisfactory", descripcion: "Juego de construcción y automatización"});
CREATE (j10:Juego {id: "cuphead", nombre: "Cuphead", descripcion: "Plataformas extremadamente desafiante con arte único"});

// Conectar juegos con emociones
MATCH (j1:Juego {id: "stardew-valley"}), (e:Emocion {tipo: "relajante"}) CREATE (j1)-[:RESUENA_CON {intensidad: 0.9}]->(e);
MATCH (j1:Juego {id: "stardew-valley"}), (e:Emocion {tipo: "social"}) CREATE (j1)-[:RESUENA_CON {intensidad: 0.7}]->(e);

MATCH (j2:Juego {id: "dark-souls"}), (e:Emocion {tipo: "desafiante"}) CREATE (j2)-[:RESUENA_CON {intensidad: 0.95}]->(e);

MATCH (j3:Juego {id: "minecraft"}), (e:Emocion {tipo: "creativo"}) CREATE (j3)-[:RESUENA_CON {intensidad: 0.9}]->(e);
MATCH (j3:Juego {id: "minecraft"}), (e:Emocion {tipo: "exploración"}) CREATE (j3)-[:RESUENA_CON {intensidad: 0.8}]->(e);

MATCH (j4:Juego {id: "among-us"}), (e:Emocion {tipo: "social"}) CREATE (j4)-[:RESUENA_CON {intensidad: 0.9}]->(e);

MATCH (j5:Juego {id: "journey"}), (e:Emocion {tipo: "contemplativo"}) CREATE (j5)-[:RESUENA_CON {intensidad: 0.9}]->(e);
MATCH (j5:Juego {id: "journey"}), (e:Emocion {tipo: "melancólico"}) CREATE (j5)-[:RESUENA_CON {intensidad: 0.7}]->(e);

MATCH (j6:Juego {id: "animal-crossing"}), (e:Emocion {tipo: "relajante"}) CREATE (j6)-[:RESUENA_CON {intensidad: 0.8}]->(e);
MATCH (j6:Juego {id: "animal-crossing"}), (e:Emocion {tipo: "social"}) CREATE (j6)-[:RESUENA_CON {intensidad: 0.8}]->(e);

MATCH (j7:Juego {id: "elden-ring"}), (e:Emocion {tipo: "desafiante"}) CREATE (j7)-[:RESUENA_CON {intensidad: 0.9}]->(e);
MATCH (j7:Juego {id: "elden-ring"}), (e:Emocion {tipo: "exploración"}) CREATE (j7)-[:RESUENA_CON {intensidad: 0.8}]->(e);

MATCH (j8:Juego {id: "fall-guys"}), (e:Emocion {tipo: "alegre"}) CREATE (j8)-[:RESUENA_CON {intensidad: 0.9}]->(e);
MATCH (j8:Juego {id: "fall-guys"}), (e:Emocion {tipo: "competitivo"}) CREATE (j8)-[:RESUENA_CON {intensidad: 0.7}]->(e);

MATCH (j9:Juego {id: "satisfactory"}), (e:Emocion {tipo: "creativo"}) CREATE (j9)-[:RESUENA_CON {intensidad: 0.85}]->(e);

MATCH (j10:Juego {id: "cuphead"}), (e:Emocion {tipo: "desafiante"}) CREATE (j10)-[:RESUENA_CON {intensidad: 0.9}]->(e);
```

#### 3.4 Verificar que Funcionó
Ejecutar esta consulta en Neo4j Browser:
```cypher
MATCH (j:Juego) RETURN count(j) as total_juegos;
```
**Debe devolver: total_juegos: 10**

### PASO 4: Configurar y Ejecutar el Backend

#### 4.1 Abrir Nueva Terminal
```bash
# Navegar al directorio backend
cd backend

# Instalar dependencias y compilar
mvn clean install

# Si da error, intentar:
mvn clean install -DskipTests
```

#### 4.2 Ejecutar Backend
```bash
mvn spring-boot:run
```

**Deberías ver logs como:**
```
Started GameSoulApplication in X.XXX seconds
```

#### 4.3 Verificar Backend
Abrir en navegador: **http://localhost:8080/api/test/hello**

**Debe mostrar:**
```json
{
  "message": "Game Soul Backend funcionando correctamente!",
  "status": "success"
}
```

### PASO 5: Configurar y Ejecutar el Frontend

#### 5.1 Abrir OTRA Terminal Nueva (mantener backend corriendo)
```bash
# Navegar al directorio frontend
cd frontend

# Instalar dependencias
npm install
```

#### 5.2 Ejecutar Frontend
```bash
npm run dev
```

**Deberías ver:**
```
Local:   http://localhost:3000/
```

#### 5.3 Verificar Frontend
Abrir en navegador: **http://localhost:3000**

**Debe mostrar:** La página de bienvenida de Game Soul con diseño azul/morado.

---

## Probar el Sistema Completo

### Flujo de Prueba
1. **Ir a** http://localhost:3000
2. **Hacer clic** en "Descubre tu juego ideal"
3. **Ingresar tu nombre** (ej: "Ana")
4. **Responder el cuestionario** de 5 preguntas
5. **Ver recomendaciones** emocionales
6. **Dar like** a algún juego (botón verde "Me gusta")
7. **Esperar 3-5 segundos**
8. **Debe aparecer** sección "Usuarios Como Tú También Jugaron"

### Qué Esperar
- **Recomendaciones Emocionales**: Basadas en tus respuestas del cuestionario
- **Recomendaciones Sociales**: "Usuarios como tú también jugaron..." (aparece después del feedback)
- **Sistema Adaptativo**: Cada like/dislike mejora futuras recomendaciones

---

## URLs de Verificación

Durante el funcionamiento, estas URLs deben funcionar:

| Servicio | URL | Qué Debe Mostrar |
|----------|-----|------------------|
| **Frontend** | http://localhost:3000 | Página de Game Soul |
| **Backend Health** | http://localhost:8080/api/test/hello | JSON con mensaje de éxito |
| **Neo4j Browser** | http://localhost:7474 | Interfaz de base de datos |
| **API Recomendaciones** | http://localhost:8080/api/recommendations/emotion/relajante | JSON con juegos relajantes |

---

## Solución de Problemas

### Error: "Docker no reconocido"
- **Solución**: Instalar Docker Desktop y reiniciar computadora
- **Verificar**: `docker --version`

### Error: "Java no reconocido" 
- **Solución**: Instalar Java 17+ y configurar JAVA_HOME
- **Verificar**: `java -version`

### Error: "No se puede conectar a Neo4j"
- **Solución**: 
  ```bash
  docker-compose down
  docker-compose up -d neo4j
  # Esperar 60 segundos
  ```

### Error: "Backend falla al iniciar"
- **Verificar Neo4j**: http://localhost:7474 debe cargar
- **Verificar puerto**: Solo una instancia del backend debe correr
- **Limpiar**: `mvn clean install`

### Error: "Frontend no carga"
- **Verificar Node.js**: `node -version` debe ser 18+
- **Limpiar cache**:
  ```bash
  rm -rf node_modules
  rm package-lock.json
  npm install
  ```

### Error: "No aparecen recomendaciones sociales"
- **Dar like** a al menos un juego
- **Esperar 5 segundos**
- **Refrescar página** si es necesario
- **Verificar logs** del backend en terminal

### Backend se cierra solo
- **Verificar puertos**: 8080 debe estar libre
- **Ver logs completos** para identificar error específico

---

## Arquitectura del Sistema

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │    Backend      │    │   Base de       │
│   (React)       │◄──►│  (Spring Boot)  │◄──►│   Datos         │
│   Port: 3000    │    │   Port: 8080    │    │   (Neo4j)       │
│                 │    │                 │    │   Port: 7474    │
│ • Cuestionario  │    │ • API REST      │    │ • Grafos        │
│ • Resultados    │    │ • Algoritmos    │    │ • Relaciones    │
│ • Feedback      │    │ • Recomendaciones│   │ • Usuarios      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

---

## Funcionalidades del Sistema

### Recomendaciones Emocionales
- Análisis de 5 preguntas sobre estado emocional
- Mapeo a 9 emociones base (alegre, relajante, desafiante, etc.)
- Algoritmo de resonancia emocional con juegos

### Sistema Social Inteligente
- Identificación automática de usuarios similares
- Recomendaciones basadas en gustos de la comunidad
- Aprendizaje continuo con cada feedback

### Base de Datos en Grafos
- Relaciones complejas entre usuarios, juegos y emociones
- Consultas optimizadas para recomendaciones en tiempo real
- Escalabilidad para miles de usuarios

---

## Parar el Sistema

Para detener todos los servicios:

1. **Frontend**: Ctrl+C en terminal del frontend
2. **Backend**: Ctrl+C en terminal del backend  
3. **Neo4j**: 
   ```bash
   docker-compose down
   ```

---

## Datos del Sistema

### Juegos Incluidos
- **Relajantes**: Stardew Valley, Animal Crossing
- **Desafiantes**: Dark Souls, Elden Ring, Cuphead
- **Creativos**: Minecraft, Satisfactory
- **Sociales**: Among Us, Animal Crossing
- **Contemplativos**: Journey

### Emociones Mapeadas
- alegre, relajante, melancólico, exploración
- desafiante, contemplativo, social, competitivo, creativo

---

## Contacto

**Equipo de Desarrollo:**
- **Ismalej** - Backend y Base de Datos
- **Adrian** - Frontend y UX  
- **Fatima** - Integración y Testing

**Universidad del Valle de Guatemala**  
CC2016 - Algoritmos y Estructura de Datos

---

## Notas Importantes

- **Primera vez**: El sistema tarda ~2 minutos en estar completamente funcional
- **Recomendaciones sociales**: Aparecen después de dar feedback (like/dislike)
- **Datos persistentes**: Los datos se guardan mientras Docker esté corriendo
- **Puertos**: 3000 (frontend), 8080 (backend), 7474 (Neo4j) deben estar libres

**El sistema está listo cuando:**
✅ Frontend carga en http://localhost:3000  
✅ Backend responde en http://localhost:8080/api/test/hello  
✅ Neo4j funciona en http://localhost:7474  
✅ Puedes completar un cuestionario y ver recomendaciones
