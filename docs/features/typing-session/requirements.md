# Typing Session - Requirements

> **Purpose**: Captures requirements for the core typing session engine
> **Module**: `src/engine/`
> **Next Step**: See `design.md` for technical implementation

## Core Input and Feedback Requirements

### US-1 Character-by-Character Input
WHEN a user types a character during a lesson
THE SYSTEM SHALL immediately validate it against the expected character

### US-2 Visual Feedback - Correct Input
WHEN a user types a correct character
THE SYSTEM SHALL display it in green color

### US-3 Visual Feedback - Incorrect Input
WHEN a user types an incorrect character
THE SYSTEM SHALL display it in red color

### US-4 Visual Feedback - Pending Input
WHEN a character has not yet been typed
THE SYSTEM SHALL display it in gray color

### US-5 Cursor Position
WHEN a typing session is active
THE SYSTEM SHALL display a cursor at the current input position

### US-6 Real-time WPM Calculation
WHEN a user is typing
THE SYSTEM SHALL calculate and display words per minute (WPM) in real-time

### US-7 Real-time Accuracy Calculation
WHEN a user is typing
THE SYSTEM SHALL calculate and display accuracy percentage in real-time

### US-8 Session Timer
WHEN a typing session starts
THE SYSTEM SHALL display elapsed time in MM:SS format

### US-9 Session Completion Detection
WHEN a user types all characters in a lesson
THE SYSTEM SHALL end the session and display results

## Scoring Requirements

### US-10 WPM Calculation Formula
THE SYSTEM SHALL calculate WPM as: (character_count / 5) / (duration_in_seconds / 60)

### US-11 Accuracy Calculation Formula
THE SYSTEM SHALL calculate accuracy as: (correct_chars / total_chars) × 100

### US-12 Real-time Metric Updates
WHEN a user types a character
THE SYSTEM SHALL immediately recalculate and update WPM and accuracy displays

## Performance Requirements

### US-13 Performance
THE SYSTEM SHALL process user input with no perceptible lag (<50ms)
