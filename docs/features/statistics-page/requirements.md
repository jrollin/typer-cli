# Statistics Page - Requirements

## Purpose
Provide users with comprehensive performance analytics to track progress and identify areas for improvement.

## User Stories

### US-1: Access Statistics
WHEN a user presses 's' from the category menu
THE SYSTEM SHALL display the statistics visualization page

### US-2: View Overall Performance
WHEN a user views the statistics page
THE SYSTEM SHALL display overall session metrics including:
- Total sessions completed
- Total keystrokes typed
- Average WPM across all sessions
- Average accuracy across all sessions

### US-3: View Mastery Breakdown
WHEN a user views the statistics page
THE SYSTEM SHALL display a count of keys in each mastery level:
- Mastered (>95% accuracy)
- Proficient (85-95% accuracy)
- Learning (70-85% accuracy)
- Beginner (<70% accuracy)

### US-4: Identify Weaknesses
WHEN a user views the statistics page
THE SYSTEM SHALL display the top 10 keys with accuracy below 80%
AND sort them by accuracy (lowest first)
AND exclude keys with fewer than 5 attempts

### US-5: View Common Errors
WHEN a user views the statistics page
THE SYSTEM SHALL display the top 5 most common mistype patterns
AND show which key was expected and which was typed instead

### US-6: Visual Keyboard Heatmap
WHEN a user views the statistics page
THE SYSTEM SHALL display a visual AZERTY keyboard with heatmap coloring
AND color each key based on its accuracy:
- Green: 90%+ (Mastered)
- Yellow: 80-90% (Good)
- LightRed: 70-80% (Learning)
- Red: <70% (Weak)

### US-7: No Data Handling
WHEN a user accesses statistics with no session data
THE SYSTEM SHALL display a placeholder message encouraging first session completion

### US-8: Navigation
WHEN a user presses ESC or 'q' on the statistics page
THE SYSTEM SHALL return to the category menu

## Acceptance Criteria
- Statistics accessible via 's' key from main menu
- Layout responsive to terminal size (minimum 80x24)
- Colors consistent with application theme
- All metrics calculated accurately from stored analytics
- Placeholder shown gracefully when no data exists
