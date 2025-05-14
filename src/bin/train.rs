use nocheat::types::PlayerStats;
use nocheat::{generate_default_model, train_model};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process;

fn print_usage() {
    println!("NoCheat Model Trainer");
    println!("Usage:");
    println!("  train default <output_path>               Generate a default model");
    println!("  train custom <training_data> <output_path> Train a model with custom data");
    println!();
    println!("Examples:");
    println!("  train default cheat_model.bin");
    println!("  train custom training_data.json cheat_model.bin");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "default" => {
            if args.len() != 3 {
                print_usage();
                process::exit(1);
            }

            let output_path = &args[2];
            println!("Generating default model at: {}", output_path);

            if let Err(e) = generate_default_model(output_path) {
                eprintln!("Error generating default model: {}", e);
                process::exit(1);
            }

            println!("Default model successfully generated!");
        }

        "custom" => {
            if args.len() != 4 {
                print_usage();
                process::exit(1);
            }

            let training_data_path = &args[2];
            let output_path = &args[3];

            println!("Loading training data from: {}", training_data_path);

            // Read the training data JSON file
            let file = File::open(training_data_path)?;
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents)?;

            // Parse the JSON into PlayerStats and labels
            let training_data: Vec<PlayerStats> = match serde_json::from_str(&contents) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error parsing training data: {}", e);
                    process::exit(1);
                }
            };

            // Extract labels from the training data
            let labels: Vec<f64> = training_data
                .iter()
                .filter_map(|stat| stat.training_label)
                .collect();

            // Verify we have valid training data
            if training_data.len() != labels.len() {
                eprintln!("Error: Some PlayerStats objects are missing training labels");
                process::exit(1);
            }

            if training_data.is_empty() {
                eprintln!("Error: No training data found");
                process::exit(1);
            }

            // Train the model
            println!(
                "Training model with {} examples ({} labeled)...",
                training_data.len(),
                labels.len()
            );

            if let Err(e) = train_model(training_data, labels, output_path) {
                eprintln!("Error training model: {}", e);
                process::exit(1);
            }

            println!("Model successfully trained and saved to: {}", output_path);
        }

        _ => {
            print_usage();
            process::exit(1);
        }
    }

    Ok(())
}
