# TUI Interface - Requirements

> **Purpose**: Captures requirements for the terminal user interface
> **Module**: `src/ui/`
> **Next Step**: See `design.md` for UI layout and rendering details

## User Interface Requirements

### R18: Minimal TUI Layout
THE SYSTEM SHALL display a terminal user interface with:
- Header showing mode/lesson name
- Text to type display area
- User input display area
- Statistics panel (WPM, Accuracy, Time)

### R19: Text Display Area
THE SYSTEM SHALL display the expected text to be typed above the user's input

### R20: Input Display Area
THE SYSTEM SHALL display the user's input with color-coded characters

### R21: Statistics Panel
THE SYSTEM SHALL display current WPM, accuracy percentage, and elapsed time

### R22: Results Screen
WHEN a session completes
THE SYSTEM SHALL display final results including:
- Final WPM
- Final accuracy
- Total duration
- Character count
- Error count

## Control Requirements

### R23: Restart Session
WHEN a session completes
UPON the user pressing 'r' key
THE SYSTEM SHALL start a new session with the same lesson

### R24: Quit After Session
WHEN a session completes
UPON the user pressing 'q' key
THE SYSTEM SHALL exit the application

### R25: Emergency Exit
UPON the user pressing ESC key at any time
THE SYSTEM SHALL immediately exit the application

### R26: Character Input Only
WHEN a typing session is active
THE SYSTEM SHALL accept only printable character input (no backspace in Phase 1)

## Quality Requirements

### R30: No Sound Effects
THE SYSTEM SHALL NOT produce any sound effects or audio feedback

### R31: Terminal Compatibility
THE SYSTEM SHALL run in any terminal supporting ANSI colors and cursor control

## Phase 1 MVP Scope Constraints

### R34: Single Lesson Mode
FOR Phase 1 MVP
THE SYSTEM SHALL automatically start with home row lesson level 1 (f and j)

### R35: No Lesson Selection
FOR Phase 1 MVP
THE SYSTEM SHALL NOT provide lesson selection UI

### R36: No Command Line Arguments
FOR Phase 1 MVP
THE SYSTEM SHALL NOT process command line arguments for lesson selection

### R37: No Backspace Support
FOR Phase 1 MVP
THE SYSTEM SHALL NOT allow backspace or correction of typed characters

## Future Phase Requirements (Out of MVP Scope)

### R39: Adaptive Mode (Phase 2+)
FUTURE: Track individual key error rates
FUTURE: Generate practice sessions focusing on problematic keys

### R41: Keyboard Visualization (Phase 2+)
FUTURE: Display AZERTY keyboard layout with highlighted keys

### R44: CLI Arguments (Phase 2+)
FUTURE: Support --lesson argument for lesson selection

### R45: Backspace Support (Phase 2+)
FUTURE: Allow correction of typing mistakes with backspace key
