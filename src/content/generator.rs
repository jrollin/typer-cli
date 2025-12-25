use super::lesson::{Lesson, LessonType};

/// Trait pour générer du contenu de leçon
pub trait ContentGenerator {
    fn generate(&self, length: usize) -> String;
}

impl ContentGenerator for Lesson {
    fn generate(&self, length: usize) -> String {
        match &self.lesson_type {
            LessonType::HomeRow { level } => match level {
                1..=4 => generate_two_key_drills(&self.keys, length),
                5 => generate_all_keys_drills(&self.keys, length),
                6 => generate_words(&self.keys, length),
                _ => String::new(),
            },
        }
    }
}

/// Générer des drills avec 2 touches (niveau 1-4)
/// Pattern: "ff jj ff jj dd kk dd kk"
fn generate_two_key_drills(keys: &[char], length: usize) -> String {
    if keys.len() != 2 {
        return String::new();
    }

    let mut result = String::new();
    let pattern = [
        format!("{}{}", keys[0], keys[0]),
        format!("{}{}", keys[1], keys[1]),
    ];

    let mut idx = 0;
    while result.len() < length {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&pattern[idx % pattern.len()]);
        idx += 1;
    }

    result.chars().take(length).collect()
}

/// Générer des drills avec toutes les touches home row
/// Pattern: mélange de toutes les touches
fn generate_all_keys_drills(_keys: &[char], length: usize) -> String {
    let mut result = String::new();

    // Patterns de 2-3 caractères avec les touches home row
    let patterns = vec![
        "ff", "jj", "dd", "kk", "ss", "ll", "qq", "mm",
        "fj", "dk", "sl", "qm", "fd", "jk", "ds", "kl",
        "fds", "jkl", "qsd", "mlk",
    ];

    let mut idx = 0;
    while result.len() < length {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(patterns[idx % patterns.len()]);
        idx += 1;
    }

    result.chars().take(length).collect()
}

/// Générer des mots simples français avec les touches home row
/// Mots possibles avec q,s,d,f,g,h,j,k,l,m: limité mais quelques mots existent
fn generate_words(_keys: &[char], length: usize) -> String {
    // Mots courts français possibles avec home row AZERTY
    // Note: très limité, principalement pour démonstration
    let words = vec![
        "la", "le", "de", "se", "me", "je",
        "mal", "sel", "les", "des", "mes",
    ];

    let mut result = String::new();
    let mut idx = 0;

    while result.len() < length {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(words[idx % words.len()]);
        idx += 1;
    }

    result.chars().take(length).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_two_key_drills() {
        let result = generate_two_key_drills(&['f', 'j'], 15);
        assert!(result.starts_with("ff jj ff jj"));
        assert!(result.len() <= 15);
    }

    #[test]
    fn test_generate_all_keys_drills() {
        let keys = vec!['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'];
        let result = generate_all_keys_drills(&keys, 30);
        assert!(!result.is_empty());
        assert!(result.len() <= 30);
    }

    #[test]
    fn test_generate_words() {
        let keys = vec!['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'];
        let result = generate_words(&keys, 20);
        assert!(result.contains("la") || result.contains("le") || result.contains("de"));
        assert!(result.len() <= 20);
    }

    #[test]
    fn test_lesson_content_generator() {
        let lessons = Lesson::home_row_lessons();

        // Test niveau 1 (ff jj)
        let content1 = lessons[0].generate(20);
        assert!(!content1.is_empty());
        assert!(content1.contains('f'));
        assert!(content1.contains('j'));

        // Test niveau 5 (all keys)
        let content5 = lessons[4].generate(30);
        assert!(!content5.is_empty());

        // Test niveau 6 (words)
        let content6 = lessons[5].generate(25);
        assert!(!content6.is_empty());
    }
}
