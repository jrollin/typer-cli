# Adaptive Mode - Design Document

> **Purpose**: Technical design for personalized adaptive training
> **Module**: `src/engine/analytics.rs` and `src/content/adaptive.rs`
> **Previous Step**: See `requirements.md` for adaptive mode requirements
> **Related**: See `../session-storage/design.md` for stats persistence

## Overview

The adaptive mode analyzes user typing patterns to identify weaknesses and generates personalized practice content. It uses statistical analysis and spaced repetition principles to optimize learning.

## Architecture

### Data Structures

**Per-Key Statistics:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStats {
    pub key: char,
    pub total_attempts: usize,
    pub correct_attempts: usize,
    pub error_count: usize,
    pub total_time_ms: u64,
    pub mistype_map: HashMap<char, usize>,  // What was typed instead
    pub last_practiced: Option<SystemTime>,
    pub mastery_level: MasteryLevel,
}

impl KeyStats {
    pub fn accuracy(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        (self.correct_attempts as f64 / self.total_attempts as f64) * 100.0
    }

    pub fn average_time_ms(&self) -> f64 {
        if self.correct_attempts == 0 {
            return 0.0;
        }
        self.total_time_ms as f64 / self.correct_attempts as f64
    }

    pub fn error_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        (self.error_count as f64 / self.total_attempts as f64) * 100.0
    }
}
```

**Mastery Levels:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MasteryLevel {
    Beginner,    // < 70% accuracy or < 5 attempts
    Learning,    // 70-85% accuracy, improving trend
    Proficient,  // 85-95% accuracy, stable
    Mastered,    // > 95% accuracy, 20+ attempts
}

impl MasteryLevel {
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

    pub fn practice_weight(&self) -> f32 {
        match self {
            MasteryLevel::Beginner => 0.6,    // 60% of practice
            MasteryLevel::Learning => 0.3,    // 30% of practice
            MasteryLevel::Proficient => 0.1,  // 10% of practice
            MasteryLevel::Mastered => 0.05,   // 5% for retention
        }
    }
}
```

**Per-Bigram Statistics:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigramStats {
    pub bigram: String,
    pub total_attempts: usize,
    pub correct_attempts: usize,
    pub total_time_ms: u64,
    pub last_practiced: Option<SystemTime>,
}

impl BigramStats {
    pub fn accuracy(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        (self.correct_attempts as f64 / self.total_attempts as f64) * 100.0
    }

    pub fn average_time_ms(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        self.total_time_ms as f64 / self.total_attempts as f64
    }
}
```

**Adaptive Analytics:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct AdaptiveAnalytics {
    pub key_stats: HashMap<char, KeyStats>,
    pub bigram_stats: HashMap<String, BigramStats>,
    pub session_history: Vec<SessionAnalytics>,
    pub total_sessions: usize,
    pub total_keystrokes: usize,
}

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
```

## Analytics Engine

### Data Collection During Session

```rust
pub struct SessionAnalyzer {
    start_time: Instant,
    key_timings: HashMap<char, Vec<Duration>>,
}

impl SessionAnalyzer {
    pub fn record_keystroke(&mut self, expected: char, typed: char, time: Instant) {
        let duration = time.duration_since(self.start_time);

        if expected == typed {
            // Record successful keystroke timing
            self.key_timings.entry(expected)
                .or_insert_with(Vec::new)
                .push(duration);
        } else {
            // Record error
        }
    }

    pub fn analyze_session(&self, session: &TypingSession) -> SessionAnalysis {
        let mut key_performance: HashMap<char, KeyPerformance> = HashMap::new();

        for (i, input) in session.inputs.iter().enumerate() {
            let expected = session.content.chars().nth(i).unwrap();

            let perf = key_performance.entry(expected)
                .or_insert_with(KeyPerformance::default);

            perf.total_attempts += 1;

            if input.is_correct {
                perf.correct_attempts += 1;
            } else {
                perf.errors.push(input.typed);
            }
        }

        SessionAnalysis { key_performance }
    }
}
```

### Weak Key Identification

