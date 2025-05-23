# Guide: Using Generic PlayerStats in NoCheat

The NoCheat library now supports generic data structures for player stats, allowing you to customize the data that is analyzed for cheat detection.

## Key Components

1. **PlayerStats<T>**: A generic structure that can work with any type of data
2. **PlayerResult<R>**: A generic structure for analysis results
3. **AnalysisResponse<R>**: A generic response wrapper for multiple player results
4. **DefaultPlayerData**: The original data structure (for backward compatibility)
5. **DefaultAnalysisResult**: The original result structure (for backward compatibility)

## Type Aliases for Backward Compatibility

- **LegacyPlayerStats** = PlayerStats<DefaultPlayerData>
- **LegacyPlayerResult** = PlayerResult<DefaultAnalysisResult>
- **LegacyAnalysisResponse** = AnalysisResponse<DefaultAnalysisResult>

## Examples

Run examples:
- `cargo run --example generic_usage`
- `cargo run --example fps_game_analysis`
- `cargo run --example multi_game_analysis`
- `cargo run --example train_model_example`


### Using the Default Structure (Backward Compatible)

```rust
use nocheat::types::{PlayerStats, DefaultPlayerData};
use std::collections::HashMap;

// Create default data
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

// Create a PlayerStats instance
let player_stats = PlayerStats::new("player123".to_string(), default_data);
```

### Creating Custom Data Structures

```rust
use nocheat::types::PlayerStats;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CustomPlayerData {
    accuracy: f32,
    reaction_time_ms: Vec<u32>,
    movement_patterns: HashMap<String, u32>,
    mouse_acceleration: Option<f32>,
}

// Create custom data
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
```

### Custom Analysis Results

```rust
use nocheat::types::{PlayerResult, AnalysisResponse};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
struct CustomAnalysisResult {
    cheating_probability: f32,
    abnormal_patterns: Vec<String>,
    confidence_score: f32,
    recommended_action: String,
}

let custom_result = CustomAnalysisResult {
    cheating_probability: 0.85,
    abnormal_patterns: vec!["AimSnap".to_string(), "RecoilControl".to_string()],
    confidence_score: 0.92,
    recommended_action: "Review gameplay footage".to_string(),
};

let player_result = PlayerResult::new("custom_player".to_string(), custom_result);

// Group multiple results
let response = AnalysisResponse {
    results: vec![player_result],
};
```

## Best Practices

1. Ensure your custom data and result types implement the necessary traits:
   - Data types: `Clone + Serialize + DeserializeOwned`
   - Result types: `Serialize + PartialEq`

2. Consider implementing serialization/deserialization for your custom types if you need to parse JSON or other formats.

3. When defining custom analysis functions, specify the generic types explicitly:
   ```rust
   fn analyze_custom_data(stats: Vec<PlayerStats<CustomPlayerData>>) -> Result<AnalysisResponse<CustomAnalysisResult>> {
       // Your analysis logic here
   }
   ```

## Complete Example

See the file `examples/generic_usage.rs` for a complete working example of custom data structures and analysis results.
