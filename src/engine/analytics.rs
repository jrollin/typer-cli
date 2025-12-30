/// Analytics engine for tracking typing performance
/// Provides per-key and per-bigram statistics for adaptive learning
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[cfg(test)]
use super::types::CharInput;
use super::types::TypingSession;

/// Mastery level classification for keys and bigrams
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MasteryLevel {
    Beginner,   // < 70% accuracy or < 5 attempts
    Learning,   // 70-85% accuracy
    Proficient, // 85-95% accuracy
    Mastered,   // > 95% accuracy with 20+ successful attempts
}

impl MasteryLevel {
    /// Classify mastery level based on key statistics
    pub fn from_stats(stats: &KeyStats) -> Self {
        let accuracy = stats.accuracy();

        if stats.total_attempts < 5 {
            return MasteryLevel::Beginner;
        }

        if accuracy >= 95.0 && stats.correct_attempts >= 20 {
            MasteryLevel::Mastered
        } else if accuracy >= 85.0 {
            MasteryLevel::Proficient
        } else if accuracy >= 70.0 {
            MasteryLevel::Learning
        } else {
            MasteryLevel::Beginner
        }
    }

    /// Get practice weight for adaptive content generation
    /// Returns fraction of practice time this mastery level should receive
    /// Phase 3: Practice time distribution for adaptive content (60%, 30%, 10%, 5%)
    /// TODO: Refactor adaptive generator to use this method instead of hardcoded values
    #[allow(dead_code)]
    pub fn practice_weight(&self) -> f32 {
        match self {
            MasteryLevel::Beginner => 0.6,   // 60% of practice
            MasteryLevel::Learning => 0.3,   // 30% of practice
            MasteryLevel::Proficient => 0.1, // 10% of practice
            MasteryLevel::Mastered => 0.05,  // 5% for retention
        }
    }
}

/// Per-key statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStats {
    pub key: char,
    pub total_attempts: usize,
    pub correct_attempts: usize,
    pub error_count: usize,
    pub total_time_ms: u64,
    pub mistype_map: HashMap<char, usize>, // What was typed instead
    pub last_practiced: Option<SystemTime>,
    pub mastery_level: MasteryLevel,
}

impl KeyStats {
    pub fn new(key: char) -> Self {
        Self {
            key,
            total_attempts: 0,
            correct_attempts: 0,
            error_count: 0,
            total_time_ms: 0,
            mistype_map: HashMap::new(),
            last_practiced: None,
            mastery_level: MasteryLevel::Beginner,
        }
    }

    /// Calculate accuracy percentage
    pub fn accuracy(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        (self.correct_attempts as f64 / self.total_attempts as f64) * 100.0
    }

    /// Calculate average time per keystroke in milliseconds
    pub fn average_time_ms(&self) -> f64 {
        if self.correct_attempts == 0 {
            return 0.0;
        }
        self.total_time_ms as f64 / self.correct_attempts as f64
    }

    /// Calculate error rate percentage
    pub fn error_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        (self.error_count as f64 / self.total_attempts as f64) * 100.0
    }

    /// Update mastery level based on current stats
    pub fn update_mastery_level(&mut self) {
        self.mastery_level = MasteryLevel::from_stats(self);
    }
}

/// Per-bigram statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigramStats {
    pub bigram: String,
    pub total_attempts: usize,
    pub correct_attempts: usize,
    pub total_time_ms: u64,
    pub last_practiced: Option<SystemTime>,
}

impl BigramStats {
    pub fn new(bigram: String) -> Self {
        Self {
            bigram,
            total_attempts: 0,
            correct_attempts: 0,
            total_time_ms: 0,
            last_practiced: None,
        }
    }

    /// Calculate accuracy percentage
    /// Public API: Bigram performance analytics for future bigram-specific reports
    #[allow(dead_code)]
    pub fn accuracy(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        (self.correct_attempts as f64 / self.total_attempts as f64) * 100.0
    }

    /// Calculate average time per bigram in milliseconds
    /// Public API: Bigram performance analytics for future bigram-specific reports
    #[allow(dead_code)]
    pub fn average_time_ms(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        self.total_time_ms as f64 / self.total_attempts as f64
    }
}

/// Session-level analytics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAnalytics {
    pub timestamp: SystemTime,
    pub lesson_type: String,
    pub wpm: f64,
    pub accuracy: f64,
    pub duration_secs: u64,
    pub weak_keys: Vec<char>,
    pub improved_keys: Vec<char>,
}

/// Complete adaptive analytics data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdaptiveAnalytics {
    pub key_stats: HashMap<char, KeyStats>,
    pub bigram_stats: HashMap<String, BigramStats>,
    pub session_history: Vec<SessionAnalytics>,
    pub total_sessions: usize,
    pub total_keystrokes: usize,
}

/// Per-key performance analysis for a single session
#[derive(Debug, Default)]
pub struct KeyPerformance {
    pub total_attempts: usize,
    pub correct_attempts: usize,
    pub errors: Vec<char>,
    pub timings: Vec<Duration>,
}

/// Session analysis result
#[derive(Debug)]
pub struct SessionAnalysis {
    pub key_performance: HashMap<char, KeyPerformance>,
    pub bigram_performance: HashMap<String, KeyPerformance>,
}

/// Analyzer for extracting statistics from typing sessions
pub struct SessionAnalyzer;

