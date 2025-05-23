use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents player statistics from a game round.
///
/// This structure is generic over `T`, which allows it to work with any JSON structure.
///
/// # Example
///
/// ```no_run
/// use nocheat::types::{PlayerStats, DefaultPlayerData};
/// use std::collections::HashMap;
///
/// // Create stats for a player using the default data structure
/// let mut shots = HashMap::new();
/// shots.insert("rifle".to_string(), 100);
///
/// let mut hits = HashMap::new();
/// hits.insert("rifle".to_string(), 50);
///
/// let player_data = DefaultPlayerData {
///     shots_fired: shots,
///     hits: hits,
///     headshots: 10,
///     shot_timestamps_ms: None,
///     training_label: None,
/// };
///
/// let player_stats = PlayerStats::new("player123".to_string(), player_data);
///
/// assert_eq!(player_stats.player_id, "player123");
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct PlayerStats<T>
where
    T: Clone + Serialize,
{
    /// Unique identifier for the player
    pub player_id: String,
    /// Generic data structure containing player statistics
    pub data: T,
}

impl<T> PlayerStats<T>
where
    T: Clone + Serialize,
{
    /// Creates a new PlayerStats instance with the given player ID and data
    pub fn new(player_id: String, data: T) -> Self {
        PlayerStats { player_id, data }
    }

    /// Converts this PlayerStats instance into one with a different data type
    /// using the provided conversion function
    pub fn convert<U, F>(&self, converter: F) -> PlayerStats<U>
    where
        U: Clone + Serialize,
        F: FnOnce(&T) -> U,
    {
        PlayerStats {
            player_id: self.player_id.clone(),
            data: converter(&self.data),
        }
    }
}

/// Default data structure for player statistics
///
/// This provides the original functionality of PlayerStats but now as a separate
/// data structure that can be used with the generic PlayerStats
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct DefaultPlayerData {
    /// Number of shots fired per weapon type
    pub shots_fired: HashMap<String, u32>,
    /// Number of successful hits registered per weapon type
    pub hits: HashMap<String, u32>,
    /// Total number of headshots this round
    pub headshots: u32,
    /// Optional raw shot timestamps in milliseconds (for timing analysis)
    pub shot_timestamps_ms: Option<Vec<u64>>,
    /// Optional training label (1.0 for cheater, 0.0 for legitimate player)
    #[serde(default)]
    pub training_label: Option<f64>,
}

/// Analysis result for a single player.
///
/// Contains the suspicion score and a list of flags indicating
/// suspicious behaviors detected for the player. This struct is generic
/// to allow for different types of analysis based on the data type.
///
/// # Example
///
/// ```no_run
/// use nocheat::types::{PlayerResult, DefaultAnalysisResult};
///
/// let result = PlayerResult::new(
///     "player123".to_string(),
///     DefaultAnalysisResult {
///         suspicion_score: 0.75,
///         flags: vec!["HighHeadshotRatio".to_string()],
///     }
/// );
///
/// assert!(result.data.suspicion_score > 0.7);
/// assert!(result.data.flags.contains(&"HighHeadshotRatio".to_string()));
/// ```
#[derive(Debug, PartialEq, Serialize)]
pub struct PlayerResult<R>
where
    R: Serialize + PartialEq,
{
    /// Unique identifier for the player (same as in PlayerStats)
    pub player_id: String,
    /// Generic result data
    pub data: R,
}

impl<R> PlayerResult<R>
where
    R: Serialize + PartialEq,
{
    /// Creates a new PlayerResult with the given player ID and result data
    pub fn new(player_id: String, data: R) -> Self {
        PlayerResult { player_id, data }
    }
}

/// Default analysis result data structure
///
/// This provides the original functionality of PlayerResult but now as a separate
/// data structure that can be used with the generic PlayerResult
#[derive(Debug, PartialEq, Serialize)]
pub struct DefaultAnalysisResult {
    /// Score between 0.0 and 1.0 indicating likelihood of cheating
    pub suspicion_score: f32,
    /// List of flags indicating specific suspicious behaviors
    pub flags: Vec<String>,
}

