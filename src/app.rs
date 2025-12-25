use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;
use std::time::Duration;

use crate::content::{ContentGenerator, Lesson};
use crate::data::{SessionRecord, Stats, Storage};
use crate::engine::{calculate_results, TypingSession};
use crate::ui;

/// Application state
#[derive(Debug, PartialEq)]
enum AppState {
    Running,
    Completed,
    Quit,
}

/// Main application
pub struct App {
    session: TypingSession,
    state: AppState,
    storage: Storage,
    stats: Stats,
}

impl App {
    pub fn new() -> io::Result<Self> {
        // Load first home row lesson
        let lessons = Lesson::home_row_lessons();
        let first_lesson = &lessons[0];
        let content = first_lesson.generate(50); // 50 chars for MVP

        let storage = Storage::new()?;
        let stats = storage.load()?;

        let mut session = TypingSession::new(content);
        session.start();

        Ok(Self {
            session,
            state: AppState::Running,
            storage,
            stats,
        })
    }

    /// Main app entry point
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            // Calculate current stats
            let result = calculate_results(&self.session);

            // Render
            terminal.draw(|f| match self.state {
                AppState::Running | AppState::Completed => {
                    if self.session.is_complete() {
                        ui::render_results(f, result.wpm, result.accuracy, result.duration, result.error_count);
                    } else {
                        ui::render(f, &self.session, result.wpm, result.accuracy);
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
            if self.session.is_complete() && self.state == AppState::Running {
                self.state = AppState::Completed;
                self.save_session()?;
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
            AppState::Running => {
                match key.code {
                    KeyCode::Esc => {
                        self.state = AppState::Quit;
                    }
                    KeyCode::Char(c) => {
                        self.session.add_input(c);
                    }
                    KeyCode::Backspace => {
                        self.session.remove_last_input();
                    }
                    _ => {}
                }
            }
            AppState::Completed => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.state = AppState::Quit;
                    }
                    KeyCode::Char('r') => {
                        // Restart
                        self.restart()?;
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
        let result = calculate_results(&self.session);

        let record = SessionRecord::new(
            "HomeRow-1".to_string(),
            result.wpm,
            result.accuracy,
            result.duration,
        );

        self.stats.add_session(record);
        self.storage.save(&self.stats)?;

        Ok(())
    }

    /// Restart new session
    fn restart(&mut self) -> io::Result<()> {
        let lessons = Lesson::home_row_lessons();
        let first_lesson = &lessons[0];
        let content = first_lesson.generate(50);

        let mut session = TypingSession::new(content);
        session.start();

        self.session = session;
        self.state = AppState::Running;

        Ok(())
    }
}
