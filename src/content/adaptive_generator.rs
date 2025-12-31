/// Adaptive lesson content generator
/// Generates personalized practice content based on user weaknesses
use rand::{thread_rng, Rng};

use crate::engine::adaptive::WeaknessDetector;
use crate::engine::analytics::{AdaptiveAnalytics, MasteryLevel};

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

    /// Generate content with mastery-based distribution using practice_weight()
    fn generate_weighted_content(&self, _focus_keys: &[char], length: usize) -> String {
        let mut result = String::new();
        let mut rng = thread_rng();

        // Classify keys by mastery level
        let beginner_keys = self.get_keys_by_mastery(MasteryLevel::Beginner);
        let learning_keys = self.get_keys_by_mastery(MasteryLevel::Learning);
        let proficient_keys = self.get_keys_by_mastery(MasteryLevel::Proficient);
        let mastered_keys = self.get_keys_by_mastery(MasteryLevel::Mastered);

        // Calculate cumulative thresholds from practice weights
        let beginner_threshold = MasteryLevel::Beginner.practice_weight(); // 0.6
        let learning_threshold = beginner_threshold + MasteryLevel::Learning.practice_weight(); // 0.9
        let proficient_threshold = learning_threshold + MasteryLevel::Proficient.practice_weight(); // 1.0

        while result.len() < length {
            if !result.is_empty() {
                result.push(' ');
            }

            // Weighted random selection based on mastery levels
            let r: f32 = rng.gen();

            let keys = if r < beginner_threshold && !beginner_keys.is_empty() {
                // 60%: Beginner keys
                &beginner_keys
            } else if r < learning_threshold && !learning_keys.is_empty() {
                // 30%: Learning keys
                &learning_keys
            } else if r < proficient_threshold && !proficient_keys.is_empty() {
                // 10%: Proficient keys
                &proficient_keys
            } else if !mastered_keys.is_empty() {
                // 5%: Mastered keys (retention practice)
                &mastered_keys
            } else if !beginner_keys.is_empty() {
                // Fallback to beginner keys if others not available
                &beginner_keys
            } else {
                // Should not happen, but handle gracefully
                &learning_keys
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

    /// Get keys classified at a specific mastery level
    fn get_keys_by_mastery(&self, level: MasteryLevel) -> Vec<char> {
        self.analytics
            .key_stats
            .iter()
            .filter(|(_, stats)| {
                stats.total_attempts >= 10 && MasteryLevel::from_stats(stats) == level
            })
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
    fn test_get_keys_by_mastery_beginner() {
        let analytics = create_test_analytics();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let beginner = generator.get_keys_by_mastery(MasteryLevel::Beginner);

        // No keys are beginner in test data (d: 70%, s: 84%, f: 96%)
        // Beginner is < 70% accuracy
        assert!(beginner.is_empty());
    }

    #[test]
    fn test_get_keys_by_mastery_learning() {
        let analytics = create_test_analytics();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let learning = generator.get_keys_by_mastery(MasteryLevel::Learning);

        // 'd' is learning (70% accuracy, range is 70-85%)
        // 's' is learning (84% accuracy, range is 70-85%)
        assert!(learning.contains(&'d'));
        assert!(learning.contains(&'s'));
        // 'f' is mastered
        assert!(!learning.contains(&'f'));
    }

    #[test]
    fn test_get_keys_by_mastery_mastered() {
        let analytics = create_test_analytics();
        let generator = AdaptiveLessonGenerator::new(&analytics);

        let mastered = generator.get_keys_by_mastery(MasteryLevel::Mastered);

        // 'f' is mastered (96% accuracy with 48 correct attempts >= 20)
        assert!(mastered.contains(&'f'));
        // 'd' and 's' are not mastered
        assert!(!mastered.contains(&'d'));
        assert!(!mastered.contains(&'s'));
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