/// Response wrapper containing analysis results for multiple players.
/// This struct is generic to work with different analysis result types.
///
/// # Example
///
/// ```no_run
/// use nocheat::types::{AnalysisResponse, PlayerResult, DefaultAnalysisResult};
///
/// let response = AnalysisResponse {
///     results: vec![
///         PlayerResult::new(
///             "player123".to_string(),
///             DefaultAnalysisResult {
///                 suspicion_score: 0.75,
///                 flags: vec!["HighHeadshotRatio".to_string()],
///             }
///         ),
///         PlayerResult::new(
///             "player456".to_string(),
///             DefaultAnalysisResult {
///                 suspicion_score: 0.2,
///                 flags: vec![],
///             }
///         )
///     ],
/// };
///
/// assert_eq!(response.results.len(), 2);
/// assert!(response.results[0].data.suspicion_score > response.results[1].data.suspicion_score);
/// ```
#[derive(Debug, PartialEq, Serialize)]
pub struct AnalysisResponse<R>
where
    R: Serialize + PartialEq,
{
    /// List of analysis results for all players
    pub results: Vec<PlayerResult<R>>,
}

/// Type alias for backward compatibility with the original PlayerStats struct
pub type LegacyPlayerStats = PlayerStats<DefaultPlayerData>;

/// Type alias for backward compatibility with the original PlayerResult struct
pub type LegacyPlayerResult = PlayerResult<DefaultAnalysisResult>;

/// Type alias for backward compatibility with the original AnalysisResponse struct
pub type LegacyAnalysisResponse = AnalysisResponse<DefaultAnalysisResult>;

/// Implementation for deserialization of PlayerStats with a DefaultPlayerData structure
impl<'de> Deserialize<'de> for PlayerStats<DefaultPlayerData> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Custom struct that handles flattened fields
        #[derive(Deserialize)]
        struct FlatPlayerStats {
            player_id: String,
            shots_fired: HashMap<String, u32>,
            hits: HashMap<String, u32>,
            headshots: u32,
            shot_timestamps_ms: Option<Vec<u64>>,
            #[serde(default)]
            training_label: Option<f64>,
        }

        let flat = FlatPlayerStats::deserialize(deserializer)?;

        // Create the nested structure
        let data = DefaultPlayerData {
            shots_fired: flat.shots_fired,
            hits: flat.hits,
            headshots: flat.headshots,
            shot_timestamps_ms: flat.shot_timestamps_ms,
            training_label: flat.training_label,
        };

        Ok(PlayerStats {
            player_id: flat.player_id,
            data,
        })
    }
}

/// A trait for data types that can be analyzed for cheating behavior
pub trait Analyzable {
    /// Calculate the accuracy rate for this player
    fn calculate_accuracy_rate(&self) -> f32;

    /// Calculate the headshot ratio for this player
    fn calculate_headshot_ratio(&self) -> f32;

    /// Extract features for machine learning models
    fn extract_features(&self) -> Vec<f32>;

    /// Check if this player's behavior is suspicious
    fn is_suspicious(&self) -> bool;
}

impl Analyzable for DefaultPlayerData {
    fn calculate_accuracy_rate(&self) -> f32 {
        let total_shots: u32 = self.shots_fired.values().sum();
        let total_hits: u32 = self.hits.values().sum();

        if total_shots == 0 {
            return 0.0;
        }

        total_hits as f32 / total_shots as f32
    }

    fn calculate_headshot_ratio(&self) -> f32 {
        let total_hits: u32 = self.hits.values().sum();

        if total_hits == 0 {
            return 0.0;
        }

        self.headshots as f32 / total_hits as f32
    }

    fn extract_features(&self) -> Vec<f32> {
        vec![
            self.calculate_accuracy_rate(),
            self.calculate_headshot_ratio(),
        ]
    }

