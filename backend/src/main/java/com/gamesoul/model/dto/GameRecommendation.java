package com.gamesoul.model.dto;

import java.util.List;

public class GameRecommendation {
    private String id;
    private String name;
    private String description;
    private double matchScore;
    private List<String> reasons;
    private List<String> genres;
    private List<String> characteristics;
    
    // Constructores
    public GameRecommendation() {}
    
    public GameRecommendation(String id, String name, String description, double matchScore) {
        this.id = id;
        this.name = name;
        this.description = description;
        this.matchScore = matchScore;
    }
    
    // Getters y Setters
    public String getId() { return id; }
    public void setId(String id) { this.id = id; }
    
    public String getName() { return name; }
    public void setName(String name) { this.name = name; }
    
    public String getDescription() { return description; }
    public void setDescription(String description) { this.description = description; }
    
    public double getMatchScore() { return matchScore; }
    public void setMatchScore(double matchScore) { this.matchScore = matchScore; }
    
    public List<String> getReasons() { return reasons; }
    public void setReasons(List<String> reasons) { this.reasons = reasons; }
    
    public List<String> getGenres() { return genres; }
    public void setGenres(List<String> genres) { this.genres = genres; }
    
    public List<String> getCharacteristics() { return characteristics; }
    public void setCharacteristics(List<String> characteristics) { this.characteristics = characteristics; }
}
