use nocheat::{build_dataframe, df_to_ndarray};
use nocheat::types::PlayerStats;
use polars::prelude::{col, DataType, IntoLazy};
use std::collections::HashMap;

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
    }]
}

#[test]
fn test_build_dataframe() {
    let stats = make_dummy_stats();
    let df = build_dataframe(&stats).expect("DataFrame creation failed");
    assert_eq!(df.height(), 1);
    assert_eq!(df.column("shots").unwrap().u32().unwrap().get(0), Some(100));
    assert_eq!(df.column("hits").unwrap().u32().unwrap().get(0), Some(50));
    assert_eq!(df.column("headshots").unwrap().u32().unwrap().get(0), Some(10));
}

#[test]
fn test_df_to_ndarray() {
    let stats = make_dummy_stats();
    let df = build_dataframe(&stats).unwrap();
    // first add computed features
    let df = df.lazy()
        .with_column((col("hits").cast(DataType::Float32) / col("shots").cast(DataType::Float32)).alias("hit_rate"))
        .with_column((col("headshots").cast(DataType::Float32) / col("hits").cast(DataType::Float32)).alias("headshot_rate"))
        .collect()
        .unwrap();
    let arr = df_to_ndarray(&df, &["hit_rate", "headshot_rate"]).expect("ndarray conversion");
    assert_eq!(arr.shape(), [1, 2]);
    let hr = arr[[0, 0]];
    let hrate = 50.0 / 100.0;
    assert!((hr - hrate).abs() < 1e-6);
}