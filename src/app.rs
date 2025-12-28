use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;
use std::time::Duration;

use crate::content::{
    AdaptiveLessonGenerator, BigramType, ContentGenerator, Language, Lesson, ProgrammingLanguage,
};
use crate::data::{SessionRecord, Stats, Storage};
use crate::engine::{calculate_results, SessionAnalyzer, TypingSession};
use crate::keyboard::AzertyLayout;
use crate::ui;

/// Application state
#[derive(Debug, PartialEq)]
enum AppState {
    DurationMenu,
    LessonMenu,
    Running,
    Completed,
    Quit,
}

/// Main application
pub struct App {
    session: Option<TypingSession>,
    state: AppState,
    storage: Storage,
    stats: Stats,
    selected_lesson: usize,
    lessons: Vec<Lesson>,
    selected_duration: usize,
    selected_duration_value: crate::engine::SessionDuration,
    keyboard_visible: bool,
    keyboard_layout: AzertyLayout,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let storage = Storage::new()?;
        let stats = storage.load()?;

        // Build complete lesson list with grouped organization
        let mut lessons = Vec::new();

        // PRIMARY SECTION: Key Training (25 lessons)
        // Organized: individual lessons → group → shift variant

        // Group 1: Lessons 1-4 (basic home row pairs)
        let key_pairs_1_4: Vec<_> = Lesson::key_pair_lessons().into_iter().take(4).collect();
        lessons.extend(key_pairs_1_4);
        lessons.extend(Lesson::key_pair_group_lessons(false).into_iter().take(1)); // Group 1-4
        lessons.extend(Lesson::key_pair_group_lessons(true).into_iter().take(1)); // Group 1-4 + Shift

        // Group 2: Lessons 5-8 (extended reaches)
        let key_pairs_5_8: Vec<_> = Lesson::key_pair_lessons()
            .into_iter()
            .skip(4)
            .take(4)
            .collect();
        lessons.extend(key_pairs_5_8);
        lessons.extend(
            Lesson::key_pair_group_lessons(false)
                .into_iter()
                .skip(1)
                .take(1),
        ); // Group 5-8
        lessons.extend(
            Lesson::key_pair_group_lessons(true)
                .into_iter()
                .skip(1)
                .take(1),
        ); // Group 5-8 + Shift

        // Group 3: Lessons 9-12 (bottom row)
        let key_pairs_9_12: Vec<_> = Lesson::key_pair_lessons()
            .into_iter()
            .skip(8)
            .take(4)
            .collect();
        lessons.extend(key_pairs_9_12);
        lessons.extend(
            Lesson::key_pair_group_lessons(false)
                .into_iter()
                .skip(2)
                .take(1),
        ); // Group 9-12
        lessons.extend(
            Lesson::key_pair_group_lessons(true)
                .into_iter()
                .skip(2)
                .take(1),
        ); // Group 9-12 + Shift

        // Group 4: Lessons 13-17 (numbers and symbols)
        let key_pairs_13_17: Vec<_> = Lesson::key_pair_lessons()
            .into_iter()
            .skip(12)
            .take(5)
            .collect();
        lessons.extend(key_pairs_13_17);
        lessons.extend(
            Lesson::key_pair_group_lessons(false)
                .into_iter()
                .skip(3)
                .take(1),
        ); // Group 13-17
        lessons.extend(
            Lesson::key_pair_group_lessons(true)
                .into_iter()
                .skip(3)
                .take(1),
        ); // Group 13-17 + Shift

        // SECONDARY SECTION: Programming & Languages (27 lessons)

        // French Bigram lessons (3 lessons)
        lessons.extend(Lesson::bigram_lessons(
            BigramType::Natural,
            Some(Language::French),
        ));

        // English Bigram lessons (3 lessons)
        lessons.extend(Lesson::bigram_lessons(
            BigramType::Natural,
            Some(Language::English),
        ));

        // Code Bigram lessons (3 lessons)
        lessons.extend(Lesson::bigram_lessons(BigramType::Code, None));

        // TypeScript Code Symbols (6 lessons)
        lessons.extend(Lesson::code_symbol_lessons(ProgrammingLanguage::TypeScript));

        // Rust Code Symbols (6 lessons)
        lessons.extend(Lesson::code_symbol_lessons(ProgrammingLanguage::Rust));

        // Python Code Symbols (6 lessons)
        lessons.extend(Lesson::code_symbol_lessons(ProgrammingLanguage::Python));

        // ADAPTIVE SECTION (if sufficient data)
        if should_show_adaptive_mode(&stats) {
            lessons.push(Lesson::adaptive_lesson());
        }

