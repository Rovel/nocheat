// Example of using generic types with custom analysis logic
use nocheat::types::{AnalysisResponse, Analyzable, DefaultPlayerData, PlayerResult, PlayerStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a custom data structure specifically for a first-person shooter game
#[derive(Clone, Debug, Deserialize, Serialize)]
struct FpsPlayerData {
    // Basic statistics
    kills: u32,
    deaths: u32,
    assists: u32,

    // Weapon-specific stats
    weapon_stats: HashMap<String, WeaponStats>,

    // Movement and positioning data
    average_position_change: f32, // How much the player moves
    camping_seconds: u32,         // Time spent in one location

    // Round information
    round_duration_seconds: u32,
    team: String, // "CT" or "T" for counter-strike like games
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct WeaponStats {
    shots_fired: u32,
    hits: u32,
    headshots: u32,
    distance_meters: Vec<f32>, // Distance for each kill
}

// Define custom analysis results
#[derive(Debug, PartialEq, Serialize)]
struct FpsAnalysisResult {
    cheating_probability: f32,
    suspected_cheats: Vec<String>,
    evidence_strength: String, // "Low", "Medium", "High"
    anomaly_details: HashMap<String, String>,
}

// Implement the Analyzable trait for our custom data
impl Analyzable for FpsPlayerData {
    fn calculate_accuracy_rate(&self) -> f32 {
        let mut total_shots = 0;
        let mut total_hits = 0;

        for weapon in self.weapon_stats.values() {
            total_shots += weapon.shots_fired;
            total_hits += weapon.hits;
        }

        if total_shots == 0 {
            return 0.0;
        }

        total_hits as f32 / total_shots as f32
    }

    fn calculate_headshot_ratio(&self) -> f32 {
        let mut total_hits = 0;
        let mut total_headshots = 0;

        for weapon in self.weapon_stats.values() {
            total_hits += weapon.hits;
            total_headshots += weapon.headshots;
        }

        if total_hits == 0 {
            return 0.0;
        }

        total_headshots as f32 / total_hits as f32
    }

    fn extract_features(&self) -> Vec<f32> {
        let kd_ratio = if self.deaths == 0 {
            self.kills as f32
        } else {
            self.kills as f32 / self.deaths as f32
        };

        vec![
            self.calculate_accuracy_rate(),
            self.calculate_headshot_ratio(),
            kd_ratio,
            self.average_position_change,
            self.camping_seconds as f32 / self.round_duration_seconds as f32,
        ]
    }

    fn is_suspicious(&self) -> bool {
        let accuracy = self.calculate_accuracy_rate();
        let headshot_ratio = self.calculate_headshot_ratio();
        let kd_ratio = if self.deaths == 0 {
            self.kills as f32
        } else {
            self.kills as f32 / self.deaths as f32
        };

        // Check for suspicious patterns
        accuracy > 0.8 || headshot_ratio > 0.7 || kd_ratio > 5.0
    }
}

// Custom analyzer function for FPS games
fn analyze_fps_players(
    players: Vec<PlayerStats<FpsPlayerData>>,
) -> AnalysisResponse<FpsAnalysisResult> {
    let mut results = Vec::new();

    for player in players {
        let mut suspected_cheats = Vec::new();
        let mut anomaly_details = HashMap::new();

        // Check accuracy (aimbot detection)
        let accuracy = player.data.calculate_accuracy_rate();
        if accuracy > 0.8 {
            suspected_cheats.push("Aimbot".to_string());
            anomaly_details.insert(
                "High Accuracy".to_string(),
                format!("{:.1}% hit rate is suspiciously high", accuracy * 100.0),
            );
        }

        // Check headshot ratio (aimbot detection)
        let headshot_ratio = player.data.calculate_headshot_ratio();
        if headshot_ratio > 0.7 {
            suspected_cheats.push("Aimbot (Headshot)".to_string());
            anomaly_details.insert(
                "Headshot Anomaly".to_string(),
                format!(
                    "{:.1}% headshot ratio is suspiciously high",
                    headshot_ratio * 100.0
                ),
            );
        }

        // Check KD ratio (general skill anomaly)
        let kd_ratio = if player.data.deaths == 0 {
            player.data.kills as f32
        } else {
            player.data.kills as f32 / player.data.deaths as f32
        };
        if kd_ratio > 5.0 {
            suspected_cheats.push("Skill Anomaly".to_string());
            anomaly_details.insert(
                "K/D Ratio".to_string(),
                format!("K/D ratio of {:.1} is unusually high", kd_ratio),
            );
        }

        // Calculate overall cheating probability
        let mut cheating_probability = 0.0;
        if !suspected_cheats.is_empty() {
            cheating_probability =
                (accuracy * 0.3 + headshot_ratio * 0.5 + (kd_ratio / 10.0) * 0.2).min(1.0);
        }

        // Determine evidence strength
        let evidence_strength = if cheating_probability > 0.8 {
            "High".to_string()
        } else if cheating_probability > 0.5 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        };

        // Create the analysis result
        let result = FpsAnalysisResult {
            cheating_probability,
            suspected_cheats,
            evidence_strength,
            anomaly_details,
        };

        results.push(PlayerResult::new(player.player_id, result));
    }

    AnalysisResponse { results }
}

fn main() {
    // Create sample weapon stats
    let rifle_stats = WeaponStats {
        shots_fired: 100,
        hits: 90,
        headshots: 65,
        distance_meters: vec![15.5, 20.2, 18.7, 25.0, 10.2],
    };

    let pistol_stats = WeaponStats {
        shots_fired: 30,
        hits: 25,
        headshots: 20,
        distance_meters: vec![8.5, 12.2, 7.3],
    };

    // Create a weapon stats map
    let mut weapon_stats = HashMap::new();
    weapon_stats.insert("rifle".to_string(), rifle_stats);
    weapon_stats.insert("pistol".to_string(), pistol_stats);

    // Create FPS player data for a suspected cheater
    let cheater_data = FpsPlayerData {
        kills: 35,
        deaths: 5,
        assists: 8,
        weapon_stats: weapon_stats.clone(),
        average_position_change: 6.5,
        camping_seconds: 12,
        round_duration_seconds: 300,
        team: "CT".to_string(),
    };

    // Modify the weapon stats for a legitimate player
    let mut legitimate_weapon_stats = weapon_stats.clone();
    if let Some(rifle) = legitimate_weapon_stats.get_mut("rifle") {
        rifle.hits = 62;
        rifle.headshots = 20;
    }
    if let Some(pistol) = legitimate_weapon_stats.get_mut("pistol") {
        pistol.hits = 15;
        pistol.headshots = 5;
    }

    // Create FPS player data for a legitimate player
    let legitimate_data = FpsPlayerData {
        kills: 15,
        deaths: 12,
        assists: 7,
        weapon_stats: weapon_stats,
        average_position_change: 8.2,
        camping_seconds: 45,
        round_duration_seconds: 300,
        team: "T".to_string(),
    };

    // Create player stats objects
    let players = vec![
        PlayerStats::new("suspicious_player".to_string(), cheater_data),
        PlayerStats::new("legitimate_player".to_string(), legitimate_data),
    ];

    // Analyze the players
    let analysis = analyze_fps_players(players);

    // Print the results
    println!("Analysis Results:");
    for result in &analysis.results {
        println!("\nPlayer: {}", result.player_id);
        println!(
            "Cheating Probability: {:.1}%",
            result.data.cheating_probability * 100.0
        );
        println!("Evidence Strength: {}", result.data.evidence_strength);

        if !result.data.suspected_cheats.is_empty() {
            println!("Suspected Cheats:");
            for cheat in &result.data.suspected_cheats {
                println!("  - {}", cheat);
            }
        }

        if !result.data.anomaly_details.is_empty() {
            println!("Anomaly Details:");
            for (key, value) in &result.data.anomaly_details {
                println!("  - {}: {}", key, value);
            }
        }
    }

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&analysis).unwrap();
    println!("\nJSON Output:\n{}", json);

    // Show how to convert from DefaultPlayerData to our custom format
    println!("\nConverting from DefaultPlayerData to FpsPlayerData:");

    // Create default player data
    let mut shots = HashMap::new();
    shots.insert("rifle".to_string(), 100);

    let mut hits = HashMap::new();
    hits.insert("rifle".to_string(), 75);

    let default_data = DefaultPlayerData {
        shots_fired: shots,
        hits: hits,
        headshots: 30,
        shot_timestamps_ms: Some(vec![1000, 1200, 1500, 1800, 2100]),
        training_label: None,
    };

    let legacy_stats = PlayerStats::new("legacy_player".to_string(), default_data);
    // Convert to our custom format
    let converted_stats = legacy_stats.convert(|data| {
        // Create weapon stats for rifle
        let rifle_shots = *data.shots_fired.get("rifle").unwrap_or(&0);
        let rifle_hits = *data.hits.get("rifle").unwrap_or(&0);
        let rifle_stats = WeaponStats {
            shots_fired: rifle_shots,
            hits: rifle_hits,
            headshots: data.headshots,
            distance_meters: vec![15.0, 20.0, 25.0], // Dummy data
        };

        // Add to weapon stats map
        let mut weapon_stats = HashMap::new();
        weapon_stats.insert("rifle".to_string(), rifle_stats);

        // Create FPS player data
        FpsPlayerData {
            kills: rifle_hits, // Assume each hit is a kill for simplicity
            deaths: 10,        // Dummy value
            assists: 5,        // Dummy value
            weapon_stats,
            average_position_change: 7.5, // Dummy value
            camping_seconds: 20,          // Dummy value
            round_duration_seconds: 300,  // Dummy value
            team: "CT".to_string(),       // Dummy value
        }
    });

    println!("Original Legacy Player ID: {}", legacy_stats.player_id);
    println!("Converted Player ID: {}", converted_stats.player_id);
    println!(
        "Converted Accuracy: {:.1}%",
        converted_stats.data.calculate_accuracy_rate() * 100.0
    );
    println!(
        "Converted Headshot Ratio: {:.1}%",
        converted_stats.data.calculate_headshot_ratio() * 100.0
    );
}
