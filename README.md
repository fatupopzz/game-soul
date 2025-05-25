# Game Soul - Sistema de Recomendaciones Emocional

> *Conectando emociones con experiencias de juego*

Un sistema inteligente que recomienda videojuegos basándose en tu estado emocional actual, utilizando grafos de conocimiento y algoritmos de resonancia emocional.

![Java](https://img.shields.io/badge/Java-17-orange?style=flat&logo=openjdk)
![Spring Boot](https://img.shields.io/badge/Spring%20Boot-3.2.0-green?style=flat&logo=spring)
![React](https://img.shields.io/badge/React-19.1-blue?style=flat&logo=react)
![Neo4j](https://img.shields.io/badge/Neo4j-Latest-red?style=flat&logo=neo4j)

---

## Requisitos del Sistema

### Software Necesario
- **Java 17 o superior**
- **Maven 3.6 o superior**
- **Node.js 18 o superior**
- **Docker Desktop**
- **Git**

### Verificar Instalaciones
```bash
# Verificar Java
java -version

# Verificar Maven
mvn -version

# Verificar Node.js
node -version
npm -version

# Verificar Docker
docker --version
docker-compose --version
```

---

## Instalación Paso a Paso

### 1. Clonar el Repositorio
```bash
git clone https://github.com/tu-usuario/game-soul.git
cd game-soul
```

### 2. Configurar Neo4j
```bash
# Levantar Neo4j con Docker
docker-compose up -d neo4j

# Verificar que esté corriendo
docker ps
```

**Importante**: Esperar 30-60 segundos para que Neo4j termine de inicializarse completamente.

### 3. Configurar la Base de Datos

#### Acceder a Neo4j Browser
1. Abrir en navegador: http://localhost:7474
2. Conectar con las siguientes credenciales:
   - **URI**: bolt://localhost:7687
   - **Usuario**: neo4j
   - **Contraseña**: password

#### Crear Esquema y Datos Iniciales
1. **Ejecutar constraints y esquema**:
   - Abrir archivo `database/schema/constraints.cypher`
   - Copiar todo el contenido
   - Pegar en Neo4j Browser y ejecutar

2. **Cargar datos iniciales**:
   - Abrir archivo `database/data/initial-data.cypher`
   - Copiar todo el contenido
   - Pegar en Neo4j Browser y ejecutar

#### Verificar que los datos se cargaron
```cypher
MATCH (n) RETURN count(n) as total_nodes
```
Debería mostrar más de 50 nodos.

### 4. Configurar y Ejecutar el Backend

```bash
# Navegar al directorio backend
cd backend

# Instalar dependencias y compilar
mvn clean install

# Ejecutar la aplicación
mvn spring-boot:run
```

#### Verificar que el Backend funciona
Abrir en navegador: http://localhost:8080/api/test/hello

Debería mostrar un mensaje JSON de confirmación.

### 5. Configurar y Ejecutar el Frontend

**Abrir nueva terminal** (mantener el backend corriendo)

```bash
# Navegar al directorio frontend
cd frontend

# Instalar dependencias
npm install

# Ejecutar en modo desarrollo
npm run dev
```

#### Verificar que el Frontend funciona
Abrir en navegador: http://localhost:3000

Debería mostrar la página de bienvenida de Game Soul.

---

## Verificación del Sistema Completo

### Prueba Funcional Completa
1. **Landing Page**: Debería cargar con diseño azul/morado
2. **Botón "Descubre tu juego ideal"**: Navegar a registro
3. **Ingresar nombre**: Cualquier nombre, continuar
4. **Cuestionario**: Responder las 5 preguntas
5. **Resultados**: Ver recomendaciones con scores
6. **Feedback**: Probar likes/dislikes en juegos

### URLs de Verificación
- **Frontend**: http://localhost:3000
- **Backend Health**: http://localhost:8080/api/test/hello
- **Neo4j Browser**: http://localhost:7474

---

## Solución de Problemas Comunes

### Neo4j no se conecta
```bash
# Verificar que Docker esté corriendo
docker ps

# Si no aparece neo4j, reiniciar
docker-compose down
docker-compose up -d neo4j

# Esperar y verificar logs
docker-compose logs neo4j
```

### Backend falla al iniciar
1. **Verificar Java version**:
   ```bash
   java -version
   ```
   Debe ser 17 o superior.

2. **Verificar que Neo4j esté disponible**:
   - Abrir http://localhost:7474
   - Debe cargar la interfaz de Neo4j

3. **Limpiar y recompilar**:
   ```bash
   cd backend
   mvn clean
   mvn install
   mvn spring-boot:run
   ```

### Frontend no carga
1. **Verificar Node.js version**:
   ```bash
   node -version
   ```
   Debe ser 18 o superior.

2. **Limpiar cache e instalar**:
   ```bash
   cd frontend
   rm -rf node_modules
   rm package-lock.json
   npm install
   npm run dev
   ```

### Error de CORS
Si el frontend no puede conectar con el backend:
1. Verificar que el backend esté en puerto 8080
2. Verificar que el frontend esté en puerto 3000
3. Reiniciar ambos servicios

### Base de datos vacía
Si no aparecen recomendaciones:
1. Verificar en Neo4j Browser:
   ```cypher
   MATCH (j:Juego) RETURN count(j)
   ```
2. Si devuelve 0, re-ejecutar `initial-data.cypher`

---

## Estructura del Proyecto

```
game-soul/
├── backend/
│   ├── src/main/java/com/gamesoul/
│   │   ├── GameSoulApplication.java
│   │   ├── controller/
│   │   ├── service/
│   │   └── model/
│   ├── src/main/resources/
│   │   └── application.yml
│   └── pom.xml
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   ├── App.jsx
│   │   └── main.jsx
│   ├── public/
│   └── package.json
├── database/
│   ├── schema/constraints.cypher
│   ├── data/initial-data.cypher
│   └── queries/
└── docker-compose.yml
```

---

## Configuración de Puertos

| Servicio | Puerto | URL |
|----------|---------|-----|
| Frontend | 3000 | http://localhost:3000 |
| Backend API | 8080 | http://localhost:8080/api |
| Neo4j Browser | 7474 | http://localhost:7474 |
| Neo4j Bolt | 7687 | bolt://localhost:7687 |

---

## Comandos Útiles

### Docker
```bash
# Ver servicios corriendo
docker ps

# Ver logs de Neo4j
docker-compose logs neo4j

# Reiniciar solo Neo4j
docker-compose restart neo4j

# Parar todos los servicios
docker-compose down
```

### Maven (Backend)
```bash
# Compilar sin ejecutar tests
mvn clean install -DskipTests

# Ejecutar solo tests
mvn test

# Ejecutar aplicación
mvn spring-boot:run
```

### NPM (Frontend)
```bash
# Instalar dependencias
npm install

# Ejecutar en desarrollo
npm run dev

# Construir para producción
npm run build
```

---

## API Endpoints Principales

### Cuestionario
```http
POST /api/questionnaire
Content-Type: application/json

{
  "user_id": "usuario123",
  "answers": {
    "tipo_experiencia": "relajante",
    "estado_animo": "tranquilo",
    "actividad_preferida": "construir",
    "tiempo_disponible": "medio",
    "meta_emocional": "calma"
  }
}
```

### Obtener Recomendaciones
```http
GET /api/recommendations/{userId}
```

### Enviar Feedback
```http
POST /api/feedback
Content-Type: application/json

{
  "userId": "usuario123",
  "gameId": "stardew-valley",
  "liked": true,
  "rating": 4
}
```

---

## Parar el Sistema

Para detener todos los servicios:

1. **Parar Frontend**: Ctrl+C en la terminal del frontend
2. **Parar Backend**: Ctrl+C en la terminal del backend
3. **Parar Neo4j**:
   ```bash
   docker-compose down
   ```

---


**Equipo de Desarrollo:**
- Ismalej - Backend y Base de Datos
- Adrian - Frontend y UX
- Fatima - Integración y Testing

---

**Nota**: Este README está enfocado en hacer funcionar el sistema. Para documentación técnica detallada sobre el algoritmo y arquitectura, consultar los archivos en `
