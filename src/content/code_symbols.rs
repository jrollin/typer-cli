/// Code symbol practice for programming languages
/// Provides realistic code snippets for TypeScript, Rust, Python
/// Programming language selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgrammingLanguage {
    TypeScript,
    Rust,
    Python,
}

/// Symbol category for progressive learning
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum SymbolCategory {
    Brackets,   // () [] {} <>
    BasicOps,   // + - * / = !
    Comparison, // == != < > <= >=
    Arrows,     // -> => ::
    Compound,   // && || ++ -- += -=
    Special,    // . , ; : ? @ #
}

/// Code snippet template with difficulty
#[derive(Debug, Clone)]
pub struct CodeSnippet {
    #[allow(dead_code)]
    pub category: SymbolCategory,
    pub template: &'static str,
    pub difficulty: u8, // 1-6
}

impl CodeSnippet {
    const fn new(category: SymbolCategory, template: &'static str, difficulty: u8) -> Self {
        Self {
            category,
            template,
            difficulty,
        }
    }
}

/// TypeScript code snippets (6 levels)
pub fn typescript_snippets() -> Vec<CodeSnippet> {
    vec![
        // Level 1: Brackets
        CodeSnippet::new(SymbolCategory::Brackets, "const arr = [1, 2, 3];", 1),
        CodeSnippet::new(SymbolCategory::Brackets, "const obj = { key: value };", 1),
        CodeSnippet::new(SymbolCategory::Brackets, "function test() {}", 1),
        // Level 2: Basic operators
        CodeSnippet::new(SymbolCategory::BasicOps, "let x = 5 + 3;", 2),
        CodeSnippet::new(SymbolCategory::BasicOps, "const result = a * b - c;", 2),
        CodeSnippet::new(SymbolCategory::BasicOps, "x += 10;", 2),
        // Level 3: Comparisons
        CodeSnippet::new(SymbolCategory::Comparison, "if (x === 5) {}", 3),
        CodeSnippet::new(SymbolCategory::Comparison, "while (i < 10) { i++; }", 3),
        CodeSnippet::new(SymbolCategory::Comparison, "if (x !== null && x > 0) {}", 3),
        // Level 4: Arrows and special
        CodeSnippet::new(SymbolCategory::Arrows, "const fn = (x) => x * 2;", 4),
        CodeSnippet::new(SymbolCategory::Arrows, "arr.map(x => x + 1);", 4),
        CodeSnippet::new(SymbolCategory::Special, "const value: number = 42;", 4),
        // Level 5: Compound operators
        CodeSnippet::new(SymbolCategory::Compound, "x ||= defaultValue;", 5),
        CodeSnippet::new(SymbolCategory::Compound, "if (a && b || c) {}", 5),
        CodeSnippet::new(SymbolCategory::Compound, "result ??= fallback;", 5),
        // Level 6: Complex realistic code
        CodeSnippet::new(
            SymbolCategory::Arrows,
            "const sum = (arr: number[]): number => arr.reduce((a, b) => a + b, 0);",
            6,
        ),
        CodeSnippet::new(
            SymbolCategory::Brackets,
            "type User = { name: string; age: number };",
            6,
        ),
        CodeSnippet::new(
            SymbolCategory::Arrows,
            "const filtered = items.filter(x => x.active).map(x => x.value);",
            6,
        ),
    ]
}

/// Rust code snippets (6 levels)
pub fn rust_snippets() -> Vec<CodeSnippet> {
    vec![
        // Level 1: Brackets
        CodeSnippet::new(SymbolCategory::Brackets, "let vec = vec![1, 2, 3];", 1),
        CodeSnippet::new(SymbolCategory::Brackets, "fn main() {}", 1),
        CodeSnippet::new(
            SymbolCategory::Brackets,
            "struct Point { x: i32, y: i32 }",
            1,
        ),
        // Level 2: Basic operators
        CodeSnippet::new(SymbolCategory::BasicOps, "let x = 5 + 3;", 2),
        CodeSnippet::new(SymbolCategory::BasicOps, "let result = a * b - c;", 2),
        CodeSnippet::new(SymbolCategory::BasicOps, "x += 10;", 2),
        // Level 3: Comparisons
        CodeSnippet::new(SymbolCategory::Comparison, "if x == 5 {}", 3),
        CodeSnippet::new(SymbolCategory::Comparison, "while i < 10 { i += 1; }", 3),
        CodeSnippet::new(SymbolCategory::Comparison, "if x != 0 && x > 5 {}", 3),
        // Level 4: Arrows and special
        CodeSnippet::new(SymbolCategory::Arrows, "fn add(x: i32) -> i32 { x + 1 }", 4),
        CodeSnippet::new(
            SymbolCategory::Arrows,
            "match x { 0 => \"zero\", _ => \"other\" }",
            4,
        ),
        CodeSnippet::new(SymbolCategory::Special, "use std::collections::HashMap;", 4),
        // Level 5: Compound and complex
        CodeSnippet::new(SymbolCategory::Arrows, "|x| x * 2", 5),
        CodeSnippet::new(SymbolCategory::Special, "impl<T> Trait for Type<T> {}", 5),
        CodeSnippet::new(SymbolCategory::Arrows, "Vec::<i32>::new()", 5),
        // Level 6: Complex realistic code
        CodeSnippet::new(
            SymbolCategory::Arrows,
            "fn sum(arr: &[i32]) -> i32 { arr.iter().fold(0, |a, &b| a + b) }",
            6,
        ),
        CodeSnippet::new(
            SymbolCategory::Arrows,
            "let result: Vec<_> = items.iter().filter(|&&x| x > 0).collect();",
            6,
        ),
        CodeSnippet::new(
            SymbolCategory::Special,
            "pub struct Config<'a> { name: &'a str }",
            6,
        ),
    ]
}

