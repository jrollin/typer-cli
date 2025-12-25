# Adaptive Mode - Task Tracking

> **Purpose**: Implementation progress for adaptive personalized training
> **Modules**: `src/engine/analytics.rs`, `src/content/adaptive.rs`
> **Status**: ⏳ PLANNED (Phase 2+)

## Phase 2+: Core Implementation

### Data Structures (src/engine/analytics.rs)
- [ ] Define `KeyStats` struct (attempts, accuracy, timing, mistypes)
- [ ] Define `BigramStats` struct (attempts, accuracy, timing)
- [ ] Define `MasteryLevel` enum (Beginner, Learning, Proficient, Mastered)
- [ ] Define `AdaptiveAnalytics` struct (key stats, bigram stats, history)
- [ ] Define `SessionAnalytics` struct (session-level metrics)
- [ ] Implement accuracy/speed calculation methods
- [ ] Implement mastery level classification logic

### Analytics Engine (src/engine/analytics.rs)
- [ ] Implement `SessionAnalyzer` for real-time keystroke tracking
- [ ] Record keystroke timing per character
- [ ] Track errors and mistype patterns
- [ ] Calculate per-key performance metrics
- [ ] Calculate per-bigram performance metrics
- [ ] Build session summary statistics
- [ ] Integrate with existing `TypingSession`

### Weakness Detection (src/engine/adaptive.rs)
- [ ] Implement `WeaknessDetector` struct
- [ ] `identify_weak_keys()` based on accuracy threshold
- [ ] `identify_slow_keys()` based on timing percentile
- [ ] `identify_weak_bigrams()` for two-letter combinations
- [ ] Implement minimum data threshold (10 attempts)
- [ ] Sort by severity (worst first)
- [ ] Return top 5 weak areas

### Spaced Repetition (src/engine/adaptive.rs)
- [ ] Implement `SpacedRepetition` algorithm
- [ ] Calculate next practice interval by mastery level
- [ ] Check if key needs practice based on last practice time
- [ ] Implement interval adjustment based on performance
- [ ] Support for beginner → learning → proficient → mastered progression

### Adaptive Content Generation (src/content/adaptive_generator.rs)
- [ ] Implement `AdaptiveLessonGenerator` struct
- [ ] Generate content with 60% weak keys
- [ ] Generate content with 30% moderate keys
- [ ] Generate content with 10% mastered keys (retention)
- [ ] Implement pattern variety (repetitions, alternations, triplets)
- [ ] Use weak bigrams in generated content
- [ ] Fallback to balanced practice when no weak areas
- [ ] Ensure deterministic generation for same weak keys

### Recommendation Engine (src/engine/adaptive.rs)
- [ ] Implement `RecommendationEngine` struct
- [ ] Analyze user progress and identify next best lesson
- [ ] Generate recommendation with reason and confidence
- [ ] Handle insufficient data case (< 10 sessions)
- [ ] Recommend adaptive mode when weak areas exist
- [ ] Recommend standard lessons when proficient
- [ ] Recommend new content (bigrams, code) for variety

### Data Persistence (src/data/stats.rs extension)
- [ ] Extend `Stats` struct with `adaptive_analytics` field
- [ ] Implement incremental analytics update after each session
- [ ] Save to `~/.config/typer-cli/stats.json`
- [ ] Load analytics on app startup
- [ ] Maintain backward compatibility with existing stats
- [ ] Implement data export to JSON/CSV
- [ ] Implement "clear all data" option

### Lesson Integration (src/content/lesson.rs)
- [ ] Extend `LessonType` enum with `Adaptive` variant
- [ ] Create `Lesson::adaptive_lesson()` factory
- [ ] Add adaptive mode to lesson menu
- [ ] Show "Locked" state when insufficient data
- [ ] Show "Recommended" badge when weak areas exist
- [ ] Display current focus areas in menu

### UI Integration (src/app.rs, src/ui/render.rs)
- [ ] Add adaptive mode to main menu
- [ ] Create `render_adaptive_info()` for pre-session display
- [ ] Show weak keys and focus areas
- [ ] Show session goals (e.g., "Improve 'd' to 85%")
- [ ] Create `render_adaptive_results()` for post-session feedback
- [ ] Show per-key improvement (before → after)
- [ ] Show next recommendation
- [ ] Show overall progress (X/26 keys proficient)
- [ ] Add progress indicators to menu