```rust
pub struct WeaknessDetector;

impl WeaknessDetector {
    pub fn identify_weak_keys(analytics: &AdaptiveAnalytics, threshold: f64) -> Vec<char> {
        let mut weak_keys: Vec<_> = analytics.key_stats
            .iter()
            .filter(|(_, stats)| {
                stats.total_attempts >= 10 &&  // Minimum data threshold
                stats.accuracy() < threshold    // Below threshold
            })
            .map(|(key, stats)| (*key, stats.error_rate()))
            .collect();

        // Sort by error rate (worst first)
        weak_keys.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        weak_keys.into_iter()
            .take(5)  // Top 5 weakest keys
            .map(|(key, _)| key)
            .collect()
    }

    pub fn identify_slow_keys(analytics: &AdaptiveAnalytics, percentile: f64) -> Vec<char> {
        let timings: Vec<_> = analytics.key_stats
            .iter()
            .filter(|(_, stats)| stats.correct_attempts >= 5)
            .map(|(key, stats)| (*key, stats.average_time_ms()))
            .collect();

        if timings.is_empty() {
            return vec![];
        }

        // Calculate percentile threshold
        let mut times: Vec<_> = timings.iter().map(|(_, t)| *t).collect();
        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let threshold_idx = ((times.len() as f64 * percentile) as usize).min(times.len() - 1);
        let threshold = times[threshold_idx];

        // Return keys slower than threshold
        timings.into_iter()
            .filter(|(_, time)| *time > threshold)
            .map(|(key, _)| key)
            .collect()
    }

    pub fn identify_weak_bigrams(analytics: &AdaptiveAnalytics) -> Vec<String> {
        let mut weak_bigrams: Vec<_> = analytics.bigram_stats
            .iter()
            .filter(|(_, stats)| {
                stats.total_attempts >= 5 &&
                stats.accuracy() < 85.0
            })
            .map(|(bigram, stats)| (bigram.clone(), stats.accuracy()))
            .collect();

        weak_bigrams.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        weak_bigrams.into_iter()
            .take(5)
            .map(|(bigram, _)| bigram)
            .collect()
    }
}
```

## Adaptive Content Generation

### Spaced Repetition Algorithm

```rust
pub struct SpacedRepetition;

impl SpacedRepetition {
    // Calculate next practice interval based on performance
    pub fn next_interval(current_mastery: MasteryLevel, accuracy: f64) -> Duration {
        match current_mastery {
            MasteryLevel::Beginner => Duration::from_secs(0),      // Practice now
            MasteryLevel::Learning if accuracy < 80.0 => {
                Duration::from_secs(60 * 30)                       // 30 minutes
            }
            MasteryLevel::Learning => Duration::from_secs(60 * 60 * 2), // 2 hours
            MasteryLevel::Proficient if accuracy < 90.0 => {
                Duration::from_secs(60 * 60 * 24)                  // 1 day
            }
            MasteryLevel::Proficient => Duration::from_secs(60 * 60 * 24 * 3), // 3 days
            MasteryLevel::Mastered => Duration::from_secs(60 * 60 * 24 * 7),   // 1 week
        }
    }

    // Check if key needs practice based on last practiced time
    pub fn needs_practice(stats: &KeyStats) -> bool {
        let Some(last_practiced) = stats.last_practiced else {
            return true;  // Never practiced
        };

        let elapsed = SystemTime::now()
            .duration_since(last_practiced)
            .unwrap_or(Duration::from_secs(0));

        let interval = Self::next_interval(
            stats.mastery_level,
            stats.accuracy()
        );

        elapsed >= interval
    }
}
```

### Adaptive Lesson Generator

