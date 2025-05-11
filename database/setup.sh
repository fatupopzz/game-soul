# !/bin/bash

echo "🚀 Configurando base de datos Game Soul..."

# Esperar a que Neo4j esté disponible...
echo "Esperando a que Neo4j esté disponible..."
sleep 10

# Variables
NEO4J_URI="bolt://localhost:7687"
NEO4J_USER="neo4j"
NEO4J_PASSWORD="password"

# Obtener directorio actual donde está el script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Instalar cypher-shell si no está instalado
if ! command -v cypher-shell &> /dev/null; then
    echo "❌ cypher-shell no está instalado. Se usará otro método."
    # Alternativa: usar curl para enviar consultas a Neo4j HTTP API
    exit 1
fi

# Función para ejecutar cypher
execute_cypher() {
    echo "Ejecutando $1..."
    if [ -f "$1" ]; then
        cypher-shell -a $NEO4J_URI -u $NEO4J_USER -p $NEO4J_PASSWORD -f "$1"
    else
        echo "❌ Error: El archivo $1 no existe"
    fi
}

# Ejecutar archivos cypher
execute_cypher "$SCRIPT_DIR/schema/constraints.cypher"
execute_cypher "$SCRIPT_DIR/data/initial-data.cypher"

echo "✅ Base de datos configurada con éxito!"