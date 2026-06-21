//! Shared drill-generation logic for n-gram lessons (bigrams, trigrams).
//!
//! Bigram and Trigram generators previously duplicated the drill/word/mixed-mode
//! loops verbatim; the only difference was the item type. These generic functions
//! operate over any [`DrillItem`] so the control flow lives in one place.

use super::bigram::Bigram;
use super::trigram::Trigram;
use rand::Rng;

/// A practice n-gram exposing its raw pattern and example words.
pub trait DrillItem {
    fn pattern(&self) -> &str;
    fn examples(&self) -> &[String];
}

impl DrillItem for Bigram {
    fn pattern(&self) -> &str {
        &self.pattern
    }
    fn examples(&self) -> &[String] {
        &self.examples
    }
}

impl DrillItem for Trigram {
    fn pattern(&self) -> &str {
        &self.pattern
    }
    fn examples(&self) -> &[String] {
        &self.examples
    }
}

/// Random space/newline separator (25% newline) appended between practice chunks.
fn push_separator(result: &mut String, rng: &mut impl Rng) {
    let separator = if rng.gen_bool(0.25) { '\n' } else { ' ' };
    result.push(separator);
}

/// Level 1: pure pattern repetition, e.g. "qu qu qu ou ou ou".
pub fn generate_drill_mode<T: DrillItem>(items: &[&T], length: usize) -> String {
    let mut result = String::new();
    let mut rng = rand::thread_rng();
    let mut idx = 0;

    while result.chars().count() < length {
        if !result.is_empty() {
            push_separator(&mut result, &mut rng);
        }

        let pattern = items[idx % items.len()].pattern();
        result.push_str(&format!("{pattern} {pattern} {pattern}"));
        idx += 1;
    }

    result.chars().take(length).collect()
}

/// Level 2: patterns in word context, cycling through each item's examples.
pub fn generate_word_mode<T: DrillItem>(items: &[&T], length: usize) -> String {
    let mut result = String::new();
    let mut rng = rand::thread_rng();
    let mut idx = 0;

    while result.chars().count() < length {
        if !result.is_empty() {
            push_separator(&mut result, &mut rng);
        }

        let item = items[idx % items.len()];
        let example_idx = (idx / items.len()) % item.examples().len();
        result.push_str(&item.examples()[example_idx]);
        idx += 1;
    }

    result.chars().take(length).collect()
}

/// Level 3-4: realistic sentences mixing the target patterns' example words.
pub fn generate_mixed_mode<T: DrillItem>(items: &[&T], length: usize) -> String {
    let mut result = String::new();
    let mut rng = rand::thread_rng();
    let mut word_count = 0;

    while result.chars().count() < length {
        if word_count > 0 {
            push_separator(&mut result, &mut rng);
        }

        let item = items[word_count % items.len()];
        let example_idx = (word_count / items.len()) % item.examples().len();
        result.push_str(&item.examples()[example_idx]);
        word_count += 1;
    }

    result.chars().take(length).collect()
}
