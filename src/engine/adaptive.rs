/// Adaptive algorithms for personalized training
/// Includes weakness detection, spaced repetition, and recommendations
use std::time::{Duration, SystemTime};

use super::analytics::{AdaptiveAnalytics, KeyStats, MasteryLevel};

/// Weakness detector for identifying problem areas
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

/// Spaced repetition algorithm for optimal practice scheduling
#[allow(dead_code)]
pub struct SpacedRepetition;

impl SpacedRepetition {
    /// Calculate next practice interval based on mastery level and performance
    #[allow(dead_code)]
    pub fn next_interval(mastery_level: MasteryLevel, accuracy: f64) -> Duration {
        match mastery_level {
            MasteryLevel::Beginner => Duration::from_secs(0), // Practice immediately
            MasteryLevel::Learning if accuracy < 80.0 => {
                Duration::from_secs(60 * 30) // 30 minutes
            }
            MasteryLevel::Learning => Duration::from_secs(60 * 60 * 2), // 2 hours
            MasteryLevel::Proficient if accuracy < 90.0 => {
                Duration::from_secs(60 * 60 * 24) // 1 day
            }
            MasteryLevel::Proficient => Duration::from_secs(60 * 60 * 24 * 3), // 3 days
            MasteryLevel::Mastered => Duration::from_secs(60 * 60 * 24 * 7),   // 1 week
        }
    }

    /// Check if a key needs practice based on last practiced time
    #[allow(dead_code)]
    pub fn needs_practice(stats: &KeyStats) -> bool {
        let Some(last_practiced) = stats.last_practiced else {
            return true; // Never practiced
        };

        let elapsed = SystemTime::now()
            .duration_since(last_practiced)
            .unwrap_or(Duration::from_secs(0));

        let interval = Self::next_interval(stats.mastery_level, stats.accuracy());

        elapsed >= interval
    }
}

/// Recommendation for next practice session
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub lesson_type: String,
    pub reason: String,
    pub confidence: f64, // 0.0 to 1.0
}

/// Recommendation engine for suggesting next lesson
#[allow(dead_code)]
pub struct RecommendationEngine;

impl RecommendationEngine {
    /// Recommend next lesson based on user analytics
    #[allow(dead_code)]
    pub fn recommend_next_lesson(analytics: &AdaptiveAnalytics) -> Recommendation {
        // Insufficient data - recommend foundation building
        if analytics.total_sessions < 10 {
            return Recommendation {
                lesson_type: "Home Row - Level 1".to_string(),
                reason: "Build foundation with basic home row practice".to_string(),
                confidence: 0.9,
            };
        }

        let weak_keys = WeaknessDetector::identify_weak_keys(analytics, 80.0);
        let weak_bigrams = WeaknessDetector::identify_weak_bigrams(analytics);

        // Recommend adaptive mode if weak areas exist
        if !weak_keys.is_empty() {
            return Recommendation {
                lesson_type: "Adaptive - Weak Keys".to_string(),
                reason: format!(
                    "Focus on weak keys: {}",
                    weak_keys.iter().collect::<String>()
                ),
                confidence: 0.85,
            };
        }

        // Recommend bigram practice if weak bigrams exist
        if !weak_bigrams.is_empty() {
            return Recommendation {
                lesson_type: "Adaptive - Weak Bigrams".to_string(),
                reason: format!("Focus on weak bigrams: {}", weak_bigrams.join(", ")),
                confidence: 0.80,
            };
        }

        // No weak areas - recommend skill expansion
        Recommendation {
            lesson_type: "Code Symbols - Level 1".to_string(),
            reason: "Expand skills with code symbol practice".to_string(),
            confidence: 0.75,
        }
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
        assert!(slow_keys.len() > 0);
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

    #[test]
    fn test_spaced_repetition_intervals() {
        // Beginner should practice immediately
        assert_eq!(
            SpacedRepetition::next_interval(MasteryLevel::Beginner, 50.0),
            Duration::from_secs(0)
        );

        // Learning with low accuracy: 30 minutes
        assert_eq!(
            SpacedRepetition::next_interval(MasteryLevel::Learning, 75.0),
            Duration::from_secs(60 * 30)
        );

        // Learning with good accuracy: 2 hours
        assert_eq!(
            SpacedRepetition::next_interval(MasteryLevel::Learning, 82.0),
            Duration::from_secs(60 * 60 * 2)
        );

        // Proficient with lower accuracy: 1 day
        assert_eq!(
            SpacedRepetition::next_interval(MasteryLevel::Proficient, 88.0),
            Duration::from_secs(60 * 60 * 24)
        );

        // Proficient with good accuracy: 3 days
        assert_eq!(
            SpacedRepetition::next_interval(MasteryLevel::Proficient, 92.0),
            Duration::from_secs(60 * 60 * 24 * 3)
        );

        // Mastered: 1 week
        assert_eq!(
            SpacedRepetition::next_interval(MasteryLevel::Mastered, 97.0),
            Duration::from_secs(60 * 60 * 24 * 7)
        );
    }

    #[test]
    fn test_needs_practice_never_practiced() {
        let stats = KeyStats::new('f');
        assert!(SpacedRepetition::needs_practice(&stats));
    }

    #[test]
    fn test_recommendation_insufficient_data() {
        let mut analytics = AdaptiveAnalytics::default();
        analytics.total_sessions = 5; // Below threshold of 10

        let rec = RecommendationEngine::recommend_next_lesson(&analytics);

        assert_eq!(rec.lesson_type, "Home Row - Level 1");
        assert!(rec.confidence > 0.8);
    }

    #[test]
    fn test_recommendation_weak_keys() {
        let analytics = create_test_analytics();
        let rec = RecommendationEngine::recommend_next_lesson(&analytics);

        assert_eq!(rec.lesson_type, "Adaptive - Weak Keys");
        assert!(rec.reason.contains("weak keys"));
        assert!(rec.confidence > 0.8);
    }

    #[test]
    fn test_recommendation_no_weak_areas() {
        let mut analytics = AdaptiveAnalytics::default();

        // Add only strong keys
        let mut f_stats = KeyStats::new('f');
        f_stats.total_attempts = 50;
        f_stats.correct_attempts = 48; // 96% accuracy
        analytics.key_stats.insert('f', f_stats);

        analytics.total_sessions = 15;

        let rec = RecommendationEngine::recommend_next_lesson(&analytics);

        assert_eq!(rec.lesson_type, "Code Symbols - Level 1");
        assert!(rec.reason.contains("Expand skills"));
    }
}
