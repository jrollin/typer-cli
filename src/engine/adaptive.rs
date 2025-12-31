/// Adaptive algorithms for personalized training
/// Includes weakness detection for identifying problem areas
use super::analytics::AdaptiveAnalytics;

/// Weakness detector for identifying problem areas
/// Phase 3: Adaptive learning framework for analytics visualization and session feedback
/// Fully tested (lines 214-380) but currently only used in examples
#[allow(dead_code)]
pub struct WeaknessDetector;

impl WeaknessDetector {
    /// Identify weak keys based on accuracy threshold
    /// Returns up to 5 weakest keys that meet minimum attempts threshold
    pub fn identify_weak_keys(analytics: &AdaptiveAnalytics, threshold: f64) -> Vec<char> {
        let mut weak_keys: Vec<_> = analytics
            .key_stats
            .iter()
            .filter(|(_, stats)| {
                stats.total_attempts >= 10 && // Minimum data threshold
                stats.accuracy() < threshold // Below threshold
            })
            .map(|(key, stats)| (*key, stats.error_rate()))
            .collect();

        // Sort by error rate (worst first)
        weak_keys.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        weak_keys
            .into_iter()
            .take(5) // Top 5 weakest keys
            .map(|(key, _)| key)
            .collect()
    }

    /// Identify slow keys based on timing percentile
    /// Returns keys slower than the given percentile (e.g., 0.75 for top 25% slowest)
    pub fn identify_slow_keys(analytics: &AdaptiveAnalytics, percentile: f64) -> Vec<char> {
        let timings: Vec<_> = analytics
            .key_stats
            .iter()
            .filter(|(_, stats)| stats.correct_attempts >= 5)
            .map(|(key, stats)| (*key, stats.average_time_ms()))
            .collect();

        if timings.is_empty() {
            return vec![];
        }

        // Calculate percentile threshold
        let mut times: Vec<_> = timings.iter().map(|(_, t)| *t).collect();
        times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let threshold_idx = ((times.len() as f64 * percentile) as usize).min(times.len() - 1);
        let threshold = times[threshold_idx];

        // Return keys slower than threshold
        timings
            .into_iter()
            .filter(|(_, time)| *time > threshold)
            .map(|(key, _)| key)
            .collect()
    }

    /// Identify weak bigrams based on accuracy
    /// Returns up to 5 weakest bigrams with minimum attempts threshold
    /// Phase 3: Bigram weakness detection for future UI session feedback
    #[allow(dead_code)]
    pub fn identify_weak_bigrams(analytics: &AdaptiveAnalytics) -> Vec<String> {
        let mut weak_bigrams: Vec<_> = analytics
            .bigram_stats
            .iter()
            .filter(|(_, stats)| stats.total_attempts >= 5 && stats.accuracy() < 85.0)
            .map(|(bigram, stats)| (bigram.clone(), stats.accuracy()))
            .collect();

        weak_bigrams.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        weak_bigrams
            .into_iter()
            .take(5)
            .map(|(bigram, _)| bigram)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::analytics::{BigramStats, KeyStats};

    fn create_test_analytics() -> AdaptiveAnalytics {
        let mut analytics = AdaptiveAnalytics::default();

        // Add strong key (f: 96% accuracy)
        let mut f_stats = KeyStats::new('f');
        f_stats.total_attempts = 50;
        f_stats.correct_attempts = 48;
        analytics.key_stats.insert('f', f_stats);

        // Add weak key (d: 70% accuracy)
        let mut d_stats = KeyStats::new('d');
        d_stats.total_attempts = 50;
        d_stats.correct_attempts = 35;
        analytics.key_stats.insert('d', d_stats);

        // Add very weak key (s: 50% accuracy)
        let mut s_stats = KeyStats::new('s');
        s_stats.total_attempts = 50;
        s_stats.correct_attempts = 25;
        analytics.key_stats.insert('s', s_stats);

        analytics.total_sessions = 10;
        analytics.total_keystrokes = 500;

        analytics
    }

    #[test]
    fn test_identify_weak_keys() {
        let analytics = create_test_analytics();
        let weak_keys = WeaknessDetector::identify_weak_keys(&analytics, 80.0);

        // Should identify d and s as weak (both below 80%)
        assert!(weak_keys.contains(&'d'));
        assert!(weak_keys.contains(&'s'));
        // Should not include f (96% accuracy)
        assert!(!weak_keys.contains(&'f'));
    }

    #[test]
    fn test_identify_weak_keys_minimum_attempts() {
        let mut analytics = AdaptiveAnalytics::default();

        // Add key with low accuracy but insufficient attempts
        let mut k_stats = KeyStats::new('k');
        k_stats.total_attempts = 5; // Below minimum threshold of 10
        k_stats.correct_attempts = 2; // 40% accuracy
        analytics.key_stats.insert('k', k_stats);

        let weak_keys = WeaknessDetector::identify_weak_keys(&analytics, 80.0);

        // Should not include key with insufficient data
        assert!(!weak_keys.contains(&'k'));
    }

    #[test]
    fn test_identify_slow_keys() {
        let mut analytics = AdaptiveAnalytics::default();

        // Add several keys with varying speeds
        let keys_and_times = [
            ('a', 100), // fast
            ('b', 120),
            ('c', 150),
            ('d', 200), // medium
            ('e', 250),
            ('f', 300), // slow
            ('g', 350), // very slow
        ];

        for (key, time_ms) in keys_and_times {
            let mut stats = KeyStats::new(key);
            stats.correct_attempts = 10;
            stats.total_time_ms = time_ms;
            analytics.key_stats.insert(key, stats);
        }

        // Get keys slower than 75th percentile (top 25% slowest)
        let slow_keys = WeaknessDetector::identify_slow_keys(&analytics, 0.75);

        // The slowest keys (f, g) should be identified
        // With 7 keys, 75th percentile index = 5 (0-based), value = 300
        // Keys > 300 should be returned: 'g' (350)
        assert!(!slow_keys.is_empty());
        assert!(slow_keys.contains(&'g'));
    }

    #[test]
    fn test_identify_weak_bigrams() {
        let mut analytics = AdaptiveAnalytics::default();

        // Add strong bigram
        let mut fj_stats = BigramStats::new("fj".to_string());
        fj_stats.total_attempts = 20;
        fj_stats.correct_attempts = 19; // 95% accuracy
        analytics.bigram_stats.insert("fj".to_string(), fj_stats);

        // Add weak bigram
        let mut dk_stats = BigramStats::new("dk".to_string());
        dk_stats.total_attempts = 20;
        dk_stats.correct_attempts = 15; // 75% accuracy
        analytics.bigram_stats.insert("dk".to_string(), dk_stats);

        let weak_bigrams = WeaknessDetector::identify_weak_bigrams(&analytics);

        // Should identify dk as weak (below 85%)
        assert!(weak_bigrams.contains(&"dk".to_string()));
        // Should not include fj (95% accuracy)
        assert!(!weak_bigrams.contains(&"fj".to_string()));
    }
}
