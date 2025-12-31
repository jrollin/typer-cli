# Adaptive Mode - Requirements

> **Purpose**: Captures requirements for personalized adaptive training
> **Module**: `src/engine/` and `src/content/` (extension)
> **Next Step**: See `design.md` for adaptive algorithm design

## Adaptive Training Requirements

### US-1 Error Pattern Detection
WHEN a user completes typing sessions
THE SYSTEM SHALL analyze error patterns to identify weak keys and bigrams
- Track which keys have highest error rates
- Track which bigrams are slowest
- Track which keys are frequently mistyped as others

### US-2 Weak Key Identification
THE SYSTEM SHALL identify a user's weakest keys based on multiple metrics:
- Accuracy rate per key (errors / total attempts)
- Speed per key (average time to type correctly)
- Consistency (variance in typing speed/accuracy)
- Minimum threshold: 10 attempts before considering

### US-3 Adaptive Lesson Generation
WHEN a user starts an adaptive lesson
THE SYSTEM SHALL generate practice content focused on identified weak areas
- 60% weak keys/bigrams
- 30% moderately difficult keys/bigrams
- 10% mastered keys/bigrams (retention)

### US-4 Difficulty Progression
THE SYSTEM SHALL automatically adjust difficulty based on performance:
- Increase difficulty when accuracy > 95% for 3 consecutive sessions
- Decrease difficulty when accuracy < 80% for 2 consecutive sessions
- Maintain difficulty when accuracy is 80-95%

### US-5 Spaced Repetition
THE SYSTEM SHALL use spaced repetition principles for weak key practice:
- Recently failed keys: practice immediately
- Moderately weak keys: practice every 2-3 sessions
- Previously weak but improving: practice every 5-7 sessions
- Mastered keys: occasional review

## Analytics Requirements

### US-6 Per-Key Statistics
THE SYSTEM SHALL track detailed statistics per key:
- Total attempts
- Correct attempts
- Error count
- Average time per keystroke
- Most common mistype (what key was pressed instead)
- Trend over time (improving/declining/stable)

### US-7 Per-Bigram Statistics
THE SYSTEM SHALL track detailed statistics per bigram:
- Total attempts
- Correct sequences
- Error count
- Average time for the bigram
- Most common errors within the bigram

### US-8 Session History Analysis
THE SYSTEM SHALL maintain session history for trend analysis:
- Last 30 sessions minimum
- Per-session metrics (WPM, accuracy, duration)
- Per-session weak keys
- Overall improvement trajectory

### US-9 Mastery Levels
THE SYSTEM SHALL classify keys/bigrams into mastery levels:
- **Beginner**: < 70% accuracy or < 5 attempts
- **Learning**: 70-85% accuracy, improving trend
- **Proficient**: 85-95% accuracy, stable performance
- **Mastered**: > 95% accuracy, 20+ successful attempts

## Adaptive Algorithm Requirements

### US-10 Minimum Data Threshold
THE SYSTEM SHALL require minimum data before enabling adaptive mode:
- At least 10 completed sessions
- At least 100 total keystrokes
- Coverage of at least 80% of home row keys

### US-11 Recommendation Engine
WHEN a user completes a session
THE SYSTEM SHALL recommend the most beneficial next practice:
- Specific weak keys identified
- Suggested lesson type (home row, bigrams, code)
- Estimated time to improvement
- Confidence level in recommendation

### US-12 Progress Visualization (Phase 3+)
FUTURE: Display visual progress indicators
- Heat map of key accuracy
- Trend graphs over time
- Improvement velocity charts
- Mastery level badges

### US-13 Adaptive Content Mixing
THE SYSTEM SHALL intelligently mix content types in adaptive lessons:
- Single key repetition for very weak keys
- Bigrams containing weak keys
- Words using weak keys
- Realistic sentences with weak keys

## Integration Requirements

### US-14 Adaptive Mode Menu Option
THE SYSTEM SHALL add adaptive training to the lesson selection menu
- Show "Adaptive Mode (Recommended)" when sufficient data exists
- Show "Adaptive Mode (Locked)" with requirements when insufficient data
- Display current focus areas when hovering

### US-15 Adaptive Session Feedback
WHEN a user completes an adaptive session
THE SYSTEM SHALL provide specific feedback:
- Which weak keys showed improvement
- Which keys still need work
- Next recommended practice focus
- Estimated sessions until proficiency

### US-16 Manual Override
THE SYSTEM SHALL allow users to manually select focus areas:
- Choose specific keys to practice
- Choose specific bigrams to practice
- Disable adaptive mode and use standard lessons
- Reset adaptive data

## Privacy and Data Requirements

### US-17 Local Data Storage
THE SYSTEM SHALL store all adaptive learning data locally
- No cloud sync required
- User data stays on device
- JSON format for transparency

### US-18 Data Export
THE SYSTEM SHALL allow users to export their statistics
- Export to JSON format
- Export to CSV for analysis
- Clear all data option (privacy)

## Future Phase Requirements

### US-19 Advanced Analytics (Phase 3+)
FUTURE: Machine learning models for better prediction
FUTURE: Pattern recognition across multiple users
FUTURE: Optimal practice schedule recommendations
FUTURE: Fatigue detection and rest recommendations

### US-20 Gamification (Phase 3+)
FUTURE: Achievement badges for mastery
FUTURE: Streak tracking for consistent practice
FUTURE: Challenges based on weak areas
FUTURE: Progress milestones and celebrations
