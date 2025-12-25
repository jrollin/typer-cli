# Typing Session - Requirements

> **Purpose**: Captures requirements for the core typing session engine
> **Module**: `src/engine/`
> **Next Step**: See `design.md` for technical implementation

## Core Input and Feedback Requirements

### R1: Character-by-Character Input
WHEN a user types a character during a lesson
THE SYSTEM SHALL immediately validate it against the expected character

### R2: Visual Feedback - Correct Input
WHEN a user types a correct character
THE SYSTEM SHALL display it in green color

### R3: Visual Feedback - Incorrect Input
WHEN a user types an incorrect character
THE SYSTEM SHALL display it in red color

### R4: Visual Feedback - Pending Input
WHEN a character has not yet been typed
THE SYSTEM SHALL display it in gray color

### R5: Cursor Position
WHEN a typing session is active
THE SYSTEM SHALL display a cursor at the current input position

### R6: Real-time WPM Calculation
WHEN a user is typing
THE SYSTEM SHALL calculate and display words per minute (WPM) in real-time

### R7: Real-time Accuracy Calculation
WHEN a user is typing
THE SYSTEM SHALL calculate and display accuracy percentage in real-time

### R8: Session Timer
WHEN a typing session starts
THE SYSTEM SHALL display elapsed time in MM:SS format

### R9: Session Completion Detection
WHEN a user types all characters in a lesson
THE SYSTEM SHALL end the session and display results

## Scoring Requirements

### R27: WPM Calculation Formula
THE SYSTEM SHALL calculate WPM as: (character_count / 5) / (duration_in_seconds / 60)

### R28: Accuracy Calculation Formula
THE SYSTEM SHALL calculate accuracy as: (correct_chars / total_chars) × 100

### R29: Real-time Metric Updates
WHEN a user types a character
THE SYSTEM SHALL immediately recalculate and update WPM and accuracy displays

## Performance Requirements

### R32: Performance
THE SYSTEM SHALL process user input with no perceptible lag (<50ms)