```rust
pub struct AdaptiveLessonGenerator {
    analytics: AdaptiveAnalytics,
}

impl AdaptiveLessonGenerator {
    pub fn generate(&self, length: usize) -> String {
        // Identify focus areas
        let weak_keys = WeaknessDetector::identify_weak_keys(&self.analytics, 80.0);
        let slow_keys = WeaknessDetector::identify_slow_keys(&self.analytics, 0.75);
        let weak_bigrams = WeaknessDetector::identify_weak_bigrams(&self.analytics);

        // Combine and prioritize
        let mut focus_keys = weak_keys;
        focus_keys.extend(slow_keys.iter().filter(|k| !focus_keys.contains(k)));

        if focus_keys.is_empty() {
            return self.generate_balanced_practice(length);
        }

        // Generate content with weighted distribution
        self.generate_weighted_content(&focus_keys, &weak_bigrams, length)
    }

    fn generate_weighted_content(
        &self,
        focus_keys: &[char],
        weak_bigrams: &[String],
        length: usize,
    ) -> String {
        let mut result = String::new();
        let mut rng = rand::thread_rng();

        // 60% weak keys, 30% moderate, 10% strong
        while result.len() < length {
            let r: f32 = rng.gen();

            let keys = if r < 0.6 {
                // Weak keys
                focus_keys
            } else if r < 0.9 {
                // Moderate keys
                &self.get_moderate_keys()
            } else {
                // Strong keys (for retention)
                &self.get_strong_keys()
            };

            if keys.is_empty() {
                continue;
            }

            // Generate pattern with selected keys
            let pattern = self.generate_pattern(keys);
            result.push_str(&pattern);
            result.push(' ');
        }

        result.trim().chars().take(length).collect()
    }

    fn generate_pattern(&self, keys: &[char]) -> String {
        // Create varied patterns: repetitions, alternations, triplets
        let mut rng = rand::thread_rng();
        let pattern_type: u8 = rng.gen_range(0..3);

        match pattern_type {
            0 => {
                // Repetition: "ff ff"
                let key = keys[rng.gen_range(0..keys.len())];
                format!("{}{}", key, key)
            }
            1 => {
                // Alternation: "fj fj"
                if keys.len() >= 2 {
                    let k1 = keys[rng.gen_range(0..keys.len())];
                    let k2 = keys[rng.gen_range(0..keys.len())];
                    format!("{}{}", k1, k2)
                } else {
                    let key = keys[0];
                    format!("{}{}", key, key)
                }
            }
            _ => {
                // Triplet: "fjd"
                if keys.len() >= 3 {
                    let k1 = keys[rng.gen_range(0..keys.len())];
                    let k2 = keys[rng.gen_range(0..keys.len())];
                    let k3 = keys[rng.gen_range(0..keys.len())];
                    format!("{}{}{}", k1, k2, k3)
                } else {
                    let key = keys[0];
                    format!("{}{}", key, key)
                }
            }
        }
    }

    fn get_moderate_keys(&self) -> Vec<char> {
        self.analytics.key_stats
            .iter()
            .filter(|(_, stats)| {
                let acc = stats.accuracy();
                acc >= 80.0 && acc < 90.0
            })
            .map(|(key, _)| *key)
            .collect()
    }

    fn get_strong_keys(&self) -> Vec<char> {
        self.analytics.key_stats
            .iter()
            .filter(|(_, stats)| stats.accuracy() >= 95.0)
            .map(|(key, _)| *key)
            .collect()
    }

    fn generate_balanced_practice(&self, length: usize) -> String {
        // Fallback when no weak areas identified
        "The quick brown fox jumps over the lazy dog"
            .chars()
            .cycle()
            .take(length)
            .collect()
    }
}
```

## Recommendation Engine

```rust
pub struct RecommendationEngine;

impl RecommendationEngine {
    pub fn recommend_next_lesson(analytics: &AdaptiveAnalytics) -> Recommendation {
        if analytics.total_sessions < 10 {
            return Recommendation {
                lesson_type: "Home Row - Level 1".to_string(),
                reason: "Build foundation with basic home row practice".to_string(),
                confidence: 0.9,
            };
        }

        let weak_keys = WeaknessDetector::identify_weak_keys(analytics, 80.0);
        let weak_bigrams = WeaknessDetector::identify_weak_bigrams(analytics);

        if !weak_keys.is_empty() {
            Recommendation {
                lesson_type: "Adaptive - Weak Keys".to_string(),
                reason: format!("Focus on weak keys: {}",
                    weak_keys.iter().collect::<String>()),
                confidence: 0.85,
            }
        } else if !weak_bigrams.is_empty() {
            Recommendation {
                lesson_type: "Adaptive - Weak Bigrams".to_string(),
                reason: format!("Focus on weak bigrams: {}",
                    weak_bigrams.join(", ")),
                confidence: 0.80,
            }
        } else {
            Recommendation {
                lesson_type: "Code Symbols - Level 1".to_string(),
                reason: "Expand skills with code symbol practice".to_string(),
                confidence: 0.75,
            }
        }
    }
}

pub struct Recommendation {
    pub lesson_type: String,
    pub reason: String,
    pub confidence: f64,  // 0.0 to 1.0
}
```

## UI Integration

### Adaptive Mode Display

