# Dockerfile para Game Soul Backend en rust 

# Primera etapa: Compilación
FROM rust:latest as builder

# Establecer directorio de trabajo
WORKDIR /app

# Instalar dependencias del sistema necesarias para compilar
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copiar todo el código fuente
COPY . .

# Compilar el proyecto
RUN cargo build --release

# Segunda etapa: Ejecutable final
FROM ubuntu:22.04

# Instalar dependencias de tiempo de ejecución
RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Crear un usuario no privilegiado para ejecutar la aplicación
RUN groupadd -r gameuser && useradd -r -g gameuser gameuser

WORKDIR /app

# Copiar el ejecutable compilado desde la etapa de construcción
COPY --from=builder /app/target/release/game-soul-backend .

# Crear un archivo .env básico
RUN echo "HOST=0.0.0.0\n\
PORT=3001\n\
ENVIRONMENT=development\n\
NEO4J_URI=bolt://neo4j:7687\n\
NEO4J_USER=neo4j\n\
NEO4J_PASSWORD=password\n\
JWT_SECRET=development_jwt_secret_key\n\
JWT_EXPIRATION=86400\n\
FRONTEND_URL=http://localhost:3000" > .env

# Cambiar la propiedad de los archivos al usuario no privilegiado
RUN chown -R gameuser:gameuser /app

# Cambiar al usuario no privilegiado
USER gameuser

# Exponer el puerto en el que se ejecuta la aplicación
EXPOSE 3001

# Comando para ejecutar la aplicación
CMD ["./game-soul-backend"]