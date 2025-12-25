/// Adaptive lesson content generator
/// Generates personalized practice content based on user weaknesses
use rand::{thread_rng, Rng};

use crate::engine::adaptive::WeaknessDetector;
use crate::engine::analytics::AdaptiveAnalytics;

/// Adaptive lesson generator that creates personalized content
pub struct AdaptiveLessonGenerator<'a> {
    analytics: &'a AdaptiveAnalytics,
}

impl<'a> AdaptiveLessonGenerator<'a> {
    pub fn new(analytics: &'a AdaptiveAnalytics) -> Self {
        Self { analytics }
    }

    /// Generate adaptive practice content of specified length
    pub fn generate(&self, length: usize) -> String {
        // Identify focus areas
        let weak_keys = WeaknessDetector::identify_weak_keys(self.analytics, 80.0);
        let slow_keys = WeaknessDetector::identify_slow_keys(self.analytics, 0.75);

        // Combine weak and slow keys (deduplicate)
        let mut focus_keys = weak_keys;
        for key in slow_keys {
            if !focus_keys.contains(&key) {
                focus_keys.push(key);
            }
        }

        if focus_keys.is_empty() {
            return self.generate_balanced_practice(length);
        }

        // Generate content with weighted distribution
        self.generate_weighted_content(&focus_keys, length)
    }

    /// Generate content with 60% weak, 30% moderate, 10% strong distribution
    fn generate_weighted_content(&self, focus_keys: &[char], length: usize) -> String {
        let mut result = String::new();
        let mut rng = thread_rng();

        let moderate_keys = self.get_moderate_keys();
        let strong_keys = self.get_strong_keys();

        while result.len() < length {
            if !result.is_empty() {
                result.push(' ');
            }

            // Weighted random selection: 60% weak, 30% moderate, 10% strong
            let r: f32 = rng.gen();

            let keys = if r < 0.6 && !focus_keys.is_empty() {
                // 60%: Weak keys
                focus_keys
            } else if r < 0.9 && !moderate_keys.is_empty() {
                // 30%: Moderate keys
                &moderate_keys
            } else if !strong_keys.is_empty() {
                // 10%: Strong keys (retention)
                &strong_keys
            } else if !focus_keys.is_empty() {
                // Fallback to weak keys if others not available
                focus_keys
            } else {
                // Should not happen, but handle gracefully
                &moderate_keys
            };

            if keys.is_empty() {
                continue;
            }

            // Generate pattern with selected keys
            let pattern = self.generate_pattern(keys);
            result.push_str(&pattern);
        }

        result.chars().take(length).collect()
    }

    /// Generate varied patterns: repetitions, alternations, triplets
    fn generate_pattern(&self, keys: &[char]) -> String {
        let mut rng = thread_rng();
        let pattern_type: u8 = rng.gen_range(0..3);

        match pattern_type {
            0 => {
                // Repetition: "ff" or "ff ff"
                let key = keys[rng.gen_range(0..keys.len())];
                if rng.gen_bool(0.5) {
                    format!("{}{}", key, key)
                } else {
                    format!("{}{} {}{}", key, key, key, key)
                }
            }
            1 => {
                // Alternation: "fj fj"
                if keys.len() >= 2 {
                    let k1 = keys[rng.gen_range(0..keys.len())];
                    let k2 = keys[rng.gen_range(0..keys.len())];
                    format!("{}{} {}{}", k1, k2, k1, k2)
                } else {
                    let key = keys[0];
                    format!("{}{}", key, key)
                }
            }
            _ => {
                // Triplet or sequence: "fjd"
                if keys.len() >= 3 {
                    let k1 = keys[rng.gen_range(0..keys.len())];
                    let k2 = keys[rng.gen_range(0..keys.len())];
                    let k3 = keys[rng.gen_range(0..keys.len())];
                    format!("{}{}{}", k1, k2, k3)
                } else if keys.len() >= 2 {
                    let k1 = keys[rng.gen_range(0..keys.len())];
                    let k2 = keys[rng.gen_range(0..keys.len())];
                    format!("{}{}", k1, k2)
                } else {
                    let key = keys[0];
                    format!("{}{}", key, key)
                }
            }
        }
    }

