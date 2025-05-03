use nocheat::types::{AnalysisResponse, PlayerStats};
use nocheat::{analyze_stats, build_dataframe, df_to_ndarray, generate_default_model, train_model};
use polars::prelude::{col, DataType, IntoLazy};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn make_dummy_stats() -> Vec<PlayerStats> {
    let mut shots = HashMap::new();
    shots.insert("rifle".to_string(), 100);
    let mut hits = HashMap::new();
    hits.insert("rifle".to_string(), 50);

    vec![PlayerStats {
        player_id: "player1".to_string(),
        shots_fired: shots,
        hits,
        headshots: 10,
        shot_timestamps_ms: None,
        training_label: None,
    }]
}

#[test]
fn test_build_dataframe() {
    let stats = make_dummy_stats();
    let df = build_dataframe(&stats).expect("DataFrame creation failed");
    assert_eq!(df.height(), 1);
    assert_eq!(df.column("shots").unwrap().u32().unwrap().get(0), Some(100));
    assert_eq!(df.column("hits").unwrap().u32().unwrap().get(0), Some(50));
    assert_eq!(
        df.column("headshots").unwrap().u32().unwrap().get(0),
        Some(10)
    );
}

#[test]
fn test_df_to_ndarray() {
    let stats = make_dummy_stats();
    let df = build_dataframe(&stats).unwrap();
    // first add computed features
    let df = df
        .lazy()
        .with_column(
            (col("hits").cast(DataType::Float32) / col("shots").cast(DataType::Float32))
                .alias("hit_rate"),
        )
        .with_column(
            (col("headshots").cast(DataType::Float32) / col("hits").cast(DataType::Float32))
                .alias("headshot_rate"),
        )
        .collect()
        .unwrap();
    let arr = df_to_ndarray(&df, &["hit_rate", "headshot_rate"]).expect("ndarray conversion");
    assert_eq!(arr.shape(), [1, 2]);
    let hr = arr[[0, 0]];
    let hrate = 50.0 / 100.0;
    assert!((hr - hrate).abs() < 1e-6);
}

#[test]
fn test_training_workflow() {
    // Create a temporary file path for the model
    let temp_dir = std::env::temp_dir();
    let model_path = temp_dir.join("test_workflow_model.bin");

    // 1. Generate training data (both normal and cheating players)
    let mut training_data = Vec::new();
    let mut labels = Vec::new();

    // Create several normal players with typical stats
    for i in 0..10 {
        let mut shots = HashMap::new();
        let mut hits = HashMap::new();

        // Normal accuracy 40-60%
        let accuracy = 0.4 + (i % 20) as f32 * 0.01;
        let shot_count = 100;
        let hit_count = (shot_count as f32 * accuracy) as u32;

        shots.insert("rifle".to_string(), shot_count);
        hits.insert("rifle".to_string(), hit_count);

        // Normal headshot ratio 10-20%
        let headshot_ratio = 0.1 + (i % 10) as f32 * 0.01;
        let headshots = (hit_count as f32 * headshot_ratio) as u32;

        training_data.push(PlayerStats {
            player_id: format!("normal_{}", i),
            shots_fired: shots,
            hits: hits,
            headshots,
            shot_timestamps_ms: None,
            training_label: Some(0.0),
        });

        labels.push(0.0); // Not a cheater
    }

    // Create several cheating players with suspicious stats
    for i in 0..10 {
        let mut shots = HashMap::new();
        let mut hits = HashMap::new();

        // Very high accuracy 80-95%
        let accuracy = 0.8 + (i % 15) as f32 * 0.01;
        let shot_count = 100;
        let hit_count = (shot_count as f32 * accuracy) as u32;

        shots.insert("rifle".to_string(), shot_count);
        hits.insert("rifle".to_string(), hit_count);

        // High headshot ratio 40-70%
        let headshot_ratio = 0.4 + (i % 30) as f32 * 0.01;
        let headshots = (hit_count as f32 * headshot_ratio) as u32;

        training_data.push(PlayerStats {
            player_id: format!("cheater_{}", i),
            shots_fired: shots,
            hits: hits,
            headshots,
            shot_timestamps_ms: None,
            training_label: Some(1.0),
        });

        labels.push(1.0); // Labeled as a cheater
    }

    // 2. Train the model
    let result = train_model(training_data, labels, model_path.to_str().unwrap());
    println!("Training model result: {:?}", result);
    assert!(result.is_ok(), "Model training failed");
    assert!(model_path.exists(), "Model file was not created");

    // Verify the file was created properly
    let file_size = std::fs::metadata(&model_path).map(|m| m.len()).unwrap_or(0);
    println!("Model file size: {} bytes", file_size);

    // 3. Create test players (one normal, one suspicious)
    let mut test_normal = HashMap::new();
    test_normal.insert("rifle".to_string(), 100);
    let mut hits_normal = HashMap::new();
    hits_normal.insert("rifle".to_string(), 50); // 50% accuracy

    let normal_player = PlayerStats {
        player_id: "test_normal".to_string(),
        shots_fired: test_normal,
        hits: hits_normal,
        headshots: 10, // 20% headshot ratio
        shot_timestamps_ms: None,
        training_label: None,
    };

    let mut test_suspicious = HashMap::new();
    test_suspicious.insert("rifle".to_string(), 100);
    let mut hits_suspicious = HashMap::new();
    hits_suspicious.insert("rifle".to_string(), 90); // 90% accuracy

    let suspicious_player = PlayerStats {
        player_id: "test_suspicious".to_string(),
        shots_fired: test_suspicious,
        hits: hits_suspicious,
        headshots: 70, // 78% headshot ratio
        shot_timestamps_ms: None,
        training_label: None,
    };

    // Save the original model file path if it exists, so we can restore it after the test
    let original_model_exists = std::path::Path::new("cheat_model.bin").exists();
    let backup_path = temp_dir.join("backup_cheat_model.bin");
    if original_model_exists {
        println!("Original model exists, creating backup");
        // Backup the existing model
        fs::copy("cheat_model.bin", &backup_path).expect("Failed to backup model");
    }

    // Copy our test model to the expected location
    println!("Copying test model to cheat_model.bin");
    fs::copy(&model_path, "cheat_model.bin").expect("Failed to copy test model");

    println!(
        "Checking if cheat_model.bin exists: {}",
        std::path::Path::new("cheat_model.bin").exists()
    );
    let file_size = std::fs::metadata("cheat_model.bin")
        .map(|m| m.len())
        .unwrap_or(0);
    println!("cheat_model.bin file size: {} bytes", file_size);

    // 4. Analyze the test players
    let test_stats = vec![normal_player, suspicious_player];
    let result = analyze_stats(test_stats);

    if let Err(ref e) = result {
        println!("Analysis error: {}", e);
    }

    assert!(result.is_ok(), "Analysis failed");

    let analysis = result.unwrap();
    assert_eq!(analysis.results.len(), 2, "Expected 2 analysis results");

    // 5. Verify the results - normal player should have low score, suspicious high score
    let normal_score = analysis.results[0].suspicion_score;
    let suspicious_score = analysis.results[1].suspicion_score;

    println!("Normal player score: {}", normal_score);
    println!("Suspicious player score: {}", suspicious_score);

    // Normal player should have a lower suspicion score than the suspicious player
    assert!(
        normal_score < suspicious_score,
        "Expected normal player ({}) to have a lower score than suspicious player ({})",
        normal_score,
        suspicious_score
    );

    // Typically scores are between 0-1, but might depend on the model
    assert!(
        suspicious_score > 0.5,
        "Expected suspicious player to have a high suspicion score, got {}",
        suspicious_score
    );

    // 6. Cleanup - restore the original model if it existed
    fs::remove_file("cheat_model.bin").expect("Failed to remove test model");
    if original_model_exists {
        fs::copy(&backup_path, "cheat_model.bin").expect("Failed to restore original model");
        fs::remove_file(&backup_path).expect("Failed to remove backup model");
    }

    // Remove the test model file
    let _ = fs::remove_file(&model_path);
}

