package com.gamesoul.model.dto;

import java.util.Map;

public class UserProfile {
    private String userId;
    private String dominantEmotion;
    private String timePreference;
    private Map<String, Double> emotionWeights;
    
    // Constructores
    public UserProfile() {}
    
    public UserProfile(String userId, String dominantEmotion, String timePreference, Map<String, Double> emotionWeights) {
        this.userId = userId;
        this.dominantEmotion = dominantEmotion;
        this.timePreference = timePreference;
        this.emotionWeights = emotionWeights;
    }
    
    // Getters y Setters
    public String getUserId() { return userId; }
    public void setUserId(String userId) { this.userId = userId; }
    
    public String getDominantEmotion() { return dominantEmotion; }
    public void setDominantEmotion(String dominantEmotion) { this.dominantEmotion = dominantEmotion; }
    
    public String getTimePreference() { return timePreference; }
    public void setTimePreference(String timePreference) { this.timePreference = timePreference; }
    
    public Map<String, Double> getEmotionWeights() { return emotionWeights; }
    public void setEmotionWeights(Map<String, Double> emotionWeights) { this.emotionWeights = emotionWeights; }
}