        Ok(Self {
            session: None,
            state: AppState::DurationMenu,
            storage,
            stats,
            selected_lesson: 0,
            lessons,
            selected_duration: 2, // Default to 5 minutes (index 2)
            selected_duration_value: crate::engine::SessionDuration::FiveMinutes,
            keyboard_visible: true, // Default visible
            keyboard_layout: AzertyLayout::new(),
        })
    }

    fn start_lesson(&mut self, lesson_index: usize) {
        let lesson = &self.lessons[lesson_index];

        // Generate initial content (500 chars)
        let content = match &lesson.lesson_type {
            crate::content::lesson::LessonType::Adaptive => {
                // Generate adaptive content if analytics available
                if let Some(analytics) = &self.stats.adaptive_analytics {
                    let generator = AdaptiveLessonGenerator::new(analytics);
                    generator.generate(500)
                } else {
                    "Insufficient data for adaptive mode. Complete more sessions first.".to_string()
                }
            }
            _ => lesson.generate(500), // Standard content generation
        };

        let session = TypingSession::new(content, self.selected_duration_value.as_duration());
        // Don't call session.start() - timer starts on first keystroke

        self.session = Some(session);
        self.state = AppState::Running;
    }

    fn generate_more_content(&mut self) {
        if let Some(session) = &mut self.session {
            let lesson = &self.lessons[self.selected_lesson];

            // Generate another 300-char chunk
            let more_content = match &lesson.lesson_type {
                crate::content::lesson::LessonType::Adaptive => {
                    if let Some(analytics) = &self.stats.adaptive_analytics {
                        let generator = AdaptiveLessonGenerator::new(analytics);
                        generator.generate(300)
                    } else {
                        String::new()
                    }
                }
                _ => lesson.generate(300),
            };

            if !more_content.is_empty() {
                session.append_content(more_content);
            }
        }
    }

    /// Main app entry point
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            // Render
            terminal.draw(|f| match self.state {
                AppState::DurationMenu => {
                    ui::render_duration_menu(f, self.selected_duration);
                }
                AppState::LessonMenu => {
                    ui::render_menu(f, &self.lessons, self.selected_lesson);
                }
                AppState::Running | AppState::Completed => {
                    if let Some(session) = &self.session {
                        let result = calculate_results(session);

                        if session.is_complete() {
                            ui::render_results(
                                f,
                                result.wpm,
                                result.accuracy,
                                result.duration,
                                result.error_count,
                            );
                        } else {
                            ui::render(
                                f,
                                session,
                                result.wpm,
                                result.accuracy,
                                self.keyboard_visible,
                                &self.keyboard_layout,
                                &self.stats.adaptive_analytics,
                            );
                        }
                    }
                }
                AppState::Quit => {}
            })?;

            // Handle events
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key_event(key)?;
                }
            }

            // Check if we need to generate more content during active session
            if self.state == AppState::Running {
                if let Some(session) = &self.session {
                    if session.needs_more_content() && !session.is_complete() {
                        self.generate_more_content();
                    }
                }
            }

            // Check session completion
            if let Some(session) = &mut self.session {
                if session.is_complete() && self.state == AppState::Running {
                    // Set end_time if not already set (e.g., time expired)
                    if session.end_time.is_none() {
                        session.end_time = Some(std::time::Instant::now());
                    }
                    self.state = AppState::Completed;
                    self.save_session()?;
                }
            }

            // Quit
            if self.state == AppState::Quit {
                break;
            }
        }

        Ok(())
    }

    /// Handle keyboard events
    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        // Ignore release events
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match self.state {
            AppState::DurationMenu => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.state = AppState::Quit;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.selected_duration > 0 {
                        self.selected_duration -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let max_idx = crate::engine::SessionDuration::all().len() - 1;
                    if self.selected_duration < max_idx {
                        self.selected_duration += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    // Save selected duration and move to lesson menu
                    self.selected_duration_value =
                        crate::engine::SessionDuration::all()[self.selected_duration];
                    self.state = AppState::LessonMenu;
                }
                _ => {}
            },
            AppState::LessonMenu => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    // Go back to duration menu
                    self.state = AppState::DurationMenu;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.selected_lesson > 0 {
                        self.selected_lesson -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.selected_lesson < self.lessons.len() - 1 {
                        self.selected_lesson += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.start_lesson(self.selected_lesson);
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    // Allow direct selection with numbers 1-6
                    if let Some(digit) = c.to_digit(10) {
                        let index = (digit as usize).saturating_sub(1);
                        if index < self.lessons.len() {
                            self.selected_lesson = index;
                            self.start_lesson(index);
                        }
                    }
                }
                _ => {}
            },
            AppState::Running => match key.code {
                KeyCode::Esc => {
                    // Return to duration menu (discard session)
                    self.state = AppState::DurationMenu;
                    self.session = None;
                }
                KeyCode::Tab => {
                    // Toggle keyboard visibility
                    self.keyboard_visible = !self.keyboard_visible;
                }
                KeyCode::Char(c) => {
                    if let Some(session) = &mut self.session {
                        session.add_input(c);
                    }
                }
                KeyCode::Backspace => {
                    if let Some(session) = &mut self.session {
                        session.remove_last_input();
                    }
                }
                _ => {}
            },
            AppState::Completed => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        // Return to duration menu
                        self.state = AppState::DurationMenu;
                        self.session = None;
                    }
                    KeyCode::Char('r') => {
                        // Restart same lesson with same duration
                        self.start_lesson(self.selected_lesson);
                    }
                    _ => {}
                }
            }
            AppState::Quit => {}
        }

        Ok(())
    }

    /// Save session
    fn save_session(&mut self) -> io::Result<()> {
        if let Some(session) = &self.session {
            let result = calculate_results(session);
            let lesson = &self.lessons[self.selected_lesson];

            // Save session record
            let record = SessionRecord::new(
                lesson.title.clone(),
                result.wpm,
                result.accuracy,
                result.duration,
                self.selected_duration_value.as_duration(),
            );
            self.stats.add_session(record);

            // Update adaptive analytics
            let analyzer = SessionAnalyzer::new();
            let analysis = analyzer.analyze_session(session);
            self.stats.update_analytics(session, analysis);

            // Save everything to JSON
            self.storage.save(&self.stats)?;
        }

        Ok(())
    }
}

/// Check if adaptive mode should be shown in the menu
fn should_show_adaptive_mode(stats: &Stats) -> bool {
    if let Some(analytics) = &stats.adaptive_analytics {
        analytics.total_sessions >= 10 && analytics.total_keystrokes >= 100
    } else {
        false
    }
}
