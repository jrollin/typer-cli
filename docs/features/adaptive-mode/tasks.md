# Adaptive Mode - Task Tracking

> **Purpose**: Implementation progress for adaptive personalized training
> **Modules**: `src/engine/analytics.rs`, `src/content/adaptive.rs`
> **Status**: ⏳ PLANNED (Phase 2+)

## Phase 2+: Core Implementation ✅ COMPLETED

### Data Structures (src/engine/analytics.rs) ✅
- [x] Define `KeyStats` struct (attempts, accuracy, timing, mistypes)
- [x] Define `BigramStats` struct (attempts, accuracy, timing)
- [x] Define `MasteryLevel` enum (Beginner, Learning, Proficient, Mastered)
- [x] Define `AdaptiveAnalytics` struct (key stats, bigram stats, history)
- [x] Define `SessionAnalytics` struct (session-level metrics)
- [x] Implement accuracy/speed calculation methods
- [x] Implement mastery level classification logic

### Analytics Engine (src/engine/analytics.rs) ✅
- [x] Implement `SessionAnalyzer` for keystroke tracking
- [x] Record keystroke timing per character
- [x] Track errors and mistype patterns
- [x] Calculate per-key performance metrics
- [x] Calculate per-bigram performance metrics
- [x] Build session summary statistics
- [x] Integrate with existing `TypingSession`

### Weakness Detection (src/engine/adaptive.rs) ✅
- [x] Implement `WeaknessDetector` struct
- [x] `identify_weak_keys()` based on accuracy threshold (< 80%, min 10 attempts)
- [x] `identify_slow_keys()` based on timing percentile (75th percentile)
- [x] `identify_weak_bigrams()` for two-letter combinations (< 85%, min 5 attempts)
- [x] Implement minimum data threshold
- [x] Sort by severity (worst first)
- [x] Return top 5 weak areas

### Spaced Repetition (src/engine/adaptive.rs) ✅
- [x] Implement `SpacedRepetition` algorithm
- [x] Calculate next practice interval by mastery level
- [x] Check if key needs practice based on last practice time
- [x] Implement interval adjustment based on performance
- [x] Support for beginner → learning → proficient → mastered progression

### Adaptive Content Generation (src/content/adaptive_generator.rs) ✅
- [x] Implement `AdaptiveLessonGenerator` struct
- [x] Generate content with 60% weak keys
- [x] Generate content with 30% moderate keys
- [x] Generate content with 10% mastered keys (retention)
- [x] Implement pattern variety (repetitions, alternations, triplets)
- [x] Use weak key focus in generated content
- [x] Fallback to balanced practice when no weak areas
- [x] Randomized pattern generation for variety

### Recommendation Engine (src/engine/adaptive.rs) ✅
- [x] Implement `RecommendationEngine` struct
- [x] Analyze user progress and identify next best lesson
- [x] Generate recommendation with reason and confidence
- [x] Handle insufficient data case (< 10 sessions)
- [x] Recommend adaptive mode when weak areas exist
- [x] Recommend standard lessons when proficient
- [x] Recommend new content (bigrams, code) for variety

### Data Persistence (src/data/stats.rs extension) ✅
- [x] Extend `Stats` struct with `adaptive_analytics` field
- [x] Implement incremental analytics update after each session
- [x] Save to `~/.config/typer-cli/stats.json`
- [x] Load analytics on app startup
- [x] Maintain backward compatibility with existing stats (optional field)
- [ ] Implement data export to JSON/CSV (Phase 3)
- [ ] Implement "clear all data" option (Phase 3)

### Lesson Integration (src/content/lesson.rs) ✅
- [x] Extend `LessonType` enum with `Adaptive` variant
- [x] Create `Lesson::adaptive_lesson()` factory
- [x] Add adaptive mode to lesson menu (conditional on sufficient data)
- [x] Show adaptive mode only when >= 10 sessions and >= 100 keystrokes
- [ ] Show "Recommended" badge when weak areas exist (Phase 3)
- [ ] Display current focus areas in menu (Phase 3)

### UI Integration (src/app.rs) ✅
- [x] Add adaptive mode to main menu (conditional)
- [x] Integrate adaptive content generation in start_lesson()
- [x] Handle insufficient data gracefully
- [ ] Create `render_adaptive_info()` for pre-session display (Phase 3)
- [ ] Show weak keys and focus areas (Phase 3)
- [ ] Show session goals (e.g., "Improve 'd' to 85%") (Phase 3)
- [ ] Create `render_adaptive_results()` for post-session feedback (Phase 3)
- [ ] Show per-key improvement (before → after) (Phase 3)
- [ ] Show next recommendation (Phase 3)
- [ ] Show overall progress (X/26 keys proficient) (Phase 3)

### Testing ✅
- [x] Test mastery level calculation (4 tests)
- [x] Test weak key identification with mock data
- [x] Test slow key identification with percentile logic
- [x] Test spaced repetition interval calculation
- [x] Test adaptive content generation (6 tests)
- [x] Test recommendation engine logic (3 tests)
- [x] Test session analyzer with mock session data
- [x] 81 total tests passing (up from 66)

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
