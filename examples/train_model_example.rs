// filepath: examples/train_model_example.rs
// Example demonstrating how to generate a default model and train a custom model
use nocheat::types::LegacyPlayerStats;
use nocheat::{generate_default_model, train_model};
use serde_json;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// Real example code moved into try_main
fn try_main() -> Result<(), Box<dyn Error>> {
    // Generate a default model
    let default_model_path = "default_cheat_model.bin";
    println!("Generating default model at '{}'...", default_model_path);
    generate_default_model(default_model_path)?;
    assert!(Path::new(default_model_path).exists());
    println!("Default model generated successfully.\n");

    // Train a custom model using flat training_data.json (DefaultPlayerData schema)
    let training_data_path = "examples/json_samples/training_data.json";
    let custom_model_path = "custom_cheat_model.bin";
    println!(
        "Training custom model from '{}' into '{}'...",
        training_data_path, custom_model_path
    );

    // Read and parse training data
    let contents = fs::read_to_string(training_data_path)?;
    let training_stats: Vec<LegacyPlayerStats> = serde_json::from_str(&contents)?;
    let labels: Vec<f64> = training_stats
        .iter()
        .filter_map(|stat| stat.data.training_label)
        .collect();

    // Verify data-label alignment
    if training_stats.len() != labels.len() {
        return Err("Mismatch between training data and labels".into());
    }

    // Train and save the model
    train_model(training_stats, labels, custom_model_path)?;
    assert!(Path::new(custom_model_path).exists());
    println!("Custom model generated successfully.");

    Ok(())
}
