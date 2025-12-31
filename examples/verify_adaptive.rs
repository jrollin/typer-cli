use serde_json::Value;
use std::fs;
/// Verify that adaptive mode would be available in the menu
/// Run with: cargo run --example verify_adaptive
use std::path::PathBuf;

fn main() {
    println!("🔍 Verifying Adaptive Mode Configuration\n");

    // Load stats
    let mut stats_path = PathBuf::from(std::env::var("HOME").unwrap());
    stats_path.push(".config/typer-cli/stats.json");

    if !stats_path.exists() {
        println!("❌ No stats file found at {:?}", stats_path);
        println!("   Run: cargo run --example create_test_stats");
        return;
    }

    let json = fs::read_to_string(&stats_path).expect("Failed to read stats");
    let stats: Value = serde_json::from_str(&json).expect("Failed to parse stats");

    // Check sessions
    let sessions = stats["sessions"].as_array().unwrap();
    println!("📊 Sessions: {}", sessions.len());

    // Check adaptive analytics
    if let Some(analytics) = stats.get("adaptive_analytics") {
        let total_sessions = analytics["total_sessions"].as_u64().unwrap_or(0);
        let total_keystrokes = analytics["total_keystrokes"].as_u64().unwrap_or(0);

        println!("📈 Adaptive Analytics:");
        println!("   - Total sessions: {}", total_sessions);
        println!("   - Total keystrokes: {}", total_keystrokes);

        // Check if adaptive mode would be shown
        let should_show = total_sessions >= 10 && total_keystrokes >= 100;

        if should_show {
            println!("\n✅ ADAPTIVE MODE WILL BE SHOWN IN MENU!");
        } else {
            println!("\n⚠️  Adaptive mode NOT shown (need >= 10 sessions and >= 100 keystrokes)");
        }

        // Show key stats
        if let Some(key_stats) = analytics["key_stats"].as_object() {
            println!("\n🎯 Key Statistics ({} keys tracked):", key_stats.len());

            let mut weak_keys = Vec::new();
            let mut moderate_keys = Vec::new();
            let mut strong_keys = Vec::new();

            for (key, stats) in key_stats {
                let total = stats["total_attempts"].as_u64().unwrap_or(0) as f64;
                let correct = stats["correct_attempts"].as_u64().unwrap_or(0) as f64;
                let accuracy = if total > 0.0 {
                    (correct / total) * 100.0
                } else {
                    0.0
                };

                if accuracy < 80.0 && total >= 10.0 {
                    weak_keys.push((key, accuracy));
                } else if (80.0..95.0).contains(&accuracy) {
                    moderate_keys.push((key, accuracy));
                } else if accuracy >= 95.0 {
                    strong_keys.push((key, accuracy));
                }
            }

            if !weak_keys.is_empty() {
                println!("\n   Weak keys (< 80% - WILL BE TARGETED):");
                weak_keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                for (key, acc) in &weak_keys {
                    println!("     • '{}': {:.1}%", key, acc);
                }
            }

            if !moderate_keys.is_empty() {
                println!("\n   Moderate keys (80-95%):");
                for (key, acc) in moderate_keys.iter().take(3) {
                    println!("     • '{}': {:.1}%", key, acc);
                }
            }

            if !strong_keys.is_empty() {
                println!("\n   Strong keys (>= 95%):");
                for (key, acc) in strong_keys.iter().take(3) {
                    println!("     • '{}': {:.1}%", key, acc);
                }
            }

            // Predict content distribution
            if should_show && !weak_keys.is_empty() {
                println!("\n📝 Predicted Adaptive Content:");
                println!(
                    "   - 60% practice on: {:?}",
                    weak_keys.iter().map(|(k, _)| k).collect::<Vec<_>>()
                );
                println!("   - 30% practice on moderate keys");
                println!("   - 10% practice on strong keys (retention)");
            }
        }
    } else {
        println!("❌ No adaptive analytics found in stats file");
        println!("   The app will create analytics automatically after sessions are completed");
    }

    println!("\n✨ To test in the app:");
    println!("   1. Run: cargo run");
    println!("   2. Look for 'Adaptive Mode' at the top of the lesson menu");
    println!("   3. Select it and press Enter");
    println!("   4. The content will focus on your weakest keys!");
}
