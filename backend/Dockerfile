FROM openjdk:17-jdk-slim

WORKDIR /app

# Instalar Maven
RUN apt-get update && apt-get install -y maven

# Copiar archivos del proyecto
COPY pom.xml .
COPY src ./src

# Construir la aplicación
RUN mvn clean package -DskipTests

# Exponer puerto
EXPOSE 8080

# Ejecutar la aplicación
CMD ["java", "-jar", "target/game-soul-backend-1.0.0.jar"]
