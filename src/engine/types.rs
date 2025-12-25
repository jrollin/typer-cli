use std::time::{Duration, Instant};

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
}

impl TypingSession {
    pub fn new(content: String) -> Self {
        Self {
            content,
            current_index: 0,
            inputs: Vec::new(),
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn add_input(&mut self, typed: char) -> bool {
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
        self.current_index >= self.content.chars().count()
    }

    pub fn duration(&self) -> Duration {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end.duration_since(start),
            (Some(start), None) => start.elapsed(),
            _ => Duration::default(),
        }
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
        let session = TypingSession::new("hello".to_string());
        assert_eq!(session.content, "hello");
        assert_eq!(session.current_index, 0);
        assert!(session.inputs.is_empty());
        assert!(session.start_time.is_none());
    }

    #[test]
    fn test_typing_session_start() {
        let mut session = TypingSession::new("hello".to_string());
        session.start();
        assert!(session.start_time.is_some());
    }

    #[test]
    fn test_typing_session_add_input() {
        let mut session = TypingSession::new("ab".to_string());
        session.start();

        assert!(session.add_input('a')); // correct
        assert_eq!(session.current_index, 1);
        assert_eq!(session.inputs.len(), 1);

        assert!(!session.add_input('c')); // incorrect
        assert_eq!(session.current_index, 2);
        assert_eq!(session.inputs.len(), 2);
        assert!(session.is_complete());
    }

    #[test]
    fn test_typing_session_complete() {
        let mut session = TypingSession::new("hi".to_string());
        session.start();

        assert!(!session.is_complete());
        session.add_input('h');
        assert!(!session.is_complete());
        session.add_input('i');
        assert!(session.is_complete());
        assert!(session.end_time.is_some());
    }

    #[test]
    fn test_typing_session_backspace() {
        let mut session = TypingSession::new("abc".to_string());
        session.start();

        session.add_input('a');
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
        let mut session = TypingSession::new("abc".to_string());
        session.start();

        // Backspace at start should return false and do nothing
        assert!(!session.remove_last_input());
        assert_eq!(session.current_index, 0);
        assert_eq!(session.inputs.len(), 0);
    }

    #[test]
    fn test_typing_session_backspace_after_completion() {
        let mut session = TypingSession::new("ab".to_string());
        session.start();

        session.add_input('a');
        session.add_input('b');
        assert!(session.is_complete());
        assert!(session.end_time.is_some());

        // Backspace after completion should clear end_time
        assert!(session.remove_last_input());
        assert!(!session.is_complete());
        assert!(session.end_time.is_none());
    }
}
