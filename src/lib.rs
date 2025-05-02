/*!
# NoCheat Anti-Cheat Library

A machine learning-based anti-cheat library for detecting suspicious player behavior in multiplayer games.
This library uses a RandomForest classifier to analyze player statistics and identify potential cheaters.

## Features

- Fast analysis of player statistics
- Machine learning-based detection
- C-compatible FFI for integration with game engines
- DataFrame-based feature engineering

## Usage Examples

Simple usage from Rust:

```rust
use nocheat::{analyze_stats};
use nocheat::types::{PlayerStats, AnalysisResponse};
use std::collections::HashMap;

// Prepare player statistics
let mut shots = HashMap::new();
shots.insert("rifle".to_string(), 100);
let mut hits = HashMap::new();
hits.insert("rifle".to_string(), 80);  // Unusually high accuracy

let player_stats = PlayerStats {
    player_id: "suspect_player".to_string(),
    shots_fired: shots,
    hits: hits,
    headshots: 40,  // Very high headshot ratio
    shot_timestamps_ms: None,
};

// Analyze the stats
let analysis = analyze_stats(vec![player_stats]);
if let Ok(response) = analysis {
    for result in response.results {
        println!("Player {} has suspicion score: {}", 
                 result.player_id, result.suspicion_score);
        
        if result.suspicion_score > 0.7 {
            println!("This player is likely cheating!");
            println!("Flags: {:?}", result.flags);
        }
    }
}
```
*/

use anyhow::Result;
use libc::{c_int, c_uchar, size_t};
use once_cell::sync::Lazy;
use polars::prelude::*;
use std::{fs::File, ptr};
use ndarray::Array2;
use randomforest::RandomForestClassifier;

pub mod types;
use types::{AnalysisResponse, PlayerResult, PlayerStats};

/// Public wrapper for statistical analysis of player data to detect cheating.
///
/// This is the main entry point for the library. It takes a vector of player 
/// statistics and returns an analysis response containing suspicion scores
/// and behavioral flags for each player.
///
/// # Arguments
///
/// * `stats` - A vector of PlayerStats structures containing data to analyze
///
/// # Returns
///
/// * `Result<AnalysisResponse>` - The analysis results wrapped in a Result
///
/// # Example
///
/// ```no_run
/// use nocheat::{analyze_stats};
/// use nocheat::types::PlayerStats;
/// use std::collections::HashMap;
///
/// // Create player statistics
/// let mut shots = HashMap::new();
/// shots.insert("rifle".to_string(), 100);
/// let mut hits = HashMap::new();
/// hits.insert("rifle".to_string(), 50);
///
/// let stats = vec![PlayerStats {
///     player_id: "player123".to_string(),
///     shots_fired: shots,
///     hits: hits,
///     headshots: 10,
///     shot_timestamps_ms: None,
/// }];
///
/// let results = analyze_stats(stats).expect("Analysis failed");
/// assert_eq!(results.results.len(), 1);
/// ```
pub fn analyze_stats(stats: Vec<PlayerStats>) -> Result<AnalysisResponse> {
    do_analysis(stats)
}

/// Load pre-trained RandomForest model on first use
static RF_MODEL: Lazy<RandomForestClassifier> = Lazy::new(|| {
    load_model("cheat_model.bin").expect("Failed to load RF model")
});

/// Deserialize RF from file
fn load_model(path: &str) -> Result<RandomForestClassifier> {
    let file = File::open(path)?;
    // Use deserialize method provided by RandomForestClassifier
    let rf = RandomForestClassifier::deserialize(file)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize model: {}", e))?;
    Ok(rf)
}