```
┌──────────────────────────────────────────────┐
│         Adaptive Mode - Recommended          │
├──────────────────────────────────────────────┤
│                                              │
│  Current Focus:                              │
│    Weak Keys: d, k, s (accuracy < 80%)       │
│    Slow Keys: q, m (> 200ms average)         │
│                                              │
│  Practice Content:                           │
│    60% Weak key drills                       │
│    30% Moderate difficulty                   │
│    10% Mastered (retention)                  │
│                                              │
│  Session Goal:                               │
│    Improve 'd' and 'k' accuracy to 85%       │
│                                              │
│  Press Enter to start                        │
│                                              │
└──────────────────────────────────────────────┘
```

### Post-Session Feedback

```
┌──────────────────────────────────────────────┐
│            Adaptive Session Complete          │
├──────────────────────────────────────────────┤
│                                              │
│  Improvement:                                │
│    ✓ Key 'd': 75% → 82% (+7%)                │
│    ✓ Key 'k': 78% → 84% (+6%)                │
│    ⚠ Key 's': 70% → 72% (+2%)                │
│                                              │
│  Next Recommendation:                        │
│    Continue adaptive practice focusing on    │
│    's' and 'l' keys (2-3 more sessions)      │
│                                              │
│  Overall Progress:                           │
│    14/26 keys at Proficient level            │
│    Home row mastery: 85%                     │
│                                              │
└──────────────────────────────────────────────┘
```

## Data Persistence

### Storage Extension

```rust
// Extend existing Stats struct
#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub sessions: Vec<SessionRecord>,
    pub adaptive_analytics: Option<AdaptiveAnalytics>,  // New field
}

impl Stats {
    pub fn update_analytics(&mut self, session: &TypingSession, analysis: SessionAnalysis) {
        let analytics = self.adaptive_analytics
            .get_or_insert_with(AdaptiveAnalytics::default);

        // Update key stats
        for (key, perf) in analysis.key_performance {
            let key_stats = analytics.key_stats
                .entry(key)
                .or_insert_with(|| KeyStats::new(key));

            key_stats.total_attempts += perf.total_attempts;
            key_stats.correct_attempts += perf.correct_attempts;
            key_stats.error_count += perf.errors.len();
            key_stats.last_practiced = Some(SystemTime::now());

            // Update mastery level
            key_stats.mastery_level = MasteryLevel::from_stats(key_stats);
        }

        analytics.total_sessions += 1;
        analytics.total_keystrokes += session.inputs.len();
    }
}
```

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_mastery_level_calculation() {
        let mut stats = KeyStats::new('f');
        stats.total_attempts = 100;
        stats.correct_attempts = 96;

        assert_eq!(MasteryLevel::from_stats(&stats), MasteryLevel::Mastered);
    }

    #[test]
    fn test_weak_key_identification() {
        let mut analytics = AdaptiveAnalytics::default();

        // Add some stats
        let mut f_stats = KeyStats::new('f');
        f_stats.total_attempts = 50;
        f_stats.correct_attempts = 48;  // 96% accuracy

        let mut d_stats = KeyStats::new('d');
        d_stats.total_attempts = 50;
        d_stats.correct_attempts = 35;  // 70% accuracy (weak!)

        analytics.key_stats.insert('f', f_stats);
        analytics.key_stats.insert('d', d_stats);

        let weak = WeaknessDetector::identify_weak_keys(&analytics, 80.0);
        assert!(weak.contains(&'d'));
        assert!(!weak.contains(&'f'));
    }

    #[test]
    fn test_adaptive_content_generation() {
        let gen = AdaptiveLessonGenerator::new(/* ... */);
        let content = gen.generate(100);

        assert!(!content.is_empty());
        assert!(content.len() <= 100);
    }
}
```

## File Locations

- `src/engine/analytics.rs` - Analytics engine and stats tracking
- `src/engine/adaptive.rs` - Adaptive algorithms and spaced repetition
- `src/content/adaptive_generator.rs` - Adaptive content generation
- `src/data/stats.rs` - Extended with adaptive analytics

## Performance Considerations

- **Incremental updates**: Update stats after each session, not bulk recomputation
- **Efficient storage**: JSON serialization for human-readable format
- **Memory usage**: Keep only last 100 sessions in memory, archive older data
- **Fast lookups**: HashMap for O(1) key stat access
- **Lazy computation**: Calculate recommendations only when needed
