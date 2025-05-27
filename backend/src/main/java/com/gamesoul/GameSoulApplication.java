package com.gamesoul;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class GameSoulApplication {
    
    public static void main(String[] args) {
        System.out.println("🚀 Iniciando Game Soul Backend...");
        SpringApplication.run(GameSoulApplication.class, args);
        System.out.println("✅ Game Soul Backend iniciado correctamente en http://localhost:8080/api");
    }
}