/// Generate test statistics for testing adaptive mode
/// Run with: cargo run --example create_test_stats
use std::collections::HashMap;
use std::time::SystemTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct SessionRecord {
    timestamp: String,
    lesson_type: String,
    wpm: f64,
    accuracy: f64,
    duration: u128,
}

#[derive(Serialize, Deserialize)]
struct KeyStats {
    key: char,
    total_attempts: usize,
    correct_attempts: usize,
    error_count: usize,
    total_time_ms: u64,
    mistype_map: HashMap<char, usize>,
    last_practiced: Option<SystemTime>,
    mastery_level: String,
}

#[derive(Serialize, Deserialize)]
struct BigramStats {
    bigram: String,
    total_attempts: usize,
    correct_attempts: usize,
    total_time_ms: u64,
    last_practiced: Option<SystemTime>,
}

#[derive(Serialize, Deserialize)]
struct AdaptiveAnalytics {
    key_stats: HashMap<char, KeyStats>,
    bigram_stats: HashMap<String, BigramStats>,
    session_history: Vec<serde_json::Value>,
    total_sessions: usize,
    total_keystrokes: usize,
}

#[derive(Serialize, Deserialize)]
struct Stats {
    sessions: Vec<SessionRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adaptive_analytics: Option<AdaptiveAnalytics>,
}

fn main() {
    println!("Creating test statistics for adaptive mode...\n");

    // Create 15 test sessions
    let mut sessions = Vec::new();
    for i in 0..15 {
        sessions.push(SessionRecord {
            timestamp: Utc::now().to_rfc3339(),
            lesson_type: format!("Home Row - Level {}", (i % 6) + 1),
            wpm: 35.0 + (i as f64 * 2.0),
            accuracy: 85.0 + (i as f64 * 0.5),
            duration: 60000,
        });
    }

    // Create key stats with varied performance
    let mut key_stats = HashMap::new();
    let now = SystemTime::now();

    // Strong keys (> 95% accuracy) - f, j, h
    for (key, attempts) in [('f', 100), ('j', 110), ('h', 95)] {
        let correct = (attempts as f64 * 0.97) as usize;
        key_stats.insert(
            key,
            KeyStats {
                key,
                total_attempts: attempts,
                correct_attempts: correct,
                error_count: attempts - correct,
                total_time_ms: 10000,
                mistype_map: HashMap::new(),
                last_practiced: Some(now),
                mastery_level: "Mastered".to_string(),
            },
        );
    }

    // Moderate keys (85-90% accuracy) - d, k, g
    for (key, attempts) in [('d', 80), ('k', 85), ('g', 90)] {
        let correct = (attempts as f64 * 0.88) as usize;
        key_stats.insert(
            key,
            KeyStats {
                key,
                total_attempts: attempts,
                correct_attempts: correct,
                error_count: attempts - correct,
                total_time_ms: 12000,
                mistype_map: HashMap::new(),
                last_practiced: Some(now),
                mastery_level: "Proficient".to_string(),
            },
        );
    }

    // WEAK keys (< 75% accuracy) - s, l, q, m - THESE SHOULD BE TARGETED!
    for (key, attempts) in [('s', 70), ('l', 75), ('q', 65), ('m', 80)] {
        let correct = (attempts as f64 * 0.68) as usize;
        let mut mistype_map = HashMap::new();
        mistype_map.insert('a', 3);
        mistype_map.insert('z', 2);

        key_stats.insert(
            key,
            KeyStats {
                key,
                total_attempts: attempts,
                correct_attempts: correct,
                error_count: attempts - correct,
                total_time_ms: 18000,
                mistype_map,
                last_practiced: Some(now),
                mastery_level: "Learning".to_string(),
            },
        );
    }

    // Create some bigram stats
    let mut bigram_stats = HashMap::new();
    bigram_stats.insert(
        "fj".to_string(),
        BigramStats {
            bigram: "fj".to_string(),
            total_attempts: 45,
            correct_attempts: 42,
            total_time_ms: 4500,
            last_practiced: Some(now),
        },
    );
    bigram_stats.insert(
        "dk".to_string(),
        BigramStats {
            bigram: "dk".to_string(),
            total_attempts: 30,
            correct_attempts: 24,
            total_time_ms: 5200,
            last_practiced: Some(now),
        },
    );

    let total_keystrokes: usize = key_stats.values().map(|ks| ks.total_attempts).sum();

    let stats = Stats {
        sessions,
        adaptive_analytics: Some(AdaptiveAnalytics {
            key_stats,
            bigram_stats,
            session_history: Vec::new(),
            total_sessions: 15,
            total_keystrokes,
        }),
    };

    // Write to config directory
    let mut config_dir = PathBuf::from(std::env::var("HOME").unwrap());
    config_dir.push(".config");
    config_dir.push("typer-cli");

    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    let stats_path = config_dir.join("stats.json");
    let json = serde_json::to_string_pretty(&stats).expect("Failed to serialize stats");
    fs::write(&stats_path, json).expect("Failed to write stats file");

    println!("✓ Created test stats at: {:?}", stats_path);
    println!("\n📊 Statistics Summary:");
    println!("  - 15 sessions completed");
    println!("  - {} keys tracked", stats.adaptive_analytics.as_ref().unwrap().key_stats.len());
    println!("  - Total keystrokes: {}", total_keystrokes);
    println!("\n🎯 Weak Keys (should be targeted by adaptive mode):");

    if let Some(analytics) = &stats.adaptive_analytics {
        for (key, key_stat) in &analytics.key_stats {
            let accuracy = (key_stat.correct_attempts as f64 / key_stat.total_attempts as f64) * 100.0;
            if accuracy < 75.0 {
                println!("  - '{}': {:.1}% accuracy ({}/{} correct)",
                    key, accuracy, key_stat.correct_attempts, key_stat.total_attempts);
            }
        }
    }

    println!("\n✅ Adaptive mode should now appear in the menu!");
    println!("   Run: cargo run");
}
