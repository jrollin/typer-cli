use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;
use std::time::Duration;

use crate::content::{BigramType, ContentGenerator, Language, Lesson, ProgrammingLanguage};
use crate::data::{SessionRecord, Stats, Storage};
use crate::engine::{calculate_results, TypingSession};
use crate::ui;

/// Application state
#[derive(Debug, PartialEq)]
enum AppState {
    Menu,
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
}

impl App {
    pub fn new() -> io::Result<Self> {
        let storage = Storage::new()?;
        let stats = storage.load()?;

        // Build complete lesson list
        let mut lessons = Vec::new();

        // Home Row lessons (6 lessons)
        lessons.extend(Lesson::home_row_lessons());

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

        Ok(Self {
            session: None,
            state: AppState::Menu,
            storage,
            stats,
            selected_lesson: 0,
            lessons,
        })
    }

    fn start_lesson(&mut self, lesson_index: usize) {
        let lesson = &self.lessons[lesson_index];
        let content = lesson.generate(80); // Generate 80 chars for practice

        let mut session = TypingSession::new(content);
        session.start();

        self.session = Some(session);
        self.state = AppState::Running;
    }

    /// Main app entry point
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            // Render
            terminal.draw(|f| match self.state {
                AppState::Menu => {
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
                            ui::render(f, session, result.wpm, result.accuracy);
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

            // Check session completion
            if let Some(session) = &self.session {
                if session.is_complete() && self.state == AppState::Running {
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
            AppState::Menu => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.state = AppState::Quit;
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
                    self.state = AppState::Menu;
                    self.session = None;
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
                        self.state = AppState::Menu;
                        self.session = None;
                    }
                    KeyCode::Char('r') => {
                        // Restart same lesson
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

            let record = SessionRecord::new(
                lesson.title.clone(),
                result.wpm,
                result.accuracy,
                result.duration,
            );

            self.stats.add_session(record);
            self.storage.save(&self.stats)?;
        }

        Ok(())
    }
}
