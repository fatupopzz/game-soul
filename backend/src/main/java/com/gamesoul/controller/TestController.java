package com.gamesoul.controller;

import java.util.Map;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/test")
public class TestController {
    
    @GetMapping("/hello")
    public Map<String, String> hello() {
        return Map.of(
            "message", "🎮 Game Soul Backend funcionando correctamente!",
            "status", "success",
            "timestamp", java.time.LocalDateTime.now().toString(),
            "version", "1.0.0",
            "endpoints", "Todos los endpoints disponibles en /api/"
        );
    }
    
    @GetMapping("/health")
    public Map<String, Object> health() {
        return Map.of(
            "status", "UP",
            "backend", "Game Soul",
            "database", "Neo4j Ready",
            "timestamp", java.time.LocalDateTime.now().toString(),
            "social_system", "Active"
        );
    }
}