    /// Get keys with moderate difficulty (80-90% accuracy)
    fn get_moderate_keys(&self) -> Vec<char> {
        self.analytics
            .key_stats
            .iter()
            .filter(|(_, stats)| {
                let acc = stats.accuracy();
                stats.total_attempts >= 10 && acc >= 80.0 && acc < 90.0
            })
            .map(|(key, _)| *key)
            .collect()
    }

    /// Get strong keys (>= 95% accuracy)
    fn get_strong_keys(&self) -> Vec<char> {
        self.analytics
            .key_stats
            .iter()
            .filter(|(_, stats)| stats.total_attempts >= 10 && stats.accuracy() >= 95.0)
            .map(|(key, _)| *key)
            .collect()
    }

    /// Fallback balanced practice when no weak areas identified
    fn generate_balanced_practice(&self, length: usize) -> String {
        "The quick brown fox jumps over the lazy dog"
            .chars()
            .cycle()
            .take(length)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::analytics::KeyStats;

    fn create_test_analytics() -> AdaptiveAnalytics {
        let mut analytics = AdaptiveAnalytics::default();

        // Add weak key
        let mut d_stats = KeyStats::new('d');
        d_stats.total_attempts = 50;
        d_stats.correct_attempts = 35; // 70% accuracy (weak)
        analytics.key_stats.insert('d', d_stats);

        // Add moderate key
        let mut s_stats = KeyStats::new('s');
        s_stats.total_attempts = 50;
        s_stats.correct_attempts = 42; // 84% accuracy (moderate)
        analytics.key_stats.insert('s', s_stats);

        // Add strong key
        let mut f_stats = KeyStats::new('f');
        f_stats.total_attempts = 50;
        f_stats.correct_attempts = 48; // 96% accuracy (strong)
        analytics.key_stats.insert('f', f_stats);

        analytics.total_sessions = 15;
        analytics.total_keystrokes = 500;

        analytics
    }

    #[test]
    fn test_generate_content() {
        let analytics = create_test_analytics();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let content = generator.generate(100);

        assert!(!content.is_empty());
        assert!(content.len() <= 100);
    }

    #[test]
    fn test_generate_contains_weak_keys() {
        let analytics = create_test_analytics();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let content = generator.generate(200);

        // Should contain the weak key 'd' multiple times (60% of content)
        assert!(content.contains('d'));
    }

    #[test]
    fn test_fallback_balanced_practice() {
        let analytics = AdaptiveAnalytics::default();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let content = generator.generate(50);

        assert!(!content.is_empty());
        assert!(content.len() <= 50);
    }

    #[test]
    fn test_get_moderate_keys() {
        let analytics = create_test_analytics();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let moderate = generator.get_moderate_keys();

        // 's' is moderate (84% accuracy)
        assert!(moderate.contains(&'s'));
        // 'd' is weak (70% accuracy), not moderate
        assert!(!moderate.contains(&'d'));
        // 'f' is strong (96% accuracy), not moderate
        assert!(!moderate.contains(&'f'));
    }

    #[test]
    fn test_get_strong_keys() {
        let analytics = create_test_analytics();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let strong = generator.get_strong_keys();

        // 'f' is strong (96% accuracy)
        assert!(strong.contains(&'f'));
        // 'd' and 's' are not strong
        assert!(!strong.contains(&'d'));
        assert!(!strong.contains(&'s'));
    }

    #[test]
    fn test_multiple_generations_vary() {
        let analytics = create_test_analytics();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let content1 = generator.generate(80);
        let content2 = generator.generate(80);

        // Due to randomness, content should likely differ
        // (Not guaranteed due to random nature, but statistically very likely)
        // Just verify both are valid
        assert!(!content1.is_empty());
        assert!(!content2.is_empty());
    }
}
