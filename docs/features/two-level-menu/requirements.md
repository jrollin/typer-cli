# Two-Level Menu System - Requirements

## Overview
Hierarchical navigation system to organize lessons into categories for improved discoverability and user experience.

## User Stories

### Navigation Flow
WHEN a user launches the application
THE SYSTEM SHALL display a category selection menu as the first screen

WHEN a user views the category menu
THE SYSTEM SHALL display 5 lesson categories: Adaptive, Finger Training, Key Training, Languages, and Code

WHEN a user views a category in the menu
THE SYSTEM SHALL display the category name, a one-line description, and a lesson count

WHEN a user selects a category with Enter or Space
THE SYSTEM SHALL navigate to a filtered lesson menu showing only lessons from that category

WHEN a user presses a number key (1-5) in the category menu
THE SYSTEM SHALL directly select and open the corresponding category

### Lesson Selection
WHEN a user views the lesson menu
THE SYSTEM SHALL display only lessons belonging to the selected category

WHEN a user views the lesson menu header
THE SYSTEM SHALL display the category name in the header (e.g., "TYPER CLI - Finger Training Lessons")

WHEN a user selects a lesson from the filtered menu
THE SYSTEM SHALL proceed to duration selection as before

### ESC Navigation
WHEN a user presses ESC in the lesson menu
THE SYSTEM SHALL return to the category selection menu

WHEN a user presses ESC in the category menu
THE SYSTEM SHALL quit the application

WHEN a user presses ESC in the duration menu
THE SYSTEM SHALL return to the filtered lesson menu

### Session Completion
WHEN a user completes a typing session
THE SYSTEM SHALL return to the filtered lesson menu for the same category

WHEN a user presses ESC during a typing session
THE SYSTEM SHALL return to the filtered lesson menu for the same category

### Visual Presentation
WHEN a user views a category
THE SYSTEM SHALL display categories with color coding: Adaptive (Cyan), Finger Training (Green), Key Training (Cyan), Languages (Yellow), Code (Magenta)

WHEN a user views a selected category
THE SYSTEM SHALL highlight the selection in yellow with bold text

WHEN a user views an unselected category
THE SYSTEM SHALL display it in its category-specific color

### Category Filtering
WHEN the system filters lessons by category
THE SYSTEM SHALL include lessons where:
- Adaptive category: lesson type is Adaptive
- Finger Training category: lesson type is FingerPair
- Key Training category: lesson type is KeyPair or KeyPairGroup
- Languages category: lesson type is Bigram (Natural), Trigram, or CommonWords
- Code category: lesson type is CodeSymbols or Bigram (Code)

### Conditional Display
WHEN the user has completed fewer than 10 sessions
THE SYSTEM SHALL not display the Adaptive category

WHEN the user has completed 10 or more sessions with 100+ keystrokes
THE SYSTEM SHALL display the Adaptive category as the first option

## Acceptance Criteria

### Functional
- ✅ Category menu displays as first screen on app launch
- ✅ 5 categories shown with descriptions and color coding
- ✅ Number keys 1-5 select categories directly
- ✅ Lesson menu shows only filtered lessons for selected category
- ✅ Category name appears in lesson menu header
- ✅ ESC returns from lessons to categories, from categories to quit
- ✅ Session completion returns to filtered lesson menu
- ✅ Adaptive category conditionally displays based on session count

### Non-Functional
- ✅ Navigation is intuitive and follows expected hierarchy
- ✅ Visual design is consistent with existing menu screens
- ✅ All existing lesson functionality remains unchanged
- ✅ No breaking changes to lesson generation or session logic
- ✅ Performance remains fast with filtering operations

## Out of Scope
- Multi-select categories
- Custom category creation
- Category reordering
- Search functionality within categories
- Category-level statistics
