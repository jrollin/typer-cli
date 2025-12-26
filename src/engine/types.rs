use std::time::{Duration, Instant};

/// Session duration presets
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum SessionDuration {
    TwoMinutes,
    ThreeMinutes,
    FiveMinutes,
    TenMinutes,
    FifteenMinutes,
}

impl SessionDuration {
    pub fn as_duration(&self) -> Duration {
        match self {
            Self::TwoMinutes => Duration::from_secs(2 * 60),
            Self::ThreeMinutes => Duration::from_secs(3 * 60),
            Self::FiveMinutes => Duration::from_secs(5 * 60),
            Self::TenMinutes => Duration::from_secs(10 * 60),
            Self::FifteenMinutes => Duration::from_secs(15 * 60),
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::TwoMinutes,
            Self::ThreeMinutes,
            Self::FiveMinutes,
            Self::TenMinutes,
            Self::FifteenMinutes,
        ]
    }

    pub fn label(&self) -> &str {
        match self {
            Self::TwoMinutes => "2 minutes",
            Self::ThreeMinutes => "3 minutes",
            Self::FiveMinutes => "5 minutes",
            Self::TenMinutes => "10 minutes",
            Self::FifteenMinutes => "15 minutes",
        }
    }
}

/// Représente une frappe de caractère individuelle
#[derive(Debug, Clone)]
pub struct CharInput {
    #[allow(dead_code)]
    pub expected: char,
    pub typed: char,
    #[allow(dead_code)]
    pub timestamp: Duration,
    pub is_correct: bool,
}

impl CharInput {
    pub fn new(expected: char, typed: char, timestamp: Duration) -> Self {
        Self {
            expected,
            typed,
            timestamp,
            is_correct: expected == typed,
        }
    }
}

/// Session de typing en cours
#[derive(Debug)]
pub struct TypingSession {
    pub content: String,
    pub current_index: usize,
    pub inputs: Vec<CharInput>,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub duration_limit: Duration,
    pub content_buffer_size: usize,
}

impl TypingSession {
    pub fn new(content: String, duration: Duration) -> Self {
        let buffer_size = content.chars().count();
        Self {
            content,
            current_index: 0,
            inputs: Vec::new(),
            start_time: None,
            end_time: None,
            duration_limit: duration,
            content_buffer_size: buffer_size,
        }
    }

    #[allow(dead_code)]
    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn add_input(&mut self, typed: char) -> bool {
        // Start timer on first keystroke
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        if self.is_complete() {
            return false;
        }

        let expected = self.content.chars().nth(self.current_index).unwrap_or('\0');
        let elapsed = self
            .start_time
            .map(|start| start.elapsed())
            .unwrap_or_default();

        let input = CharInput::new(expected, typed, elapsed);
        let is_correct = input.is_correct;

        self.inputs.push(input);
        self.current_index += 1;

        if self.is_complete() {
            self.end_time = Some(Instant::now());
        }

        is_correct
    }

    pub fn remove_last_input(&mut self) -> bool {
        if self.current_index > 0 && !self.inputs.is_empty() {
            self.inputs.pop();
            self.current_index -= 1;
            self.end_time = None; // Clear end time if we backspace after completion
            true
        } else {
            false
        }
    }

    pub fn is_complete(&self) -> bool {
        // Complete when time expires OR content exhausted
        if let Some(start) = self.start_time {
            if start.elapsed() >= self.duration_limit {
                return true;
            }
        }

        // Fallback: content-based completion
        self.current_index >= self.content.chars().count()
    }

    pub fn duration(&self) -> Duration {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end.duration_since(start),
            (Some(start), None) => start.elapsed(),
            _ => Duration::default(),
        }
    }

    pub fn remaining_time(&self) -> Duration {
        match self.start_time {
            Some(start) => {
                let elapsed = start.elapsed();
                if elapsed >= self.duration_limit {
                    Duration::ZERO
                } else {
                    self.duration_limit - elapsed
                }
            }
            None => self.duration_limit, // Full duration if not started
        }
    }

    pub fn needs_more_content(&self) -> bool {
        // Generate more when user is within 200 chars of buffer end
        let remaining_chars = self.content_buffer_size.saturating_sub(self.current_index);
        remaining_chars < 200
    }

    pub fn append_content(&mut self, new_content: String) {
        self.content.push(' ');
        self.content.push_str(&new_content);
        self.content_buffer_size = self.content.chars().count();
    }
}

