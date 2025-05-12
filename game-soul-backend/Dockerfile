# Dockerfile para Game Soul Backend (Rust)

# Primera etapa: Compilación
FROM rust:1.70-slim-bullseye as builder

# Establecer directorio de trabajo
WORKDIR /app

# Instalar dependencias del sistema necesarias para compilar
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copiar archivos de manifiesto de Cargo
COPY Cargo.toml Cargo.lock* ./

# Crear una estructura de directorios mínima para aprovechar la caché de capas de Docker
RUN mkdir -p src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar el código fuente real
COPY src/ src/

# Forzar la recompilación con el código real
RUN touch src/main.rs && \
    cargo build --release

# Segunda etapa: Ejecutable final
FROM debian:bullseye-slim

# Instalar dependencias de tiempo de ejecución
RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Crear un usuario no privilegiado para ejecutar la aplicación
RUN groupadd -r gameuser && useradd -r -g gameuser gameuser

WORKDIR /app

# Copiar el ejecutable compilado desde la etapa de construcción
COPY --from=builder /app/target/release/game-soul-backend .

# Copiar archivos de configuración
COPY .env.example .env

# Cambiar la propiedad de los archivos al usuario no privilegiado
RUN chown -R gameuser:gameuser /app

# Cambiar al usuario no privilegiado
USER gameuser

# Exponer el puerto en el que se ejecuta la aplicación
EXPOSE 3001

# Comando para ejecutar la aplicación
CMD ["./game-soul-backend"]