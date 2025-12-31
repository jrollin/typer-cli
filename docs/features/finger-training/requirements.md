# Finger Training - Requirements

## Overview
Finger-based typing lessons that target specific finger pairs with progressive difficulty levels, allowing users to strengthen individual fingers and improve muscle memory for the French AZERTY layout.

## User Stories

### Core Functionality

### US-1 Generate Finger Training Content
WHEN a user selects a finger training lesson
THE SYSTEM SHALL generate practice content using only keys assigned to the selected finger pair

### US-2 Display Finger Training When Ready
WHEN a user completes 10 or more typing sessions
THE SYSTEM SHALL display finger training lessons in the menu after adaptive mode

### US-3 Provide Multiple Difficulty Variants
WHEN a user selects a finger pair lesson
THE SYSTEM SHALL provide 6 difficulty variants (3 levels × 2 modes)

### Difficulty Progression

### US-4 Level 1 Home Row Keys
WHEN a user selects Level 1 (Home Row)
THE SYSTEM SHALL include only home row keys for the selected finger pair

### US-5 Level 2 Extended Keys
WHEN a user selects Level 2 (Extended)
THE SYSTEM SHALL include home row, top row, and bottom row keys for the selected finger pair

### US-6 Level 3 All Keys
WHEN a user selects Level 3 (All Keys)
THE SYSTEM SHALL include all keys including number row symbols for the selected finger pair

### Shift Variants

### US-7 Base Lesson Without Shift
WHEN a user selects a base lesson (without shift)
THE SYSTEM SHALL generate drills using only base characters

### US-8 Shift Variant With Mixed Characters
WHEN a user selects a shift variant lesson
THE SYSTEM SHALL generate drills with mixed case using 50% lowercase, 40% uppercase, and 10% symbols

### Finger Pairs

### US-9 Create Four Finger Pairs
WHEN the system generates finger training lessons
THE SYSTEM SHALL create lessons for 4 finger pairs: Pinky, Ring, Middle, and Index

### US-10 Use Correct AZERTY Mappings
WHEN the system assigns keys to finger pairs
THE SYSTEM SHALL use corrected French AZERTY mappings where symbols are base and numbers are shift

### Content Generation

### US-11 Three-Phase Drill Pattern
WHEN generating drill content for finger lessons
THE SYSTEM SHALL use a 3-phase pattern: repetitions, then pairs, then triplets

### US-12 Randomly Distribute Characters
WHEN generating shift variant drills
THE SYSTEM SHALL randomly distribute characters according to the weighted distribution

### US-13 Exclude Placeholder Characters
WHEN generating content
THE SYSTEM SHALL exclude placeholder characters (null, newline) from practice

### Menu Organization

### US-14 Display Finger Training Section
WHEN displaying the lesson menu
THE SYSTEM SHALL show finger training lessons in a dedicated section with green separator

### US-15 Order With Adaptive Mode
WHEN adaptive mode is available
THE SYSTEM SHALL order lessons: Adaptive first, Finger Training second, Primary third, Secondary last

### US-16 Order Without Adaptive Mode
WHEN adaptive mode is not available
THE SYSTEM SHALL order lessons: Finger Training first, Primary second, Secondary third

## Acceptance Criteria

- 24 finger training lessons available (4 pairs × 6 variants)
- Each finger pair has 3 difficulty levels with base and shift variants
- Lessons numbered sequentially after adaptive mode
- Green "FINGER TRAINING" separator in menu
- Content generation excludes placeholder keys
- Shift variants follow 50/40/10 distribution
- All keys correctly mapped to French AZERTY finger assignments

## Constraints

- French AZERTY layout only
- Number row: symbols are base, numbers require shift
- Thumb excluded (spacebar only, not meaningful for practice)
- Content length respects specified limit without overflow