/// Build a Polars DataFrame from PlayerStats
///
/// Converts a slice of PlayerStats into a DataFrame for easier analysis.
///
/// # Arguments
///
/// * `stats` - A slice of PlayerStats structures
///
/// # Returns
///
/// * `Result<DataFrame>` - A DataFrame containing player statistics
///
/// # Example
///
/// ```
/// use nocheat::{build_dataframe};
/// use nocheat::types::PlayerStats;
/// use std::collections::HashMap;
///
/// // Create test player statistics
/// let mut shots = HashMap::new();
/// shots.insert("rifle".to_string(), 100);
/// let mut hits = HashMap::new();
/// hits.insert("rifle".to_string(), 50);
///
/// let stats = vec![PlayerStats {
///     player_id: "player123".to_string(),
///     shots_fired: shots,
///     hits: hits,
///     headshots: 10,
///     shot_timestamps_ms: None,
/// }];
///
/// let df = build_dataframe(&stats).expect("DataFrame creation failed");
/// assert_eq!(df.height(), 1);
/// ```
pub fn build_dataframe(stats: &[PlayerStats]) -> Result<DataFrame> {
    let ids: Vec<&str> = stats.iter().map(|p| p.player_id.as_str()).collect();
    let shots: Vec<u32>  = stats.iter().map(|p| p.shots_fired.values().sum()).collect();
    let hits: Vec<u32>   = stats.iter().map(|p| p.hits.values().sum()).collect();
    let headshots: Vec<u32> = stats.iter().map(|p| p.headshots).collect();

    let df = df! {
        "player_id" => ids,
        "shots"     => shots,
        "hits"      => hits,
        "headshots" => headshots,
    }?;
    Ok(df)
}

/// Convert selected DataFrame columns into an ndarray for model inference
///
/// Extracts specific columns from a DataFrame and converts them to a 2D ndarray
/// format that can be used for machine learning model inference.
///
/// # Arguments
///
/// * `df` - A reference to the source DataFrame
/// * `cols` - A slice of column names to extract
///
/// # Returns
///
/// * `Result<Array2<f32>>` - A 2D array containing the extracted data
///
/// # Example
///
/// ```no_run
/// // Note: This example is marked as no_run to avoid compilation issues in doctests
/// use nocheat::{build_dataframe, df_to_ndarray};
/// use nocheat::types::PlayerStats;
/// use std::collections::HashMap;
/// use polars::prelude::{col, IntoLazy, DataType};
///
/// // Create test player statistics
/// let mut shots = HashMap::new();
/// shots.insert("rifle".to_string(), 100);
/// let mut hits = HashMap::new();
/// hits.insert("rifle".to_string(), 50);
///
/// let stats = vec![PlayerStats {
///     player_id: "player123".to_string(),
///     shots_fired: shots,
///     hits: hits,
///     headshots: 10,
///     shot_timestamps_ms: None,
/// }];
///
/// let df = build_dataframe(&stats).expect("DataFrame creation failed");
/// 
/// // Add computed columns
/// let df = df.lazy()
///     .with_column((col("hits").cast(DataType::Float32) / col("shots").cast(DataType::Float32))
///         .alias("hit_rate"))
///     .collect()
///     .expect("Failed to compute hit_rate");
///
/// let features = df_to_ndarray(&df, &["hit_rate"]).expect("Failed to convert to ndarray");
/// assert_eq!(features.shape()[0], 1); // One row
/// assert_eq!(features.shape()[1], 1); // One column
/// ```
pub fn df_to_ndarray(df: &DataFrame, cols: &[&str]) -> Result<Array2<f32>> {
    let n = df.height();
    let m = cols.len();
    let mut arr = Array2::<f32>::zeros((n, m));
    for (j, &col_name) in cols.iter().enumerate() {
        let ca = df.column(col_name)?.f32()?;
        for (i, v) in ca.into_no_null_iter().enumerate() {
            arr[(i, j)] = v;
        }
    }
    Ok(arr)
}

