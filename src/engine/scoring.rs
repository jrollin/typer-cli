use super::types::{SessionResult, TypingSession};
use std::time::Duration;

/// Calculer les résultats d'une session
pub fn calculate_results(session: &TypingSession) -> SessionResult {
    let char_count = session.inputs.len();
    let error_count = session.inputs.iter().filter(|i| !i.is_correct).count();
    let accuracy = if char_count > 0 {
        ((char_count - error_count) as f64 / char_count as f64) * 100.0
    } else {
        0.0
    };

    let duration = session.duration();
    let wpm = calculate_wpm(char_count, duration);

    SessionResult::new(wpm, accuracy, duration, char_count, error_count)
}

/// Calculer WPM (Words Per Minute)
/// Formule standard: (caractères / 5) / (temps en minutes)
/// On divise par 5 car la moyenne d'un mot anglais est ~5 caractères
pub fn calculate_wpm(char_count: usize, duration: Duration) -> f64 {
    let minutes = duration.as_secs_f64() / 60.0;

    if minutes <= 0.0 {
        return 0.0;
    }

    let words = char_count as f64 / 5.0;
    words / minutes
}

/// Calculer l'accuracy en pourcentage
#[allow(dead_code)]
pub fn calculate_accuracy(correct: usize, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    (correct as f64 / total as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_wpm_zero_duration() {
        let wpm = calculate_wpm(100, Duration::from_secs(0));
        assert_eq!(wpm, 0.0);
    }

    #[test]
    fn test_calculate_wpm_one_minute() {
        // 100 caractères en 1 minute = 20 mots/min
        let wpm = calculate_wpm(100, Duration::from_secs(60));
        assert!((wpm - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_wpm_thirty_seconds() {
        // 50 caractères en 30 secondes = 20 mots/min
        let wpm = calculate_wpm(50, Duration::from_secs(30));
        assert!((wpm - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_accuracy_perfect() {
        let accuracy = calculate_accuracy(100, 100);
        assert_eq!(accuracy, 100.0);
    }

    #[test]
    fn test_calculate_accuracy_half() {
        let accuracy = calculate_accuracy(50, 100);
        assert_eq!(accuracy, 50.0);
    }

    #[test]
    fn test_calculate_accuracy_zero_total() {
        let accuracy = calculate_accuracy(0, 0);
        assert_eq!(accuracy, 0.0);
    }

    #[test]
    fn test_calculate_results() {
        let mut session = TypingSession::new("hello".to_string());
        session.start();

        session.add_input('h'); // correct

        // Simuler du temps entre les frappes
        std::thread::sleep(Duration::from_millis(100));

        session.add_input('e'); // correct
        session.add_input('x'); // incorrect
        session.add_input('l'); // correct
        session.add_input('o'); // correct

        let result = calculate_results(&session);

        assert_eq!(result.char_count, 5);
        assert_eq!(result.error_count, 1);
        assert_eq!(result.accuracy, 80.0); // 4/5 = 80%
        // La durée devrait être au moins 100ms
        assert!(result.duration.as_millis() >= 100);
        // Avec au moins 100ms et 5 caractères, WPM devrait être > 0
        assert!(result.wpm > 0.0, "WPM was {} for duration {:?}", result.wpm, result.duration);
    }
}