    fn is_suspicious(&self) -> bool {
        let accuracy = self.calculate_accuracy_rate();
        let headshot_ratio = self.calculate_headshot_ratio();

        accuracy > 0.8 || headshot_ratio > 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_stats_creation() {
        let mut shots = HashMap::new();
        shots.insert("rifle".to_string(), 100);
        shots.insert("pistol".to_string(), 20);

        let mut hits = HashMap::new();
        hits.insert("rifle".to_string(), 50);
        hits.insert("pistol".to_string(), 15);

        let player_data = DefaultPlayerData {
            shots_fired: shots,
            hits: hits,
            headshots: 10,
            shot_timestamps_ms: Some(vec![100, 200, 300]),
            training_label: None,
        };

        let stats = PlayerStats::new("player123".to_string(), player_data);

        assert_eq!(stats.player_id, "player123");
        assert_eq!(*stats.data.shots_fired.get("rifle").unwrap(), 100);
        assert_eq!(*stats.data.hits.get("pistol").unwrap(), 15);
        assert_eq!(stats.data.headshots, 10);
        assert_eq!(stats.data.shot_timestamps_ms.unwrap().len(), 3);
    }

    #[test]
    fn test_player_result_creation() {
        let result_data = DefaultAnalysisResult {
            suspicion_score: 0.75,
            flags: vec!["HighHeadshotRatio".to_string(), "AimSnap".to_string()],
        };

        let result = PlayerResult::new("player123".to_string(), result_data);

        assert_eq!(result.player_id, "player123");
        assert_eq!(result.data.suspicion_score, 0.75);
        assert_eq!(result.data.flags.len(), 2);
        assert!(result.data.flags.contains(&"HighHeadshotRatio".to_string()));
    }

    #[test]
    fn test_analysis_response_creation() {
        let response = AnalysisResponse {
            results: vec![
                PlayerResult::new(
                    "player123".to_string(),
                    DefaultAnalysisResult {
                        suspicion_score: 0.75,
                        flags: vec!["HighHeadshotRatio".to_string()],
                    },
                ),
                PlayerResult::new(
                    "player456".to_string(),
                    DefaultAnalysisResult {
                        suspicion_score: 0.2,
                        flags: vec![],
                    },
                ),
            ],
        };

        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].player_id, "player123");
        assert_eq!(response.results[1].player_id, "player456");
    }

    #[test]
    fn test_convert_player_stats() {
        let mut shots = HashMap::new();
        shots.insert("rifle".to_string(), 100);

        let mut hits = HashMap::new();
        hits.insert("rifle".to_string(), 50);

        let player_data = DefaultPlayerData {
            shots_fired: shots,
            hits: hits,
            headshots: 10,
            shot_timestamps_ms: None,
            training_label: None,
        };

        let legacy_stats = PlayerStats::new("player123".to_string(), player_data);

        // Define a simple custom data type
        #[derive(Clone, Debug, Serialize)]
        struct SimpleData {
            accuracy: f32,
            headshot_ratio: f32,
        }

        // Convert to the custom data type
        let custom_stats = legacy_stats.convert(|data| {
            let accuracy = data.calculate_accuracy_rate();
            let headshot_ratio = data.calculate_headshot_ratio();

            SimpleData {
                accuracy,
                headshot_ratio,
            }
        });

        assert_eq!(custom_stats.player_id, "player123");
        assert_eq!(custom_stats.data.accuracy, 0.5);
        assert_eq!(custom_stats.data.headshot_ratio, 0.2);
    }
    #[test]
    fn test_analyzable_trait() {
        let mut shots = HashMap::new();
        shots.insert("rifle".to_string(), 100);
        shots.insert("pistol".to_string(), 50);

        let mut hits = HashMap::new();
        hits.insert("rifle".to_string(), 90); // 90% accuracy for rifle (suspicious)
        hits.insert("pistol".to_string(), 40); // 80% accuracy for pistol (suspicious)

        let player_data = DefaultPlayerData {
            shots_fired: shots,
            hits: hits,
            headshots: 70, // Very high headshot count (suspicious)
            shot_timestamps_ms: None,
            training_label: None,
        };

        // Calculate accuracy rate
        assert_eq!(player_data.calculate_accuracy_rate(), 130.0 / 150.0);

        // Calculate headshot ratio
        assert_eq!(player_data.calculate_headshot_ratio(), 70.0 / 130.0);

        // Check if suspicious - should be true with these values
        assert!(player_data.is_suspicious());

        // Extract features
        let features = player_data.extract_features();
        assert_eq!(features.len(), 2);
        assert_eq!(features[0], 130.0 / 150.0);
        assert_eq!(features[1], 70.0 / 130.0);
    }
}