/// Python code snippets (6 levels)
pub fn python_snippets() -> Vec<CodeSnippet> {
    vec![
        // Level 1: Brackets
        CodeSnippet::new(SymbolCategory::Brackets, "arr = [1, 2, 3]", 1),
        CodeSnippet::new(SymbolCategory::Brackets, "def test():", 1),
        CodeSnippet::new(SymbolCategory::Brackets, "obj = {\"key\": \"value\"}", 1),
        // Level 2: Basic operators
        CodeSnippet::new(SymbolCategory::BasicOps, "x = 5 + 3", 2),
        CodeSnippet::new(SymbolCategory::BasicOps, "result = a * b - c", 2),
        CodeSnippet::new(SymbolCategory::BasicOps, "x += 10", 2),
        // Level 3: Comparisons
        CodeSnippet::new(SymbolCategory::Comparison, "if x == 5:", 3),
        CodeSnippet::new(SymbolCategory::Comparison, "while i < 10: i += 1", 3),
        CodeSnippet::new(SymbolCategory::Comparison, "if x != 0 and x > 5:", 3),
        // Level 4: Special symbols
        CodeSnippet::new(SymbolCategory::Special, "def func(x: int) -> int:", 4),
        CodeSnippet::new(SymbolCategory::Brackets, "result = [x for x in arr]", 4),
        CodeSnippet::new(SymbolCategory::Special, "@decorator", 4),
        // Level 5: Complex structures
        CodeSnippet::new(SymbolCategory::Brackets, "{k: v for k, v in items}", 5),
        CodeSnippet::new(SymbolCategory::Special, "lambda x: x * 2", 5),
        CodeSnippet::new(SymbolCategory::Brackets, "print(f\"value: {x}\")", 5),
        // Level 6: Complex realistic code
        CodeSnippet::new(
            SymbolCategory::Brackets,
            "def sum(arr: list[int]) -> int: return sum(x for x in arr)",
            6,
        ),
        CodeSnippet::new(
            SymbolCategory::Brackets,
            "result = [x * 2 for x in items if x > 0]",
            6,
        ),
        CodeSnippet::new(
            SymbolCategory::Special,
            "class Config: name: str; value: int",
            6,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_snippets_count() {
        let snippets = typescript_snippets();
        assert!(!snippets.is_empty());
        assert!(snippets.len() >= 18); // At least 3 per level x 6 levels
    }

    #[test]
    fn test_rust_snippets_count() {
        let snippets = rust_snippets();
        assert!(!snippets.is_empty());
        assert!(snippets.len() >= 18);
    }

    #[test]
    fn test_python_snippets_count() {
        let snippets = python_snippets();
        assert!(!snippets.is_empty());
        assert!(snippets.len() >= 18);
    }

    #[test]
    fn test_difficulty_progression() {
        let snippets = typescript_snippets();

        // Check that we have snippets at each difficulty level
        for level in 1..=6 {
            let has_level = snippets.iter().any(|s| s.difficulty == level);
            assert!(
                has_level,
                "TypeScript should have snippets at level {}",
                level
            );
        }
    }

    #[test]
    fn test_snippets_not_empty() {
        for snippet in typescript_snippets() {
            assert!(!snippet.template.is_empty());
        }
        for snippet in rust_snippets() {
            assert!(!snippet.template.is_empty());
        }
        for snippet in python_snippets() {
            assert!(!snippet.template.is_empty());
        }
    }
}