/// Core analysis function: feature engineering + RF inference
fn do_analysis(stats: Vec<PlayerStats>) -> Result<AnalysisResponse> {
    // 1. DataFrame
    let mut df = build_dataframe(&stats)?;

    // 2. Compute features lazily
    let lf = df.lazy()
        .with_column((col("hits") / col("shots")).alias("hit_rate"))
        .with_column((col("headshots") / col("hits")).alias("headshot_rate"));
    df = lf.collect()?;

    // 3. Extract features for RF
    let features = df_to_ndarray(&df, &["hit_rate", "headshot_rate"])?;
    
    // 4. Model inference - properly handle prediction for each row
    let mut results = Vec::with_capacity(stats.len());
    let hit_rates = df.column("hit_rate")?.f32()?;
    
    for (i, stat) in stats.into_iter().enumerate() {
        // Convert features to f64 array for each row as expected by RandomForestClassifier
        let row_features: Vec<f64> = features.row(i)
            .iter()
            .map(|&v| v as f64)
            .collect();
            
        // Get prediction score (single f64 value)
        let score = RF_MODEL.predict(&row_features) as f32;
        
        // Build flags
        let mut flags = Vec::new();
        if hit_rates.get(i).unwrap() > 0.8 {
            flags.push("HighHitRate".to_string());
        }
        
        results.push(PlayerResult { 
            player_id: stat.player_id, 
            suspicion_score: score, 
            flags 
        });
    }

    Ok(AnalysisResponse { results })
}

/// FFI: analyze a JSON buffer of PlayerStats; returns JSON buffer
///
/// This function provides a C-compatible interface for the cheat detection system.
/// It takes a JSON buffer containing player statistics, analyzes them, and returns 
/// the results as a JSON buffer.
///
/// # Safety
///
/// This function is unsafe because it deals with raw pointers and memory allocation
/// across the FFI boundary. The caller is responsible for:
///
/// - Ensuring the input pointers are valid and properly aligned
/// - Freeing the returned buffer using the `free_buffer` function
///
/// # Arguments
///
/// * `stats_json_ptr` - Pointer to a UTF-8 encoded JSON buffer
/// * `stats_json_len` - Length of the JSON buffer in bytes
/// * `out_json_ptr` - Pointer to a location where the output buffer pointer will be stored
/// * `out_json_len` - Pointer to a location where the output buffer length will be stored
///
/// # Returns
///
/// * `0` on success
/// * Negative values on various errors:
///   * `-1` - Null pointer provided
///   * `-2` - JSON parsing error
///   * `-3` - Analysis error
///   * `-4` - Serialization error
///   * `-5` - Memory allocation error
#[no_mangle]
pub extern "C" fn analyze_round(
    stats_json_ptr: *const c_uchar,
    stats_json_len: size_t,
    out_json_ptr: *mut *mut c_uchar,
    out_json_len: *mut size_t,
) -> c_int {
    // safety: assume valid UTF-8 JSON
    if stats_json_ptr.is_null() || out_json_ptr.is_null() || out_json_len.is_null() {
        return -1;
    }
    let input = unsafe { std::slice::from_raw_parts(stats_json_ptr, stats_json_len) };
    let stats: Vec<PlayerStats> = match serde_json::from_slice(input) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    match analyze_stats(stats) {
        Ok(resp) => write_buffer(&resp, out_json_ptr, out_json_len),
        Err(_) => -3,
    }
}

/// Companion to free allocated buffer
///
/// This function must be called to free the memory allocated by `analyze_round`.
///
/// # Safety
///
/// This function is unsafe because it deals with raw pointers and memory deallocation.
/// The caller must ensure that:
///
/// - The pointer was previously allocated by `analyze_round`
/// - The pointer has not already been freed
/// - The length matches what was given in `out_json_len`
///
/// # Arguments
///
/// * `ptr` - Pointer to the buffer to free
/// * `len` - Length of the buffer in bytes
#[no_mangle]
pub extern "C" fn free_buffer(ptr: *mut c_uchar, len: size_t) {
    if ptr.is_null() || len == 0 { return; }
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}