/// Résultats d'une session complétée
#[derive(Debug, Clone)]
pub struct SessionResult {
    pub wpm: f64,
    pub accuracy: f64,
    pub duration: Duration,
    #[allow(dead_code)]
    pub char_count: usize,
    pub error_count: usize,
}

impl SessionResult {
    pub fn new(
        wpm: f64,
        accuracy: f64,
        duration: Duration,
        char_count: usize,
        error_count: usize,
    ) -> Self {
        Self {
            wpm,
            accuracy,
            duration,
            char_count,
            error_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_input_correct() {
        let input = CharInput::new('a', 'a', Duration::from_secs(1));
        assert!(input.is_correct);
        assert_eq!(input.expected, 'a');
        assert_eq!(input.typed, 'a');
    }

    #[test]
    fn test_char_input_incorrect() {
        let input = CharInput::new('a', 'b', Duration::from_secs(1));
        assert!(!input.is_correct);
        assert_eq!(input.expected, 'a');
        assert_eq!(input.typed, 'b');
    }

    #[test]
    fn test_typing_session_new() {
        let session = TypingSession::new("hello".to_string(), Duration::from_secs(60));
        assert_eq!(session.content, "hello");
        assert_eq!(session.current_index, 0);
        assert!(session.inputs.is_empty());
        assert!(session.start_time.is_none());
        assert_eq!(session.duration_limit, Duration::from_secs(60));
        assert_eq!(session.content_buffer_size, 5);
    }

    #[test]
    fn test_typing_session_start() {
        let mut session = TypingSession::new("hello".to_string(), Duration::from_secs(60));
        session.start();
        assert!(session.start_time.is_some());
    }

    #[test]
    fn test_typing_session_add_input() {
        let mut session = TypingSession::new("ab".to_string(), Duration::from_secs(60));
        // Don't call start() - timer starts on first input now

        assert!(session.add_input('a')); // correct
        assert_eq!(session.current_index, 1);
        assert_eq!(session.inputs.len(), 1);
        assert!(session.start_time.is_some()); // Timer started on first input

        assert!(!session.add_input('c')); // incorrect
        assert_eq!(session.current_index, 2);
        assert_eq!(session.inputs.len(), 2);
        assert!(session.is_complete());
    }

    #[test]
    fn test_typing_session_complete() {
        let mut session = TypingSession::new("hi".to_string(), Duration::from_secs(60));

        assert!(!session.is_complete());
        session.add_input('h'); // Timer starts here
        assert!(!session.is_complete());
        session.add_input('i');
        assert!(session.is_complete());
        assert!(session.end_time.is_some());
    }

    #[test]
    fn test_typing_session_backspace() {
        let mut session = TypingSession::new("abc".to_string(), Duration::from_secs(60));

        session.add_input('a'); // Timer starts here
        session.add_input('x'); // wrong character
        assert_eq!(session.current_index, 2);
        assert_eq!(session.inputs.len(), 2);

        // Backspace should remove last input
        assert!(session.remove_last_input());
        assert_eq!(session.current_index, 1);
        assert_eq!(session.inputs.len(), 1);

        // Now type correct character
        session.add_input('b');
        assert_eq!(session.current_index, 2);
        assert_eq!(session.inputs.len(), 2);
    }

    #[test]
    fn test_typing_session_backspace_at_start() {
        let mut session = TypingSession::new("abc".to_string(), Duration::from_secs(60));

        // Backspace at start should return false and do nothing
        assert!(!session.remove_last_input());
        assert_eq!(session.current_index, 0);
        assert_eq!(session.inputs.len(), 0);
    }

    #[test]
    fn test_typing_session_backspace_after_completion() {
        let mut session = TypingSession::new("ab".to_string(), Duration::from_secs(60));

        session.add_input('a'); // Timer starts here
        session.add_input('b');
        assert!(session.is_complete());
        assert!(session.end_time.is_some());

        // Backspace after completion should clear end_time
        assert!(session.remove_last_input());
        assert!(!session.is_complete());
        assert!(session.end_time.is_none());
    }
}
