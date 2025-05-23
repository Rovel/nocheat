# Using Generic JSON Training Data with NoCheat

This guide demonstrates how to use the generic JSON training data samples provided with NoCheat for different game types. The library's generic structure allows you to analyze any type of game data by adapting the player statistics format to your specific needs.

## Available Sample Data

We've included several JSON data samples for different game types:

1. **FPS Game Data** (`fps_training_data.json`)
   - Focuses on weapon accuracy, headshot ratios, and kill/death ratios
   - Includes detailed weapon stats per weapon type
   - Tracks movement patterns and camping behavior

2. **MOBA Game Data** (`moba_training_data.json`)
   - Focuses on champion performance, damage dealt, and skill usage patterns
   - Includes detailed champion stats, gold earned, and item build orders
   - Tracks skill usage timings and crowd control scores

3. **Battle Royale Game Data** (`battle_royale_training_data.json`)
   - Focuses on survival time, kills, and damage dealt
   - Includes looting patterns, movement across the map
   - Tracks weapon accuracy and placement statistics

4. **Racing Game Data** (`racing_training_data.json`)
   - Focuses on lap times, consistency, and racing patterns
   - Includes detailed telemetry data such as speed, braking points
   - Tracks sector times and racing line efficiency

## Loading and Using the Generic JSON Data

### 1. Define Your Custom Data Structure

First, you need to define a Rust structure that matches your JSON format:

```rust
// For FPS games
#[derive(Clone, Debug, Deserialize, Serialize)]
struct FpsPlayerData {
    kills: u32,
    deaths: u32,
    assists: u32,
    weapon_stats: HashMap<String, WeaponStats>,
    average_position_change: f32,
    camping_seconds: u32,
    round_duration_seconds: u32,
    team: String,
    training_label: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct WeaponStats {
    shots_fired: u32,
    hits: u32,
    headshots: u32,
    distance_meters: Vec<f32>,
}
```

### 2. Implement the Analyzable Trait

```rust
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
        // Return relevant features for model training
        vec![
            self.calculate_accuracy_rate(),
            self.calculate_headshot_ratio(),
            self.kills as f32 / self.deaths.max(1) as f32,
        ]
    }
    
    fn is_suspicious(&self) -> bool {
        // Define suspicious behavior
        self.calculate_accuracy_rate() > 0.8 || 
        self.calculate_headshot_ratio() > 0.7 ||
        (self.kills > 30 && self.deaths < 5)
    }
}
```

### 3. Load JSON Data and Create PlayerStats Instances

```rust
use std::fs::File;
use std::io::Read;
use serde_json::from_str;
use nocheat::types::PlayerStats;

fn load_fps_training_data(path: &str) -> Vec<PlayerStats<FpsPlayerData>> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    from_str(&contents).expect("Failed to parse JSON")
}

// Load the training data
let fps_players = load_fps_training_data("examples/json_samples/fps_training_data.json");
```

### 4. Create a Game-Specific Analyzer

```rust
fn analyze_fps_data(players: Vec<PlayerStats<FpsPlayerData>>) -> AnalysisResponse<FpsAnalysisResult> {
    let mut results = Vec::new();
    
    for player in players {
        // Extract features specific to FPS games
        let accuracy = player.data.calculate_accuracy_rate();
        let headshot_ratio = player.data.calculate_headshot_ratio();
        let kd_ratio = player.data.kills as f32 / player.data.deaths.max(1) as f32;
        
        // Build a list of suspected cheats
        let mut suspected_cheats = Vec::new();
        let mut anomaly_details = HashMap::new();
        
        if accuracy > 0.8 {
            suspected_cheats.push("Aimbot".to_string());
            anomaly_details.insert("High Accuracy".to_string(), 
                                  format!("{:.1}% hit rate", accuracy * 100.0));
        }
        
        if headshot_ratio > 0.7 {
            suspected_cheats.push("Headshot Hack".to_string());
            anomaly_details.insert("Headshot Anomaly".to_string(),
                                  format!("{:.1}% headshot ratio", headshot_ratio * 100.0));
        }
        
        if kd_ratio > 5.0 {
            suspected_cheats.push("Skill Anomaly".to_string());
            anomaly_details.insert("K/D Ratio".to_string(),
                                  format!("K/D ratio of {:.1}", kd_ratio));
        }
        
        // Calculate overall cheating probability
        let cheating_probability = (accuracy * 0.4 + headshot_ratio * 0.4 + (kd_ratio / 10.0) * 0.2)
            .min(1.0);
            
        // Create the result
        let result = FpsAnalysisResult {
            cheating_probability,
            suspected_cheats,
            evidence_strength: if cheating_probability > 0.8 { "High" } 
                               else if cheating_probability > 0.5 { "Medium" }
                               else { "Low" }.to_string(),
            anomaly_details,
        };
        
        results.push(PlayerResult::new(player.player_id, result));
    }
    
    AnalysisResponse { results }
}
```

### 5. Train a Custom Model Using Game-Specific Features

```rust
use nocheat::train_model;

fn train_fps_cheat_model(fps_players: Vec<PlayerStats<FpsPlayerData>>, output_path: &str) -> Result<()> {
    // Extract features for the model
    let mut features = Vec::new();
    let mut labels = Vec::new();
    
    for player in &fps_players {
        // Create feature vector
        let player_features = vec![
            player.data.calculate_accuracy_rate() as f64,
            player.data.calculate_headshot_ratio() as f64,
            (player.data.kills as f64) / (player.data.deaths.max(1) as f64),
            player.data.average_position_change as f64,
            (player.data.camping_seconds as f64) / (player.data.round_duration_seconds as f64)
        ];
        
        features.push(player_features);
        
        // Get the training label
        if let Some(label) = player.data.training_label {
            labels.push(label);
        } else {
            // Skip players without labels
            features.pop();
        }
    }
    
    // Train the model using the nocheat training API
    // This is a simplified example - you would need to adapt this to work with your feature vectors
    let _result = train_custom_model(features, labels, output_path);
    
    Ok(())
}
```

## Adapting to Other Game Types

The same pattern can be applied to the other game types:

1. Define a Rust structure matching your JSON structure
2. Implement the `Analyzable` trait for your game-specific data
3. Load the JSON data and create `PlayerStats<YourDataType>` instances
4. Create a game-specific analyzer that extracts relevant features
5. Train a custom model tailored to your game

For each game type, focus on the metrics that are most relevant for cheat detection:

- **MOBA Games**: Look for abnormal damage output, unrealistic skill timing patterns, and suspiciously high gold/xp rates
- **Battle Royale Games**: Focus on hit accuracy, headshot ratios, and unusual movement/looting patterns
- **Racing Games**: Check for impossible lap times, unrealistic cornering speeds, and perfect racing lines

## Using Different Models for Different Games

You can maintain separate trained models for each game type:

```rust
// Load game-specific models
let fps_model = load_model("models/fps_cheat_model.bin").expect("Failed to load FPS model");
let moba_model = load_model("models/moba_cheat_model.bin").expect("Failed to load MOBA model");
let br_model = load_model("models/battle_royale_cheat_model.bin").expect("Failed to load BR model");
let racing_model = load_model("models/racing_cheat_model.bin").expect("Failed to load Racing model");
```

This approach allows you to have specialized cheat detection tailored to each game's unique mechanics and cheating patterns.
