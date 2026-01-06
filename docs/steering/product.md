# Typer CLI - Product Overview

> **Purpose**: Defines your product's purpose, target users, key features, and business objectives
> **Type**: Steering Document - Persistent knowledge about the project
> **Related**: See `tech.md` for technology choices, `structure.md` for architecture

## Product Vision

Typer CLI is a terminal-native typing trainer designed specifically for developers who want to master touch typing on AZERTY keyboards. It provides distraction-free practice with immediate feedback, real-time metrics, and code-focused exercises.

## Target Users

### Primary Audience
- **French-speaking developers** learning or improving AZERTY touch typing
- **Terminal enthusiasts** who prefer CLI tools over GUI applications
- **Programmers** who want to practice typing with code symbols and patterns

### User Characteristics
- Comfortable with terminal environments
- Value minimalism and focused tools
- Want to improve typing speed for coding efficiency
- May be switching from hunt-and-peck to touch typing
- Prefer data-driven feedback on progress

## Problem Statement

### Current Pain Points
1. **GUI typing trainers are distracting**: Most typing tutors use flashy GUIs that break developer workflow
2. **Poor AZERTY support**: Many typing trainers focus on QWERTY, with AZERTY as an afterthought
3. **Generic practice content**: Few trainers offer code-specific practice (symbols, common programming patterns)
4. **Context switching cost**: Having to leave the terminal to practice typing disrupts focus

### Our Solution
A lightweight, terminal-based typing trainer that:
- Runs directly in the terminal where developers already work
- Provides first-class AZERTY support
- Offers code-focused practice exercises
- Gives instant feedback and real-time metrics
- Maintains minimal, distraction-free interface

## Key Features

### Phase 1 (MVP)
- **Home Row Practice**: Progressive lessons for AZERTY home row (qsdfghjklm)
- **Real-time Feedback**: Immediate visual indication (green/red) of correct/incorrect characters
- **Live Metrics**: WPM and accuracy calculated and displayed in real-time
- **Session Persistence**: Automatic saving of practice statistics
- **Minimal Interface**: Clean TUI with just essential information

### Phase 2 (Completed)
- **Bigram Training**: Common French (40 with accents), English (30), and Code (10) letter pairs
- **Trigram Training**: French (25 with accents) and English (20) three-letter sequences
- **Code Symbols**: TypeScript, Rust, Python syntax patterns (18 lessons total)
- **Common Words**: French and English most frequent words practice

### Phase 2+ (Completed)
- **Adaptive Mode**: Automatically focus on keys with highest error rates
- **Analytics Tracking**: Per-key and per-bigram statistics with mastery levels
- **Weakness Detection**: Identifies keys < 80% accuracy or slow speed
- **Spaced Repetition**: Practice intervals based on mastery level
- **Personalized Content**: 60% weak, 30% moderate, 10% strong key distribution

### Phase 3 (Completed)
- **Keyboard Visualization**: Full AZERTY layout with real-time next-key highlighting
- **Finger Color Hints**: Toggle finger placement guides (Ctrl+F)
- **Accuracy Heatmap**: Performance overlay on keyboard (Ctrl+H)
- **Layout Improvements**: Reorganized interface for better usability
- **Finger Training**: 24 bilateral finger lessons (4 pairs × 6 variants)
- **Two-Level Menu**: Category-based navigation system for improved lesson discovery

### Phase 3+ (Future)
- **Progress Tracking**: Historical graphs showing WPM and accuracy trends over time
- **Enhanced Analytics UI**: Pre/post-session feedback, progress indicators
- **Data Export**: JSON/CSV export for external analysis
- **Custom Lessons**: User-defined practice content
- **Gamification**: Optional achievements and streaks
- **Themes**: Customizable color schemes

## Key Differentiators

1. **Terminal-Native**: No GUI bloat, runs where developers already work
2. **AZERTY First**: Designed for AZERTY from the ground up, not a QWERTY port
3. **Code-Focused**: Specific training for programming symbols and patterns
4. **Zero Distractions**: No sounds, no animations, no unnecessary chrome
5. **Developer-Friendly**: Built with Rust, open architecture, extensible

## Success Metrics

### User Success
- **Speed Improvement**: Users increase WPM by 10+ within 2 weeks of regular practice
- **Accuracy**: Users maintain 95%+ accuracy while improving speed
- **Retention**: Users return to practice at least 3x per week

### Product Success
- **Adoption**: Active users in developer community
- **Completion Rate**: Users complete at least 5 practice sessions
- **Engagement**: Average session duration of 5+ minutes

## Non-Goals (What We Won't Do)

- ❌ **Gamification**: No points, badges, or achievements (distraction-free focus)
- ❌ **Social Features**: No leaderboards, sharing, or competition
- ❌ **Audio Feedback**: No sound effects or music
- ❌ **Multiple Keyboard Layouts**: AZERTY only (Phase 1-3), may expand later
- ❌ **Mobile/Web Versions**: Terminal-only, leveraging native environment
- ❌ **Lesson Marketplace**: No user-generated content sharing platform

## User Journey

### First-Time User
1. Install/compile typer-cli
2. Run `cargo run` or `./typer-cli`
3. See category selection menu with 6 lesson types
4. Select a category (e.g., "Row Training")
5. Choose a lesson from filtered list
6. Select practice duration (1-10 minutes)
7. Receive instant feedback on each keystroke
8. Complete first session, see results and stats
9. Stats automatically saved for next session

### Returning User
1. Launch application
2. Browse categories to find desired lesson type
3. Select lesson from filtered category view
4. Track improvement via real-time metrics
5. Build muscle memory through consistent practice
6. Return to same category after session completion

### Advanced User
1. Use Adaptive mode (unlocks after 10 sessions) to focus on weak keys
2. Practice specific finger pairs with targeted drills
3. Master code symbols for programming languages
4. Toggle keyboard display features (finger hints, heatmap)
5. Track per-key and per-bigram statistics automatically
6. Progress through spaced repetition intervals

## Design Principles

1. **Simplicity First**: Every feature must justify its existence
2. **Immediate Feedback**: No waiting, no batch processing
3. **Data Transparency**: Show users exactly how they're performing
4. **Respect Terminal Conventions**: Use standard controls and layouts
5. **No Regression**: Once a feature works, it keeps working
6. **Performance Matters**: Typing feedback must be instantaneous (<50ms)

## Competitive Landscape

### Alternatives
- **gtypist**: Terminal typing tutor, but QWERTY-focused, dated interface
- **Typing.com, TypingClub**: GUI-based, distracting, poor AZERTY support
- **klavaro**: GUI application, generic content, not code-focused
- **MonkeyType (web)**: Good metrics, but browser-based, not code-specific

### Our Advantage
- Only AZERTY-first terminal typing trainer
- Specifically designed for developer workflow
- Code-specific practice content
- Rust-powered performance and reliability
- Minimal, distraction-free design

## Future Vision

### Long-term Goals
- Become the standard typing trainer for French-speaking developers
- Support additional layouts (BÉPO, Dvorak) based on demand
- Integration with coding practice platforms
- Plugin system for custom lesson types

### Extensibility
- Modular architecture allows easy addition of new lesson types
- JSON-based stats enable third-party analysis tools
- Clean separation of concerns for community contributions
