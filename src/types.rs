use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents player statistics from a game round.
///
/// This structure contains all the statistics for a single player that are 
/// needed to analyze whether the player might be cheating.
///
/// # Example
///
/// ```no_run
/// use nocheat::types::PlayerStats;
/// use std::collections::HashMap;
///
/// // Create stats for a player
/// let mut shots = HashMap::new();
/// shots.insert("rifle".to_string(), 100);
/// 
/// let mut hits = HashMap::new();
/// hits.insert("rifle".to_string(), 50);
///
/// let player_stats = PlayerStats {
///     player_id: "player123".to_string(),
///     shots_fired: shots,
///     hits: hits,
///     headshots: 10,
///     shot_timestamps_ms: None,
///     training_label: None,
/// };
///
/// assert_eq!(player_stats.player_id, "player123");
/// ```
#[derive(Deserialize, Clone)]
pub struct PlayerStats {
    /// Unique identifier for the player
    pub player_id: String,
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
/// suspicious behaviors detected for the player.
///
/// # Example
///
/// ```no_run
/// use nocheat::types::PlayerResult;
///
/// let result = PlayerResult {
///     player_id: "player123".to_string(),
///     suspicion_score: 0.75,
///     flags: vec!["HighHeadshotRatio".to_string()],
/// };
///
/// assert!(result.suspicion_score > 0.7);
/// assert!(result.flags.contains(&"HighHeadshotRatio".to_string()));
/// ```
#[derive(Serialize, Debug, PartialEq)]
pub struct PlayerResult {
    /// Unique identifier for the player (same as in PlayerStats)
    pub player_id: String,
    /// Score between 0.0 and 1.0 indicating likelihood of cheating
    pub suspicion_score: f32,
    /// List of flags indicating specific suspicious behaviors
    pub flags: Vec<String>,
}

/// Response wrapper containing analysis results for multiple players.
///
/// # Example
///
/// ```no_run
/// use nocheat::types::{AnalysisResponse, PlayerResult};
///
/// let response = AnalysisResponse {
///     results: vec![
///         PlayerResult {
///             player_id: "player123".to_string(),
///             suspicion_score: 0.75,
///             flags: vec!["HighHeadshotRatio".to_string()],
///         },
///         PlayerResult {
///             player_id: "player456".to_string(),
///             suspicion_score: 0.2,
///             flags: vec![],
///         }
///     ],
/// };
///
/// assert_eq!(response.results.len(), 2);
/// assert!(response.results[0].suspicion_score > response.results[1].suspicion_score);
/// ```
#[derive(Serialize, Debug, PartialEq)]
pub struct AnalysisResponse {
    /// List of analysis results for all players
    pub results: Vec<PlayerResult>,
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

        let stats = PlayerStats {
            player_id: "player123".to_string(),
            shots_fired: shots,
            hits: hits,
            headshots: 10,
            shot_timestamps_ms: Some(vec![100, 200, 300]),
            training_label: None,
        };

        assert_eq!(stats.player_id, "player123");
        assert_eq!(*stats.shots_fired.get("rifle").unwrap(), 100);
        assert_eq!(*stats.hits.get("pistol").unwrap(), 15);
        assert_eq!(stats.headshots, 10);
        assert_eq!(stats.shot_timestamps_ms.unwrap().len(), 3);
    }

    #[test]
    fn test_player_result_creation() {
        let result = PlayerResult {
            player_id: "player123".to_string(),
            suspicion_score: 0.75,
            flags: vec!["HighHeadshotRatio".to_string(), "AimSnap".to_string()],
        };

        assert_eq!(result.player_id, "player123");
        assert_eq!(result.suspicion_score, 0.75);
        assert_eq!(result.flags.len(), 2);
        assert!(result.flags.contains(&"HighHeadshotRatio".to_string()));
    }

    #[test]
    fn test_analysis_response_creation() {
        let response = AnalysisResponse {
            results: vec![
                PlayerResult {
                    player_id: "player123".to_string(),
                    suspicion_score: 0.75,
                    flags: vec!["HighHeadshotRatio".to_string()],
                },
                PlayerResult {
                    player_id: "player456".to_string(),
                    suspicion_score: 0.2,
                    flags: vec![],
                },
            ],
        };

        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].player_id, "player123");
        assert_eq!(response.results[1].player_id, "player456");
    }
}