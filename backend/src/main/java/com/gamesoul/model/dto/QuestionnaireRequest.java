package com.gamesoul.model.dto;

import java.util.Map;
import java.util.List;
import com.fasterxml.jackson.annotation.JsonProperty;

public class QuestionnaireRequest {
    @JsonProperty("user_id")
    private String userId;
    
    private Map<String, String> answers;
    
    private List<String> dealbreakers;
    
    // Constructores
    public QuestionnaireRequest() {}
    
    public QuestionnaireRequest(String userId, Map<String, String> answers) {
        this.userId = userId;
        this.answers = answers;
    }
    
    // Getters y Setters
    public String getUserId() { return userId; }
    public void setUserId(String userId) { this.userId = userId; }
    
    public Map<String, String> getAnswers() { return answers; }
    public void setAnswers(Map<String, String> answers) { this.answers = answers; }
    
    public List<String> getDealbreakers() { return dealbreakers; }
    public void setDealbreakers(List<String> dealbreakers) { this.dealbreakers = dealbreakers; }
}
