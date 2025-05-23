use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nocheat::types::{DefaultPlayerData, LegacyPlayerStats, PlayerStats};
use nocheat::{build_dataframe, df_to_ndarray, generate_default_model, train_model};
use polars::prelude::{col, DataType, IntoLazy};
use std::collections::HashMap;

fn make_dummy_stats(n: usize) -> Vec<LegacyPlayerStats> {
    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        let mut shots = HashMap::new();
        let mut hits = HashMap::new();

        // Vary the values slightly for each player
        let accuracy = 0.5 + (i % 40) as f32 * 0.01; // 50-90% accuracy
        let headshot_ratio = 0.1 + (i % 30) as f32 * 0.01; // 10-40% headshots

        // Create stats with different weapons
        shots.insert("rifle".to_string(), 100);
        shots.insert("pistol".to_string(), 50);

        hits.insert("rifle".to_string(), (100.0 * accuracy) as u32);
        hits.insert("pistol".to_string(), (50.0 * accuracy) as u32);

        let total_hits = (150.0 * accuracy) as u32;
        let headshots = (total_hits as f32 * headshot_ratio) as u32;

        let player_data = DefaultPlayerData {
            shots_fired: shots,
            hits,
            headshots,
            shot_timestamps_ms: None,
            training_label: None,
        };

        result.push(PlayerStats::new(format!("player_{}", i), player_data));
    }

    result
}

fn create_training_data(n: usize) -> (Vec<LegacyPlayerStats>, Vec<f64>) {
    let mut players = Vec::with_capacity(n);
    let mut labels = Vec::with_capacity(n);

    // Create half normal players
    for i in 0..(n / 2) {
        let mut shots = HashMap::new();
        let mut hits = HashMap::new();

        // Normal accuracy 40-60%
        let accuracy = 0.4 + (i % 20) as f32 * 0.01;

        shots.insert("rifle".to_string(), 100);
        hits.insert("rifle".to_string(), (100.0 * accuracy) as u32);

        let headshot_ratio = 0.1 + (i % 15) as f32 * 0.01; // 10-25% headshots
        let headshots = ((100.0 * accuracy) as f32 * headshot_ratio) as u32;

        let player_data = DefaultPlayerData {
            shots_fired: shots,
            hits,
            headshots,
            shot_timestamps_ms: None,
            training_label: None,
        };

        players.push(PlayerStats::new(format!("normal_{}", i), player_data));

        labels.push(0.0);
    } // Create half cheaters
    for i in 0..(n / 2) {
        let mut shots = HashMap::new();
        let mut hits = HashMap::new();

        // High accuracy 70-95%
        let accuracy = 0.7 + (i % 25) as f32 * 0.01;

        shots.insert("rifle".to_string(), 100);
        hits.insert("rifle".to_string(), (100.0 * accuracy) as u32);

        let headshot_ratio = 0.4 + (i % 40) as f32 * 0.01; // 40-80% headshots
        let headshots = ((100.0 * accuracy) as f32 * headshot_ratio) as u32;

        let player_data = DefaultPlayerData {
            shots_fired: shots,
            hits,
            headshots,
            shot_timestamps_ms: None,
            training_label: None,
        };

        players.push(PlayerStats::new(format!("cheater_{}", i), player_data));

        labels.push(1.0);
    }

    (players, labels)
}

fn bench_build_dataframe(c: &mut Criterion) {
    let stats = make_dummy_stats(1_000);
    c.bench_function("build_dataframe_1000", |b| {
        b.iter(|| {
            let _ = build_dataframe(black_box(&stats)).unwrap();
        })
    });
}

fn bench_df_to_ndarray(c: &mut Criterion) {
    let stats = make_dummy_stats(1_000);
    let df = build_dataframe(&stats).unwrap();
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
    c.bench_function("df_to_ndarray_1000", |b| {
        b.iter(|| {
            let _ = df_to_ndarray(black_box(&df), &["hit_rate", "headshot_rate"]).unwrap();
        })
    });
}

fn bench_train_model(c: &mut Criterion) {
    let (training_data, labels) = create_training_data(100);
    let temp_dir = std::env::temp_dir();
    let model_path = temp_dir.join("bench_model.bin");

    c.bench_function("train_model_100", |b| {
        b.iter(|| {
            let _ = train_model(
                black_box(training_data.clone()),
                black_box(labels.clone()),
                black_box(model_path.to_str().unwrap()),
            )
            .unwrap();
        })
    });

    // Clean up after benchmark
    let _ = std::fs::remove_file(&model_path);
}

fn bench_generate_default_model(c: &mut Criterion) {
    let temp_dir = std::env::temp_dir();
    let model_path = temp_dir.join("bench_default_model.bin");

    c.bench_function("generate_default_model", |b| {
        b.iter(|| {
            let _ = generate_default_model(black_box(model_path.to_str().unwrap())).unwrap();
        })
    });

    // Clean up after benchmark
    let _ = std::fs::remove_file(&model_path);
}

criterion_group!(
    benches,
    bench_build_dataframe,
    bench_df_to_ndarray,
    bench_train_model,
    bench_generate_default_model
);
criterion_main!(benches);
