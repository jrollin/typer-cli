use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

use crate::engine::analytics::{AdaptiveAnalytics, KeyStats, SessionAnalysis};
use crate::engine::TypingSession;

/// Enregistrement d'une session sauvegardée
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub timestamp: String, // Format ISO 8601
    pub lesson_type: String,
    pub wpm: f64,
    pub accuracy: f64,
    #[serde(with = "duration_serde")]
    pub duration: Duration,
}

impl SessionRecord {
    pub fn new(lesson_type: String, wpm: f64, accuracy: f64, duration: Duration) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            lesson_type,
            wpm,
            accuracy,
            duration,
        }
    }
}

/// Stats globales de l'utilisateur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub sessions: Vec<SessionRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_analytics: Option<AdaptiveAnalytics>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            adaptive_analytics: None,
        }
    }

    pub fn add_session(&mut self, record: SessionRecord) {
        self.sessions.push(record);
    }

    #[allow(dead_code)]
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    #[allow(dead_code)]
    pub fn average_wpm(&self) -> f64 {
        if self.sessions.is_empty() {
            return 0.0;
        }
        let total: f64 = self.sessions.iter().map(|s| s.wpm).sum();
        total / self.sessions.len() as f64
    }

    #[allow(dead_code)]
    pub fn average_accuracy(&self) -> f64 {
        if self.sessions.is_empty() {
            return 0.0;
        }
        let total: f64 = self.sessions.iter().map(|s| s.accuracy).sum();
        total / self.sessions.len() as f64
    }

    /// Update adaptive analytics with session data
    pub fn update_analytics(&mut self, session: &TypingSession, analysis: SessionAnalysis) {
        // Get or create adaptive analytics
        let analytics = self
            .adaptive_analytics
            .get_or_insert_with(AdaptiveAnalytics::default);

        // Update per-key statistics
        for (key, perf) in analysis.key_performance {
            let key_stats = analytics
                .key_stats
                .entry(key)
                .or_insert_with(|| KeyStats::new(key));

            key_stats.total_attempts += perf.total_attempts;
            key_stats.correct_attempts += perf.correct_attempts;
            key_stats.error_count += perf.errors.len();

            // Update timing (sum all timings)
            let total_time_ms: u64 = perf.timings.iter().map(|d| d.as_millis() as u64).sum();
            key_stats.total_time_ms += total_time_ms;

            // Update mistype map
            for error_char in perf.errors {
                *key_stats.mistype_map.entry(error_char).or_insert(0) += 1;
            }

            key_stats.last_practiced = Some(SystemTime::now());
            key_stats.update_mastery_level();
        }

        // Update per-bigram statistics
        for (bigram, perf) in analysis.bigram_performance {
            let bigram_stats = analytics
                .bigram_stats
                .entry(bigram.clone())
                .or_insert_with(|| crate::engine::analytics::BigramStats::new(bigram));

            bigram_stats.total_attempts += perf.total_attempts;
            bigram_stats.correct_attempts += perf.correct_attempts;

            let total_time_ms: u64 = perf.timings.iter().map(|d| d.as_millis() as u64).sum();
            bigram_stats.total_time_ms += total_time_ms;

            bigram_stats.last_practiced = Some(SystemTime::now());
        }

        // Update global counters
        analytics.total_sessions += 1;
        analytics.total_keystrokes += session.inputs.len();
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

// Module pour sérialiser/désérialiser Duration
// Stores duration as milliseconds to preserve subsecond precision
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u128(duration.as_millis())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u128::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_new() {
        let stats = Stats::new();
        assert_eq!(stats.session_count(), 0);
        assert_eq!(stats.average_wpm(), 0.0);
    }

    #[test]
    fn test_stats_add_session() {
        let mut stats = Stats::new();
        let record =
            SessionRecord::new("HomeRow-1".to_string(), 45.0, 95.0, Duration::from_secs(60));
        stats.add_session(record);
        assert_eq!(stats.session_count(), 1);
    }

    #[test]
    fn test_stats_averages() {
        let mut stats = Stats::new();
        stats.add_session(SessionRecord::new(
            "HomeRow-1".to_string(),
            40.0,
            90.0,
            Duration::from_secs(60),
        ));
        stats.add_session(SessionRecord::new(
            "HomeRow-2".to_string(),
            60.0,
            100.0,
            Duration::from_secs(60),
        ));

        assert_eq!(stats.average_wpm(), 50.0);
        assert_eq!(stats.average_accuracy(), 95.0);
    }

    #[test]
    fn test_session_record_serialization() {
        let record = SessionRecord::new(
            "HomeRow-1".to_string(),
            45.5,
            97.3,
            Duration::from_secs(120),
        );

        let json = serde_json::to_string(&record).unwrap();
        let deserialized: SessionRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.lesson_type, "HomeRow-1");
        assert!((deserialized.wpm - 45.5).abs() < 0.01);
        assert_eq!(deserialized.duration, Duration::from_secs(120));
    }
}