impl SessionAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyze a completed typing session to extract per-key and per-bigram performance
    pub fn analyze_session(&self, session: &TypingSession) -> SessionAnalysis {
        let mut key_performance: HashMap<char, KeyPerformance> = HashMap::new();
        let mut bigram_performance: HashMap<String, KeyPerformance> = HashMap::new();

        // Analyze each keystroke
        for (i, input) in session.inputs.iter().enumerate() {
            let expected = input.expected;
            let typed = input.typed;

            // Update per-key statistics
            let perf = key_performance.entry(expected).or_default();

            perf.total_attempts += 1;

            if input.is_correct {
                perf.correct_attempts += 1;
                perf.timings.push(input.timestamp);
            } else {
                perf.errors.push(typed);
            }

            // Analyze bigrams (two consecutive keys)
            if i > 0 {
                let prev_input = &session.inputs[i - 1];
                if prev_input.is_correct && input.is_correct {
                    // Only count bigrams where both keys were typed correctly
                    let bigram = format!("{}{}", prev_input.expected, expected);

                    let bigram_perf = bigram_performance.entry(bigram).or_default();

                    bigram_perf.total_attempts += 1;
                    bigram_perf.correct_attempts += 1;

                    // Bigram timing is the difference between the two keystrokes
                    if let Some(prev_time) =
                        prev_input.timestamp.checked_sub(Duration::from_secs(0))
                    {
                        if let Some(time_diff) = input.timestamp.checked_sub(prev_time) {
                            bigram_perf.timings.push(time_diff);
                        }
                    }
                }
            }
        }

        SessionAnalysis {
            key_performance,
            bigram_performance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mastery_level_beginner() {
        let stats = KeyStats::new('f');
        assert_eq!(MasteryLevel::from_stats(&stats), MasteryLevel::Beginner);

        let mut stats = KeyStats::new('f');
        stats.total_attempts = 3;
        stats.correct_attempts = 3;
        assert_eq!(MasteryLevel::from_stats(&stats), MasteryLevel::Beginner);
    }

    #[test]
    fn test_mastery_level_learning() {
        let mut stats = KeyStats::new('f');
        stats.total_attempts = 50;
        stats.correct_attempts = 40; // 80% accuracy
        assert_eq!(MasteryLevel::from_stats(&stats), MasteryLevel::Learning);
    }

    #[test]
    fn test_mastery_level_proficient() {
        let mut stats = KeyStats::new('f');
        stats.total_attempts = 50;
        stats.correct_attempts = 45; // 90% accuracy
        assert_eq!(MasteryLevel::from_stats(&stats), MasteryLevel::Proficient);
    }

    #[test]
    fn test_mastery_level_mastered() {
        let mut stats = KeyStats::new('f');
        stats.total_attempts = 100;
        stats.correct_attempts = 96; // 96% accuracy with 20+ correct
        assert_eq!(MasteryLevel::from_stats(&stats), MasteryLevel::Mastered);
    }

    #[test]
    fn test_key_stats_accuracy() {
        let mut stats = KeyStats::new('f');
        stats.total_attempts = 100;
        stats.correct_attempts = 85;
        assert_eq!(stats.accuracy(), 85.0);
    }

    #[test]
    fn test_key_stats_error_rate() {
        let mut stats = KeyStats::new('f');
        stats.total_attempts = 100;
        stats.error_count = 15;
        assert_eq!(stats.error_rate(), 15.0);
    }

    #[test]
    fn test_key_stats_average_time() {
        let mut stats = KeyStats::new('f');
        stats.correct_attempts = 10;
        stats.total_time_ms = 2000; // 2 seconds total
        assert_eq!(stats.average_time_ms(), 200.0); // 200ms per key
    }

    #[test]
    fn test_bigram_stats_accuracy() {
        let mut stats = BigramStats::new("fj".to_string());
        stats.total_attempts = 50;
        stats.correct_attempts = 45;
        assert_eq!(stats.accuracy(), 90.0);
    }

    #[test]
    fn test_practice_weights() {
        assert_eq!(MasteryLevel::Beginner.practice_weight(), 0.6);
        assert_eq!(MasteryLevel::Learning.practice_weight(), 0.3);
        assert_eq!(MasteryLevel::Proficient.practice_weight(), 0.1);
        assert_eq!(MasteryLevel::Mastered.practice_weight(), 0.05);
    }

    #[test]
    fn test_session_analyzer_basic() {
        use std::time::Instant;

        let session = TypingSession {
            content: "test".to_string(),
            current_index: 4,
            duration_limit: Duration::from_secs(300),
            content_buffer_size: 4,
            inputs: vec![
                CharInput {
                    expected: 't',
                    typed: 't',
                    timestamp: Duration::from_millis(100),
                    is_correct: true,
                },
                CharInput {
                    expected: 'e',
                    typed: 'e',
                    timestamp: Duration::from_millis(250),
                    is_correct: true,
                },
                CharInput {
                    expected: 's',
                    typed: 's',
                    timestamp: Duration::from_millis(400),
                    is_correct: true,
                },
                CharInput {
                    expected: 't',
                    typed: 't',
                    timestamp: Duration::from_millis(550),
                    is_correct: true,
                },
            ],
            start_time: Some(Instant::now()),
            end_time: Some(Instant::now()),
        };

        let analyzer = SessionAnalyzer::new();
        let analysis = analyzer.analyze_session(&session);

        // Check key performance
        assert_eq!(analysis.key_performance.len(), 3); // t, e, s
        assert_eq!(analysis.key_performance[&'t'].total_attempts, 2);
        assert_eq!(analysis.key_performance[&'t'].correct_attempts, 2);
        assert_eq!(analysis.key_performance[&'e'].total_attempts, 1);
        assert_eq!(analysis.key_performance[&'s'].total_attempts, 1);
    }
}
