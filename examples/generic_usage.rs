use nocheat::types::{AnalysisResponse, DefaultPlayerData, PlayerResult, PlayerStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a custom data structure for player statistics
#[derive(Clone, Debug, Deserialize, Serialize)]
struct CustomPlayerData {
    // Game-specific statistics that differ from the default structure
    accuracy: f32,
    reaction_time_ms: Vec<u32>,
    movement_patterns: HashMap<String, u32>,
    mouse_acceleration: Option<f32>,
}

// Define a custom analysis result structure
#[derive(Debug, PartialEq, Serialize)]
struct CustomAnalysisResult {
    cheating_probability: f32,
    abnormal_patterns: Vec<String>,
    confidence_score: f32,
    recommended_action: String,
}

fn main() {
    // Example 1: Using the default data structure (backward compatible)
    let mut shots = HashMap::new();
    shots.insert("rifle".to_string(), 100);

    let mut hits = HashMap::new();
    hits.insert("rifle".to_string(), 50);

    let default_data = DefaultPlayerData {
        shots_fired: shots,
        hits: hits,
        headshots: 10,
        shot_timestamps_ms: None,
        training_label: None,
    };

    // Create a PlayerStats instance with the default data structure
    let player_stats = PlayerStats::new("player123".to_string(), default_data);

    println!(
        "Default PlayerStats: {}, headshots: {}",
        player_stats.player_id, player_stats.data.headshots
    );

    // Example 2: Using a custom data structure
    let mut movement = HashMap::new();
    movement.insert("jumps".to_string(), 50);
    movement.insert("crouches".to_string(), 30);

    let custom_data = CustomPlayerData {
        accuracy: 0.75,
        reaction_time_ms: vec![250, 220, 230, 210, 240],
        movement_patterns: movement,
        mouse_acceleration: Some(1.5),
    };

    // Create a PlayerStats instance with custom data
    let custom_stats = PlayerStats::new("custom_player".to_string(), custom_data);

    println!(
        "Custom PlayerStats: {}, accuracy: {}",
        custom_stats.player_id, custom_stats.data.accuracy
    );

    // Example 3: Creating custom analysis results
    let custom_result = CustomAnalysisResult {
        cheating_probability: 0.85,
        abnormal_patterns: vec!["AimSnap".to_string(), "RecoilControl".to_string()],
        confidence_score: 0.92,
        recommended_action: "Review gameplay footage".to_string(),
    };

    let player_result = PlayerResult::new("custom_player".to_string(), custom_result);

    println!(
        "Custom analysis result - Player: {}, Cheating probability: {}, Recommended action: {}",
        player_result.player_id,
        player_result.data.cheating_probability,
        player_result.data.recommended_action
    );

    // Example 4: Creating an AnalysisResponse with custom results
    let response = AnalysisResponse {
        results: vec![player_result],
    };

    println!(
        "Analysis response contains {} result(s)",
        response.results.len()
    );

    // Example 5: Serializing to JSON
    let json = serde_json::to_string_pretty(&response).unwrap();
    println!("JSON output:\n{}", json);
}
