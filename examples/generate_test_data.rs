use chrono::Utc;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("🔧 Generating test analytics data...");

    // Create config directory
    let home = std::env::var("HOME").expect("$HOME not set");
    let config_dir = PathBuf::from(home).join(".config").join("typer-cli");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    let stats_file = config_dir.join("stats.json");

    // Generate test data with realistic progression
    let test_data = json!({
        "sessions": [
            // Early sessions - lower performance
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "HomeRow-1",
                "wpm": 35.0,
                "accuracy": 85.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "HomeRow-2",
                "wpm": 38.0,
                "accuracy": 87.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "HomeRow-3",
                "wpm": 40.0,
                "accuracy": 88.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            // Middle sessions - improving performance
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "Bigram-1",
                "wpm": 42.0,
                "accuracy": 89.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "Bigram-2",
                "wpm": 45.0,
                "accuracy": 90.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "Bigram-3",
                "wpm": 48.0,
                "accuracy": 91.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            // Recent sessions - best performance
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "Code-1",
                "wpm": 50.0,
                "accuracy": 92.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "Code-2",
                "wpm": 55.0,
                "accuracy": 93.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "Code-3",
                "wpm": 60.0,
                "accuracy": 94.0,
                "duration": 60000,
                "duration_limit": 300000
            },
            {
                "timestamp": Utc::now().to_rfc3339(),
                "lesson_type": "Code-4",
                "wpm": 65.0,
                "accuracy": 95.0,
                "duration": 60000,
                "duration_limit": 300000
            }
        ],
        "adaptive_analytics": {
            "key_stats": {
                "f": {
                    "key": "f",
                    "total_attempts": 100,
                    "correct_attempts": 98,
                    "error_count": 2,
                    "total_time_ms": 5000,
                    "mistype_map": {"d": 1, "g": 1},
                    "last_practiced": null,
                    "mastery_level": "Mastered"
                },
                "j": {
                    "key": "j",
                    "total_attempts": 100,
                    "correct_attempts": 97,
                    "error_count": 3,
                    "total_time_ms": 5500,
                    "mistype_map": {"h": 2, "k": 1},
                    "last_practiced": null,
                    "mastery_level": "Mastered"
                },
                "d": {
                    "key": "d",
                    "total_attempts": 80,
                    "correct_attempts": 75,
                    "error_count": 5,
                    "total_time_ms": 6000,
                    "mistype_map": {"s": 3, "f": 2},
                    "last_practiced": null,
                    "mastery_level": "Proficient"
                },
                "k": {
                    "key": "k",
                    "total_attempts": 85,
                    "correct_attempts": 78,
                    "error_count": 7,
                    "total_time_ms": 6500,
                    "mistype_map": {"l": 4, "j": 3},
                    "last_practiced": null,
                    "mastery_level": "Proficient"
                }
            },
            "bigram_stats": {
                "fj": {
                    "bigram": "fj",
                    "total_attempts": 50,
                    "correct_attempts": 48,
                    "total_time_ms": 3000,
                    "last_practiced": null
                },
                "dk": {
                    "bigram": "dk",
                    "total_attempts": 30,
                    "correct_attempts": 25,
                    "total_time_ms": 2000,
                    "last_practiced": null
                }
            },
            "session_history": [],
            "total_sessions": 10,
            "total_keystrokes": 2000
        }
    });

    // Write the test data
    let json_string =
        serde_json::to_string_pretty(&test_data).expect("Failed to serialize test data");
    fs::write(&stats_file, json_string).expect("Failed to write test data");

    println!("✅ Generated test data with 10 sessions");
    println!("📊 WPM Range: 35-65 WPM (showing progression)");
    println!("🎯 Accuracy Range: 85-95% (showing improvement)");
    println!("📈 Total sessions: 10");
    println!("📝 Data saved to: {}", stats_file.display());
    println!("\n💡 To test the analytics:");
    println!("   1. Run: cargo run");
    println!("   2. Press 's' to view statistics");
    println!("   3. Press 'h' to see history with ASCII charts");
    println!("   4. Press 'd' to see detailed analytics");
    println!("   5. Press 'e' to test export functionality");
}
