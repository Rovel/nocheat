use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nocheat::{build_dataframe, df_to_ndarray};
use nocheat::types::PlayerStats;
use std::collections::HashMap;
use polars::prelude::{col, IntoLazy, DataType};

fn make_dummy_stats(n: usize) -> Vec<PlayerStats> {
    let mut vec = Vec::with_capacity(n);
    for i in 0..n {
        let mut shots = HashMap::new();
        shots.insert("rifle".to_string(), 100);
        let mut hits = HashMap::new();
        hits.insert("rifle".to_string(), 50);
        vec.push(PlayerStats {
            player_id: format!("player{}", i),
            shots_fired: shots.clone(),
            hits: hits.clone(),
            headshots: 10,
            shot_timestamps_ms: None,
        });
    }
    vec
}

fn bench_build_dataframe(c: &mut Criterion) {
    let stats = make_dummy_stats(1_000);
    c.bench_function("build_dataframe_1000", |b| b.iter(|| {
        let _ = build_dataframe(black_box(&stats)).unwrap();
    }));
}

fn bench_df_to_ndarray(c: &mut Criterion) {
    let stats = make_dummy_stats(1_000);
    let df = build_dataframe(&stats).unwrap();
    let df = df.lazy()
        .with_column((col("hits").cast(DataType::Float32) / col("shots").cast(DataType::Float32)).alias("hit_rate"))
        .with_column((col("headshots").cast(DataType::Float32) / col("hits").cast(DataType::Float32)).alias("headshot_rate"))
        .collect()
        .unwrap();
    c.bench_function("df_to_ndarray_1000", |b| b.iter(|| {
        let _ = df_to_ndarray(black_box(&df), &["hit_rate", "headshot_rate"]).unwrap();
    }));
}

criterion_group!(benches, bench_build_dataframe, bench_df_to_ndarray);
criterion_main!(benches);