#[test]
fn test_generate_default_model() {
    // Create a temporary file path for the model
    let temp_dir = std::env::temp_dir();
    let model_path = temp_dir.join("test_default_model.bin");

    // Generate the default model
    let result = generate_default_model(model_path.to_str().unwrap());
    println!("Default model generation result: {:?}", result);
    assert!(result.is_ok(), "Default model generation failed");
    assert!(model_path.exists(), "Default model file was not created");

    // Verify the file was created properly
    let file_size = std::fs::metadata(&model_path).map(|m| m.len()).unwrap_or(0);
    println!("Generated model file size: {} bytes", file_size);

    // Create a suspicious player for testing
    let mut shots = HashMap::new();
    shots.insert("rifle".to_string(), 100);
    let mut hits = HashMap::new();
    hits.insert("rifle".to_string(), 95); // 95% accuracy

    let suspicious_player = PlayerStats {
        player_id: "suspicious".to_string(),
        shots_fired: shots,
        hits: hits,
        headshots: 80, // 84% headshot ratio (very suspicious)
        shot_timestamps_ms: None,
        training_label: None,
    };

    // Save the original model file path if it exists, so we can restore it after the test
    let original_model_exists = std::path::Path::new("cheat_model.bin").exists();
    let backup_path = temp_dir.join("backup_cheat_model.bin");
    if original_model_exists {
        println!("Original model exists, creating backup");
        // Backup the existing model
        fs::copy("cheat_model.bin", &backup_path).expect("Failed to backup model");
    }

    // Copy our test model to the expected location
    println!("Copying test model to cheat_model.bin");
    fs::copy(&model_path, "cheat_model.bin").expect("Failed to copy test model");

    println!(
        "Checking if cheat_model.bin exists: {}",
        std::path::Path::new("cheat_model.bin").exists()
    );
    let file_size = std::fs::metadata("cheat_model.bin")
        .map(|m| m.len())
        .unwrap_or(0);
    println!("cheat_model.bin file size: {} bytes", file_size);

    // Run analysis with the default model
    let analysis = analyze_stats(vec![suspicious_player]);

    if let Err(ref e) = analysis {
        println!("Analysis error: {}", e);
    }

    assert!(analysis.is_ok(), "Analysis with default model failed");

    let result = analysis.unwrap();
    let score = result.results[0].suspicion_score;
    println!("Suspicious player score: {}", score);

    // With such suspicious stats, the score should be high
    assert!(
        score > 0.7,
        "Expected suspicious player to have a high score with default model, got {}",
        score
    );

    // Cleanup - restore the original model if it existed
    fs::remove_file("cheat_model.bin").expect("Failed to remove test model");
    if original_model_exists {
        fs::copy(&backup_path, "cheat_model.bin").expect("Failed to restore original model");
        fs::remove_file(&backup_path).expect("Failed to remove backup model");
    }

    // Remove the test model file
    let _ = fs::remove_file(&model_path);
}