### Testing
- [ ] Test mastery level calculation
- [ ] Test weak key identification with mock data
- [ ] Test slow key identification
- [ ] Test spaced repetition interval calculation
- [ ] Test adaptive content generation
- [ ] Test recommendation engine logic
- [ ] Test data persistence (save/load)
- [ ] Integration test: complete session → update analytics → save
- [ ] Test with various edge cases (no data, all mastered, etc.)

### Documentation
- [ ] Update README with adaptive mode feature
- [ ] Document analytics data format
- [ ] Add adaptive mode user guide
- [ ] Update CLAUDE.md with adaptive algorithms

## Phase 3: Advanced Features

### Visual Progress Tracking
- [ ] Heat map visualization of key accuracy
- [ ] Trend graphs (accuracy over time)
- [ ] Improvement velocity charts
- [ ] Mastery level badges
- [ ] Per-key detailed statistics view

### Advanced Analytics
- [ ] Machine learning for better predictions
- [ ] Pattern recognition (common error types)
- [ ] Optimal practice schedule recommendations
- [ ] Fatigue detection (declining performance)
- [ ] Rest recommendations
- [ ] Plateau detection and intervention

### Gamification
- [ ] Achievement system for mastery milestones
- [ ] Streak tracking for consistent practice
- [ ] Daily/weekly challenges
- [ ] Progress celebrations
- [ ] Virtual rewards for improvement

### Multi-Session Analysis
- [ ] Compare current session to historical average
- [ ] Identify long-term trends
- [ ] Predict time to proficiency
- [ ] Generate personalized learning paths
- [ ] Adaptive difficulty adjustment

## Implementation Notes

### Priority Order
1. Core analytics engine (keystroke tracking, stats)
2. Weakness detection (identify problem areas)
3. Adaptive content generation (targeted practice)
4. UI integration (display and feedback)
5. Recommendation engine (next lesson suggestions)
6. Advanced features (visualization, gamification)

### Estimated Complexity
- **Data structures**: Medium (stats tracking, serialization)
- **Analytics engine**: High (real-time tracking, complex calculations)
- **Adaptive algorithms**: High (spaced repetition, weighted generation)
- **UI integration**: Medium (new screens, progress display)
- **Overall**: High complexity, 7-10 days of work

### Dependencies
- `rand` crate for randomized content generation (already used?)
- `serde` for serialization (already in use)
- No additional dependencies required for core features

### Testing Strategy
- Extensive unit tests for analytics calculations
- Mock data for testing edge cases
- Integration tests with simulated sessions
- Long-term testing with real typing data
- A/B testing adaptive vs. standard lessons
- User studies to validate effectiveness

### Privacy Considerations
- All data stored locally (no cloud sync)
- User controls data export/deletion
- Transparent JSON format for inspection
- No personally identifiable information
- Optional: encrypt stats file

### Algorithm Tuning
**Key parameters to tune:**
- Weak key threshold: 80% accuracy (configurable?)
- Slow key percentile: 75th percentile (top 25% slowest)
- Minimum attempts before analysis: 10 attempts
- Content distribution: 60% weak, 30% moderate, 10% strong
- Mastery levels: < 70%, 70-85%, 85-95%, > 95%
- Spaced repetition intervals: tuned per mastery level

These should be validated with real user data and adjusted as needed.

### Performance Optimizations
- Incremental analytics updates (don't recompute everything)
- Lazy computation (calculate only when needed)
- Efficient data structures (HashMap for O(1) lookups)
- Keep only last 100 sessions in memory
- Archive older data to separate file
- Background analytics computation (don't block UI)

### Validation Metrics
**How to measure adaptive mode effectiveness:**
- Average improvement rate (WPM gain per session)
- Time to mastery (sessions needed to reach 95%)
- Weak key resolution rate (% of weak keys improved)
- User engagement (sessions per week)
- Subjective satisfaction (user survey)

Should be compared against standard lesson progression.

### Edge Cases to Handle
- New user (no data) → recommend standard lessons
- All keys mastered → recommend new content types
- Inconsistent performance → lower confidence, broader practice
- Rapid improvement → accelerate progression
- Declining performance → easier content, rest suggestion
- Limited practice variety → force diversity
- Single problem key → intensive focused drill
