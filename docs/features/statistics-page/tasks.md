# Statistics Page - Implementation Tasks

## Phase 1: Core Implementation

### State Machine Changes (src/app.rs)
- [x] Add `Statistics` variant to AppState enum (line 24)
- [x] Add render call in main loop (~line 295)
- [x] Add 's' key handler in LessonTypeMenu state (~line 375)
- [x] Add ESC/q handler for Statistics state (~line 510)

### UI Rendering (src/ui/render.rs)
- [x] Create `render_statistics()` main function (~line 923)
- [x] Implement 3-section layout (Header | Content | Instructions)
- [x] Implement 2-column content split (40% / 60%)
- [x] Create `render_overall_stats_block()` helper
- [x] Create `render_mastery_breakdown()` helper
- [x] Create `render_weaknesses_list()` helper
- [x] Create `render_common_mistypes()` helper
- [x] Integrate keyboard heatmap rendering
- [x] Add placeholder for no data scenario
- [x] Update category menu instructions (line 843)

## Phase 2: Testing
- [x] Test with no data (fresh install)
- [x] Test with analytics data (10+ sessions)
- [x] Test perfect performance (no weaknesses)
- [x] Test navigation flow (menu → stats → menu)
- [x] Test terminal resize (80x24, 100x30, 120x40)
- [x] Verify color accuracy matches mastery levels
- [x] Cross-check displayed stats with JSON file

## Phase 3: Documentation
- [x] Update CLAUDE.md with Phase 3.5 section
- [x] Update README.md with Statistics Dashboard feature
- [x] Create feature documentation (requirements.md, design.md, tasks.md)
- [x] Update docs/README.md index if needed

## Phase 4: Polish
- [x] Verify consistent color scheme
- [x] Test edge cases (no weaknesses, no mistypes)
- [x] Ensure proper error handling
- [x] Code review for consistency with app patterns
