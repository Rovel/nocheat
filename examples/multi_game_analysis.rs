// Example showing how to load and use different types of generic training data
use nocheat::types::{AnalysisResponse, Analyzable, PlayerResult, PlayerStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// --- FPS GAME DATA STRUCTURES ---
#[derive(Clone, Debug, Deserialize, Serialize)]
struct FpsPlayerData {
    kills: u32,
    deaths: u32,
    assists: u32,
    weapon_stats: HashMap<String, WeaponStats>,
    average_position_change: f32,
    camping_seconds: u32,
    round_duration_seconds: u32,
    team: String,
    training_label: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct WeaponStats {
    shots_fired: u32,
    hits: u32,
    headshots: u32,
    distance_meters: Vec<f32>,
}

// --- MOBA GAME DATA STRUCTURES ---
#[derive(Clone, Debug, Deserialize, Serialize)]
struct MobaPlayerData {
    champion: String,
    level: u32,
    gold_earned: u32,
    gold_spent: u32,
    kills: u32,
    deaths: u32,
    assists: u32,
    damage_stats: DamageStats,
    healing: u32,
    damage_mitigated: u32,
    vision_score: u32,
    crowd_control_score: u32,
    skill_timings: Vec<SkillTiming>,
    item_build_order: Vec<u32>,
    team: String,
    match_duration_seconds: u32,
    training_label: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct DamageStats {
    physical_damage_dealt: u32,
    magic_damage_dealt: u32,
    true_damage_dealt: u32,
    damage_to_champions: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SkillTiming {
    skill: String,
    timestamps_ms: Vec<u32>,
}

// --- BATTLE ROYALE GAME DATA STRUCTURES ---
#[derive(Clone, Debug, Deserialize, Serialize)]
struct BattleRoyalePlayerData {
    match_id: String,
    placement: u32,
    survival_time_seconds: u32,
    kills: u32,
    damage_dealt: u32,
    damage_taken: u32,
    revives: u32,
    distance_traveled: DistanceTraveled,
    loot_collected: LootCollected,
    weapon_stats: HashMap<String, BrWeaponStats>,
    hot_drop: bool,
    team_size: u32,
    training_label: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct DistanceTraveled {
    walking: u32,
    swimming: u32,
    driving: u32,
    flying: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct LootCollected {
    weapons: u32,
    ammo: u32,
    healing: u32,
    armor: u32,
    attachments: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BrWeaponStats {
    shots_fired: u32,
    hits: u32,
    headshots: u32,
    damage: u32,
}

// --- RACING GAME DATA STRUCTURES ---
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RacingPlayerData {
    track_id: String,
    car_model: String,
    finish_position: u32,
    finish_time_seconds: f32,
    best_lap_time_seconds: f32,
    average_lap_time_seconds: f32,
    lap_times: Vec<f32>,
    penalties: Penalties,
    telemetry: Telemetry,
    sectors: Vec<SectorTime>,
    pit_stops: u32,
    fuel_consumption: f32,
    tire_wear: TireWear,
    weather_conditions: String,
    training_label: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Penalties {
    corner_cutting: u32,
    speeding: u32,
    collision: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Telemetry {
    top_speed_kmh: f32,
    average_speed_kmh: f32,
    gear_changes: u32,
    braking_points: u32,
    acceleration_events: u32,
    drift_angles: Vec<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SectorTime {
    sector_id: u32,
    times: Vec<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TireWear {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

// --- RTS GAME DATA STRUCTURES ---
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RtsPlayerData {
    build_order: Vec<String>,
    units_trained: u32,
    units_destroyed: u32,
    resources_gathered: Resources,
    average_apm: u32,
    game_duration_seconds: u32,
    training_label: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Resources {
    minerals: u32,
    vespene: u32,
}

// --- ANALYSIS RESULT STRUCTURES ---
#[derive(Debug, PartialEq, Serialize)]
struct GenericAnalysisResult {
    cheating_probability: f32,
    suspected_cheats: Vec<String>,
    evidence_strength: String,
    anomaly_details: HashMap<String, String>,
}

// --- IMPLEMENT ANALYZABLE TRAIT ---
// For FPS data
impl Analyzable for FpsPlayerData {
    fn calculate_accuracy_rate(&self) -> f32 {
        let mut total_shots = 0;
        let mut total_hits = 0;

        for weapon in self.weapon_stats.values() {
            total_shots += weapon.shots_fired;
            total_hits += weapon.hits;
        }

        if total_shots == 0 {
            return 0.0;
        }

        total_hits as f32 / total_shots as f32
    }

    fn calculate_headshot_ratio(&self) -> f32 {
        let mut total_hits = 0;
        let mut total_headshots = 0;

        for weapon in self.weapon_stats.values() {
            total_hits += weapon.hits;
            total_headshots += weapon.headshots;
        }

        if total_hits == 0 {
            return 0.0;
        }

        total_headshots as f32 / total_hits as f32
    }

    fn extract_features(&self) -> Vec<f32> {
        let kd_ratio = if self.deaths == 0 {
            self.kills as f32
        } else {
            self.kills as f32 / self.deaths as f32
        };

        vec![
            self.calculate_accuracy_rate(),
            self.calculate_headshot_ratio(),
            kd_ratio,
            self.average_position_change,
            self.camping_seconds as f32 / self.round_duration_seconds as f32,
        ]
    }

    fn is_suspicious(&self) -> bool {
        let accuracy = self.calculate_accuracy_rate();
        let headshot_ratio = self.calculate_headshot_ratio();
        let kd_ratio = if self.deaths == 0 {
            self.kills as f32
        } else {
            self.kills as f32 / self.deaths as f32
        };

        accuracy > 0.8 || headshot_ratio > 0.7 || kd_ratio > 5.0
    }
}

// For Battle Royale data
impl Analyzable for BattleRoyalePlayerData {
    fn calculate_accuracy_rate(&self) -> f32 {
        let mut total_shots = 0;
        let mut total_hits = 0;

        for weapon in self.weapon_stats.values() {
            total_shots += weapon.shots_fired;
            total_hits += weapon.hits;
        }

        if total_shots == 0 {
            return 0.0;
        }

        total_hits as f32 / total_shots as f32
    }

    fn calculate_headshot_ratio(&self) -> f32 {
        let mut total_hits = 0;
        let mut total_headshots = 0;

        for weapon in self.weapon_stats.values() {
            total_hits += weapon.hits;
            total_headshots += weapon.headshots;
        }

        if total_hits == 0 {
            return 0.0;
        }

        total_headshots as f32 / total_hits as f32
    }

    fn extract_features(&self) -> Vec<f32> {
        vec![
            self.calculate_accuracy_rate(),
            self.calculate_headshot_ratio(),
            self.kills as f32,
            self.damage_dealt as f32 / self.damage_taken.max(1) as f32,
            self.placement as f32,
            self.survival_time_seconds as f32,
        ]
    }

    fn is_suspicious(&self) -> bool {
        self.calculate_accuracy_rate() > 0.9
            || self.calculate_headshot_ratio() > 0.8
            || (self.kills > 25 && self.placement <= 3)
    }
}

// For Racing data
impl Analyzable for RacingPlayerData {
    fn calculate_accuracy_rate(&self) -> f32 {
        // Racing games don't have a direct accuracy metric
        // Instead, we'll use consistency as a proxy
        if self.lap_times.is_empty() {
            return 0.0;
        }

        let average = self.average_lap_time_seconds;
        let deviation: f32 = self
            .lap_times
            .iter()
            .map(|&t| (t - average).abs())
            .sum::<f32>()
            / self.lap_times.len() as f32;

        // Return consistency (1.0 means perfect consistency)
        1.0 - (deviation / average).min(1.0)
    }

    fn calculate_headshot_ratio(&self) -> f32 {
        // Racing games don't have headshots
        // Instead, we'll use sector perfectness as a proxy
        if self.sectors.is_empty() {
            return 0.0;
        }

        let mut perfect_sectors = 0;
        let mut total_sectors = 0;

        for sector in &self.sectors {
            if !sector.times.is_empty() {
                let min_time = sector.times.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                let avg_time: f32 = sector.times.iter().sum::<f32>() / sector.times.len() as f32;

                // If minimum time is within 1% of average, count as "perfect"
                if (avg_time - min_time) / avg_time < 0.01 {
                    perfect_sectors += 1;
                }

                total_sectors += 1;
            }
        }

        if total_sectors == 0 {
            return 0.0;
        }

        perfect_sectors as f32 / total_sectors as f32
    }

    fn extract_features(&self) -> Vec<f32> {
        vec![
            self.best_lap_time_seconds,
            self.average_lap_time_seconds,
            self.calculate_accuracy_rate(), // consistency
            self.telemetry.top_speed_kmh,
            self.telemetry.average_speed_kmh,
            self.penalties.corner_cutting as f32,
            self.penalties.collision as f32,
        ]
    }

    fn is_suspicious(&self) -> bool {
        // Detect suspiciously fast times or perfect consistency
        if self.calculate_accuracy_rate() > 0.98 {
            return true; // Too perfect consistency
        }

        // Check if lap times are suspiciously fast (depends on the track)
        match self.track_id.as_str() {
            "Monaco" => self.best_lap_time_seconds < 75.0,
            "Silverstone" => self.best_lap_time_seconds < 85.0,
            "Monza" => self.best_lap_time_seconds < 75.0,
            "Spa" => self.best_lap_time_seconds < 80.0,
            _ => false,
        }
    }
}

// For RTS data
impl Analyzable for RtsPlayerData {
    fn calculate_accuracy_rate(&self) -> f32 {
        if self.units_trained == 0 {
            return 0.0;
        }
        self.units_destroyed as f32 / self.units_trained as f32
    }
    fn calculate_headshot_ratio(&self) -> f32 {
        // Use normalized APM as a proxy
        let duration = self.game_duration_seconds as f32;
        if duration == 0.0 {
            return 0.0;
        }
        self.average_apm as f32 / duration
    }
    fn extract_features(&self) -> Vec<f32> {
        vec![
            self.calculate_accuracy_rate(),
            self.calculate_headshot_ratio(),
            self.average_apm as f32,
            self.resources_gathered.minerals as f32,
            self.resources_gathered.vespene as f32,
        ]
    }
    fn is_suspicious(&self) -> bool {
        self.calculate_accuracy_rate() > 1.5 || self.calculate_headshot_ratio() > 2.0
    }
}

// --- IMPLEMENT ANALYZABLE TRAIT FOR MOBA DATA ---
impl Analyzable for MobaPlayerData {
    fn calculate_accuracy_rate(&self) -> f32 {
        if self.deaths == 0 {
            return self.kills as f32;
        }
        self.kills as f32 / self.deaths as f32
    }
    fn calculate_headshot_ratio(&self) -> f32 {
        if self.kills + self.assists == 0 {
            return 0.0;
        }
        self.assists as f32 / (self.kills + self.assists) as f32
    }
    fn extract_features(&self) -> Vec<f32> {
        vec![
            self.calculate_accuracy_rate(),
            self.calculate_headshot_ratio(),
            self.level as f32,
            self.gold_earned as f32 / self.gold_spent.max(1) as f32,
        ]
    }
    fn is_suspicious(&self) -> bool {
        self.calculate_accuracy_rate() > 5.0 || self.calculate_headshot_ratio() > 0.5
    }
}

// Custom wrapper struct for JSON deserialization
#[derive(Deserialize)]
struct PlayerDataWrapper<T> {
    player_id: String,
    data: T,
}

// Convert from wrapper to PlayerStats
fn convert_to_player_stats<T>(wrappers: Vec<PlayerDataWrapper<T>>) -> Vec<PlayerStats<T>>
where
    T: Clone + Serialize,
{
    wrappers
        .into_iter()
        .map(|w| PlayerStats::new(w.player_id, w.data))
        .collect()
}

// Functions to load different data types
fn load_json_data<T>(file_path: &str) -> Result<Vec<PlayerStats<T>>, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de> + Clone + Serialize,
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let wrappers: Vec<PlayerDataWrapper<T>> = serde_json::from_str(&contents)?;
    Ok(convert_to_player_stats(wrappers))
}

/// Analyze any Analyzable player data into GenericAnalysisResult
fn analyze_generic_data<T: Analyzable + Clone + Serialize>(
    players: &[PlayerStats<T>],
) -> AnalysisResponse<GenericAnalysisResult> {
    let mut results = Vec::new();
    for player in players {
        let accuracy = player.data.calculate_accuracy_rate();
        let headshot = player.data.calculate_headshot_ratio();
        let mut suspected_cheats = Vec::new();
        let mut anomaly_details = HashMap::new();
        if accuracy > 0.85 {
            suspected_cheats.push("High Efficiency".to_string());
            anomaly_details.insert(
                "Accuracy Rate".to_string(),
                format!("{:.1}%", accuracy * 100.0),
            );
        }
        if headshot > 0.75 {
            suspected_cheats.push("High APM or Consistency".to_string());
            anomaly_details.insert("Normalized APM".to_string(), format!("{:.1}", headshot));
        }
        let mut cheating_probability = 0.0;
        if !suspected_cheats.is_empty() {
            cheating_probability = (accuracy * 0.5 + headshot * 0.5).min(1.0);
        }
        let evidence_strength = if cheating_probability > 0.8 {
            "High".to_string()
        } else if cheating_probability > 0.5 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        };
        let result = GenericAnalysisResult {
            cheating_probability,
            suspected_cheats,
            evidence_strength,
            anomaly_details,
        };
        results.push(PlayerResult::new(player.player_id.clone(), result));
    }
    AnalysisResponse { results }
}

fn analyze_fps_data(
    players: &[PlayerStats<FpsPlayerData>],
) -> AnalysisResponse<GenericAnalysisResult> {
    let mut results = Vec::new();

    for player in players {
        let accuracy = player.data.calculate_accuracy_rate();
        let headshot_ratio = player.data.calculate_headshot_ratio();
        let kd_ratio = if player.data.deaths == 0 {
            player.data.kills as f32
        } else {
            player.data.kills as f32 / player.data.deaths as f32
        };

        let mut suspected_cheats = Vec::new();
        let mut anomaly_details = HashMap::new();

        // Check for suspicious patterns
        if accuracy > 0.8 {
            suspected_cheats.push("Aimbot".to_string());
            anomaly_details.insert(
                "High Accuracy".to_string(),
                format!("{:.1}% hit rate is suspiciously high", accuracy * 100.0),
            );
        }

        if headshot_ratio > 0.7 {
            suspected_cheats.push("Aimbot (Headshot)".to_string());
            anomaly_details.insert(
                "Headshot Anomaly".to_string(),
                format!(
                    "{:.1}% headshot ratio is suspiciously high",
                    headshot_ratio * 100.0
                ),
            );
        }

        if kd_ratio > 5.0 {
            suspected_cheats.push("Skill Anomaly".to_string());
            anomaly_details.insert(
                "K/D Ratio".to_string(),
                format!("K/D ratio of {:.1} is unusually high", kd_ratio),
            );
        }

        // Calculate overall cheating probability
        let mut cheating_probability = 0.0;
        if !suspected_cheats.is_empty() {
            cheating_probability =
                (accuracy * 0.3 + headshot_ratio * 0.5 + (kd_ratio / 10.0) * 0.2).min(1.0);
        }

        // Determine evidence strength
        let evidence_strength = if cheating_probability > 0.8 {
            "High".to_string()
        } else if cheating_probability > 0.5 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        };

        // Create the analysis result
        let result = GenericAnalysisResult {
            cheating_probability,
            suspected_cheats,
            evidence_strength,
            anomaly_details,
        };

        results.push(PlayerResult::new(player.player_id.clone(), result));
    }

    AnalysisResponse { results }
}

fn analyze_battle_royale_data(
    players: &[PlayerStats<BattleRoyalePlayerData>],
) -> AnalysisResponse<GenericAnalysisResult> {
    let mut results = Vec::new();

    for player in players {
        let accuracy = player.data.calculate_accuracy_rate();
        let headshot_ratio = player.data.calculate_headshot_ratio();
        let kills = player.data.kills as f32;
        let placement = player.data.placement as f32;

        let mut suspected_cheats = Vec::new();
        let mut anomaly_details = HashMap::new();

        // Check for suspicious patterns
        if accuracy > 0.9 {
            suspected_cheats.push("Aimbot".to_string());
            anomaly_details.insert(
                "High Accuracy".to_string(),
                format!("{:.1}% hit rate is suspiciously high", accuracy * 100.0),
            );
        }

        if headshot_ratio > 0.8 {
            suspected_cheats.push("Headshot Hack".to_string());
            anomaly_details.insert(
                "Headshot Anomaly".to_string(),
                format!(
                    "{:.1}% headshot ratio is suspiciously high",
                    headshot_ratio * 100.0
                ),
            );
        }

        if kills > 25.0 && placement <= 3.0 {
            suspected_cheats.push("Kill Anomaly".to_string());
            anomaly_details.insert(
                "High Kill Count".to_string(),
                format!("{} kills with placement {} is unusual", kills, placement),
            );
        }

        // Calculate overall cheating probability
        let mut cheating_probability = 0.0;
        if !suspected_cheats.is_empty() {
            cheating_probability = (accuracy * 0.3
                + headshot_ratio * 0.3
                + (1.0 - placement / 100.0) * 0.2
                + (kills / 40.0) * 0.2)
                .min(1.0);
        }

        // Determine evidence strength
        let evidence_strength = if cheating_probability > 0.8 {
            "High".to_string()
        } else if cheating_probability > 0.5 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        };

        // Create the analysis result
        let result = GenericAnalysisResult {
            cheating_probability,
            suspected_cheats,
            evidence_strength,
            anomaly_details,
        };

        results.push(PlayerResult::new(player.player_id.clone(), result));
    }

    AnalysisResponse { results }
}

fn analyze_racing_data(
    players: &[PlayerStats<RacingPlayerData>],
) -> AnalysisResponse<GenericAnalysisResult> {
    let mut results = Vec::new();

    for player in players {
        let consistency = player.data.calculate_accuracy_rate(); // Using accuracy as consistency
        let perfect_sectors = player.data.calculate_headshot_ratio(); // Using headshot ratio as perfect sectors
        let best_lap = player.data.best_lap_time_seconds;

        let mut suspected_cheats = Vec::new();
        let mut anomaly_details = HashMap::new();

        // Check for suspicious patterns based on track
        let too_fast = match player.data.track_id.as_str() {
            "Monaco" => best_lap < 75.0,
            "Silverstone" => best_lap < 85.0,
            "Monza" => best_lap < 75.0,
            "Spa" => best_lap < 80.0,
            _ => false,
        };

        if too_fast {
            suspected_cheats.push("Speed Hack".to_string());
            anomaly_details.insert(
                "Impossible Lap Time".to_string(),
                format!(
                    "Lap time of {:.2}s on {} is unrealistically fast",
                    best_lap, player.data.track_id
                ),
            );
        }

        if consistency > 0.98 {
            suspected_cheats.push("Bot Driver".to_string());
            anomaly_details.insert(
                "Perfect Consistency".to_string(),
                format!(
                    "Consistency of {:.1}% is unrealistically perfect",
                    consistency * 100.0
                ),
            );
        }

        if perfect_sectors > 0.9 {
            suspected_cheats.push("Perfect Racing Line".to_string());
            anomaly_details.insert(
                "Sector Perfection".to_string(),
                format!(
                    "{:.1}% of sectors were perfect - likely automated driving",
                    perfect_sectors * 100.0
                ),
            );
        }

        // Calculate overall cheating probability
        let mut cheating_probability = 0.0;
        if !suspected_cheats.is_empty() {
            // Different weights for racing games
            cheating_probability =
                (consistency * 0.4 + perfect_sectors * 0.4 + (too_fast as u8 as f32) * 0.2)
                    .min(1.0);
        }

        // Determine evidence strength
        let evidence_strength = if cheating_probability > 0.8 {
            "High".to_string()
        } else if cheating_probability > 0.5 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        };

        // Create the analysis result
        let result = GenericAnalysisResult {
            cheating_probability,
            suspected_cheats,
            evidence_strength,
            anomaly_details,
        };

        results.push(PlayerResult::new(player.player_id.clone(), result));
    }

    AnalysisResponse { results }
}

// Main function to demonstrate loading and analyzing different game data types
#[allow(unused_variables)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading and analyzing different game data types using generic structures\n");

    // --- FPS GAME ANALYSIS ---
    println!("==== FPS GAME ANALYSIS ====");
    let fps_data: Vec<PlayerStats<FpsPlayerData>> =
        load_json_data("examples/json_samples/fps_training_data.json")?;
    println!("Loaded {} FPS player records", fps_data.len());

    let fps_analysis = analyze_fps_data(&fps_data);

    for result in &fps_analysis.results {
        println!("\nPlayer: {}", result.player_id);
        println!(
            "Cheating Probability: {:.1}%",
            result.data.cheating_probability * 100.0
        );
        println!("Evidence Strength: {}", result.data.evidence_strength);

        if !result.data.suspected_cheats.is_empty() {
            println!("Suspected Cheats:");
            for cheat in &result.data.suspected_cheats {
                println!("  - {}", cheat);
            }
        }
    }

    // --- BATTLE ROYALE GAME ANALYSIS ---
    println!("\n==== BATTLE ROYALE GAME ANALYSIS ====");
    let br_data: Vec<PlayerStats<BattleRoyalePlayerData>> =
        load_json_data("examples/json_samples/battle_royale_training_data.json")?;
    println!("Loaded {} Battle Royale player records", br_data.len());

    let br_analysis = analyze_battle_royale_data(&br_data);

    for result in &br_analysis.results {
        println!("\nPlayer: {}", result.player_id);
        println!(
            "Cheating Probability: {:.1}%",
            result.data.cheating_probability * 100.0
        );
        println!("Evidence Strength: {}", result.data.evidence_strength);

        if !result.data.suspected_cheats.is_empty() {
            println!("Suspected Cheats:");
            for cheat in &result.data.suspected_cheats {
                println!("  - {}", cheat);
            }
        }
    }

    // --- RACING GAME ANALYSIS ---
    println!("\n==== RACING GAME ANALYSIS ====");
    let racing_data: Vec<PlayerStats<RacingPlayerData>> =
        load_json_data("examples/json_samples/racing_training_data.json")?;
    println!("Loaded {} Racing player records", racing_data.len());

    let racing_analysis = analyze_racing_data(&racing_data);

    for result in &racing_analysis.results {
        println!("\nPlayer: {}", result.player_id);
        println!(
            "Cheating Probability: {:.1}%",
            result.data.cheating_probability * 100.0
        );
        println!("Evidence Strength: {}", result.data.evidence_strength);

        if !result.data.suspected_cheats.is_empty() {
            println!("Suspected Cheats:");
            for cheat in &result.data.suspected_cheats {
                println!("  - {}", cheat);
            }
        }
    }

    // --- MOBA GAME ANALYSIS ---
    println!("\n==== MOBA GAME ANALYSIS ====");
    let moba_data: Vec<PlayerStats<MobaPlayerData>> =
        load_json_data("examples/json_samples/moba_training_data.json")?;
    println!("Loaded {} MOBA player records", moba_data.len());

    let moba_analysis = analyze_generic_data(&moba_data);

    for result in &moba_analysis.results {
        println!("\nPlayer: {}", result.player_id);
        println!(
            "Cheating Probability: {:.1}%",
            result.data.cheating_probability * 100.0
        );
        println!("Evidence Strength: {}", result.data.evidence_strength);

        if !result.data.suspected_cheats.is_empty() {
            println!("Suspected Cheats:");
            for cheat in &result.data.suspected_cheats {
                println!("  - {}", cheat);
            }
        }
    }

    // --- RTS GAME ANALYSIS ---
    println!("\n==== RTS GAME ANALYSIS ====");
    let rts_data: Vec<PlayerStats<RtsPlayerData>> =
        load_json_data("examples/json_samples/rts_training_data.json")?;
    println!("Loaded {} RTS player records", rts_data.len());

    let rts_analysis = analyze_generic_data(&rts_data);

    for result in &rts_analysis.results {
        println!("\nPlayer: {}", result.player_id);
        println!(
            "Cheating Probability: {:.1}%",
            result.data.cheating_probability * 100.0
        );
        println!("Evidence Strength: {}", result.data.evidence_strength);

        if !result.data.suspected_cheats.is_empty() {
            println!("Suspected Cheats:");
            for cheat in &result.data.suspected_cheats {
                println!("  - {}", cheat);
            }
        }
    }

    Ok(())
}
