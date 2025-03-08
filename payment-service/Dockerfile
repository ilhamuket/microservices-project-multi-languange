FROM openjdk:17-jdk-slim

WORKDIR /app

# Copy Maven wrapper and pom.xml
COPY .mvn/ .mvn/
COPY mvnw pom.xml ./

# Build all dependencies
RUN ./mvnw dependency:go-offline -B

# Copy source code
COPY src/ src/

# Build app
RUN ./mvnw package -DskipTests

# Expose port
EXPOSE 8080

# Run the application
CMD ["java", "-jar", "target/payment-service-0.0.1-SNAPSHOT.jar"]