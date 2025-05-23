package com.gamesoul.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
@RequestMapping("/test")
public class TestController {
    
    @GetMapping("/hello")
    public Map<String, String> hello() {
        return Map.of(
            "message", "🎮 Game Soul Backend funcionando correctamente!",
            "status", "success",
            "timestamp", java.time.LocalDateTime.now().toString()
        );
    }
}
