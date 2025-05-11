@echo off
echo 🚀 Configurando base de datos Game Soul...

REM Esperar a que Neo4j esté disponible...
echo Esperando a que Neo4j esté disponible...
timeout /t 10

REM Variables
set NEO4J_URI=bolt://localhost:7687
set NEO4J_USER=neo4j
set NEO4J_PASSWORD=password

REM Obtener directorio actual
set SCRIPT_DIR=%~dp0

REM Comprobar si cypher-shell está en el PATH
where cypher-shell >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo ❌ cypher-shell no está instalado o no está en el PATH.
    echo Buscando instalación de Neo4j...
    
    REM Verificar ubicaciones comunes
    if exist "C:\Program Files\Neo4j\bin\cypher-shell.bat" (
        set CYPHER_SHELL="C:\Program Files\Neo4j\bin\cypher-shell.bat"
        echo Encontrado en: !CYPHER_SHELL!
        goto :FOUND_CYPHER
    )
    
    if exist "C:\Program Files (x86)\Neo4j\bin\cypher-shell.bat" (
        set CYPHER_SHELL="C:\Program Files (x86)\Neo4j\bin\cypher-shell.bat"
        echo Encontrado en: !CYPHER_SHELL!
        goto :FOUND_CYPHER
    )
    
    echo ❌ No se pudo encontrar cypher-shell.
    echo Por favor, asegúrate de que Neo4j está instalado correctamente.
    exit /b 1
) else (
    set CYPHER_SHELL=cypher-shell
    goto :FOUND_CYPHER
)

:FOUND_CYPHER
REM Ejecutar archivos cypher
echo Ejecutando %SCRIPT_DIR%schema\constraints.cypher...
if exist "%SCRIPT_DIR%schema\constraints.cypher" (
    %CYPHER_SHELL% -a %NEO4J_URI% -u %NEO4J_USER% -p %NEO4J_PASSWORD% -f "%SCRIPT_DIR%schema\constraints.cypher"
) else (
    echo ❌ Error: El archivo %SCRIPT_DIR%schema\constraints.cypher no existe
)

echo Ejecutando %SCRIPT_DIR%data\initial-data.cypher...
if exist "%SCRIPT_DIR%data\initial-data.cypher" (
    %CYPHER_SHELL% -a %NEO4J_URI% -u %NEO4J_USER% -p %NEO4J_PASSWORD% -f "%SCRIPT_DIR%data\initial-data.cypher"
) else (
    echo ❌ Error: El archivo %SCRIPT_DIR%data\initial-data.cypher no existe
)

echo ✅ Base de datos configurada con éxito!