/// Serialize response and allocate C buffer
fn write_buffer(
    resp: &AnalysisResponse,
    out_json_ptr: *mut *mut c_uchar,
    out_json_len: *mut size_t,
) -> c_int {
    let json = match serde_json::to_vec(resp) {
        Ok(j) => j,
        Err(_) => return -4,
    };
    let len = json.len();
    unsafe {
        let buf = libc::malloc(len) as *mut c_uchar;
        if buf.is_null() { return -5; }
        ptr::copy_nonoverlapping(json.as_ptr(), buf, len);
        *out_json_ptr = buf;
        *out_json_len = len;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_stats() -> Vec<PlayerStats> {
        let mut shots1 = HashMap::new();
        shots1.insert("rifle".to_string(), 100);
        let mut hits1 = HashMap::new();
        hits1.insert("rifle".to_string(), 50);
        
        let mut shots2 = HashMap::new();
        shots2.insert("rifle".to_string(), 100);
        shots2.insert("pistol".to_string(), 50);
        let mut hits2 = HashMap::new();
        hits2.insert("rifle".to_string(), 90); // suspicious hit rate
        hits2.insert("pistol".to_string(), 45); // suspicious hit rate

        vec![
            PlayerStats {
                player_id: "normal_player".to_string(),
                shots_fired: shots1,
                hits: hits1,
                headshots: 10,
                shot_timestamps_ms: None,
            },
            PlayerStats {
                player_id: "suspicious_player".to_string(),
                shots_fired: shots2,
                hits: hits2,
                headshots: 50, // suspicious headshot count
                shot_timestamps_ms: None,
            },
        ]
    }

    #[test]
    fn test_build_dataframe_columns() {
        let stats = create_test_stats();
        let df = build_dataframe(&stats).expect("DataFrame creation failed");
        
        // Verify the DataFrame structure
        assert_eq!(df.height(), 2);
        assert_eq!(df.width(), 4);
        assert!(df.column("player_id").is_ok());
        assert!(df.column("shots").is_ok());
        assert!(df.column("hits").is_ok());
        assert!(df.column("headshots").is_ok());
    }

    #[test]
    fn test_build_dataframe_values() {
        let stats = create_test_stats();
        let df = build_dataframe(&stats).expect("DataFrame creation failed");
        
        // Check specific values
        let player_ids = df.column("player_id").unwrap();
        // Using string conversion instead of direct utf8 access
        let player_id_0 = player_ids.get(0).unwrap().to_string();
        let player_id_1 = player_ids.get(1).unwrap().to_string();
        assert!(player_id_0.contains("normal_player"));
        assert!(player_id_1.contains("suspicious_player"));
        
        let shots = df.column("shots").unwrap().u32().unwrap();
        assert_eq!(shots.get(0), Some(100));
        assert_eq!(shots.get(1), Some(150)); // 100 + 50
        
        let hits = df.column("hits").unwrap().u32().unwrap();
        assert_eq!(hits.get(0), Some(50));
        assert_eq!(hits.get(1), Some(135)); // 90 + 45
        
        let headshots = df.column("headshots").unwrap().u32().unwrap();
        assert_eq!(headshots.get(0), Some(10));
        assert_eq!(headshots.get(1), Some(50));
    }

    #[test]
    fn test_df_to_ndarray_conversion() {
        let stats = create_test_stats();
        let df = build_dataframe(&stats).expect("DataFrame creation failed");
        
        // Create a test column
        let df = df.lazy()
            .with_column((col("headshots").cast(DataType::Float32) / 
                      col("shots").cast(DataType::Float32)).alias("test_ratio"))
            .collect()
            .expect("Failed to compute test_ratio");
            
        // Convert to ndarray
        let features = df_to_ndarray(&df, &["test_ratio"]).expect("Failed to convert");
        
        // Verify dimensions
        assert_eq!(features.shape(), [2, 1]);
        
        // Verify values with some tolerance for floating-point precision
        let expected_normal = 10.0 / 100.0;
        let expected_suspicious = 50.0 / 150.0;
        
        let tolerance = 1e-5;
        assert!((features[[0, 0]] - expected_normal).abs() < tolerance);
        assert!((features[[1, 0]] - expected_suspicious).abs() < tolerance);
    }
}