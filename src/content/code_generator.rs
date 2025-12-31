/// Content generator for code symbol practice lessons
use super::code_symbols::{
    python_snippets, rust_snippets, typescript_snippets, CodeSnippet, ProgrammingLanguage,
};

pub struct CodeSymbolGenerator {
    snippets: Vec<CodeSnippet>,
}

impl CodeSymbolGenerator {
    pub fn new(language: ProgrammingLanguage) -> Self {
        let snippets = match language {
            ProgrammingLanguage::TypeScript => typescript_snippets(),
            ProgrammingLanguage::Rust => rust_snippets(),
            ProgrammingLanguage::Python => python_snippets(),
        };

        Self { snippets }
    }

    /// Generate code practice content for a given level
    /// Levels 1-6: Progressive difficulty from basic brackets to complex code
    pub fn generate(&self, level: usize, length: usize) -> String {
        let filtered_snippets: Vec<_> = self
            .snippets
            .iter()
            .filter(|s| s.difficulty <= level as u8)
            .collect();

        if filtered_snippets.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let mut idx = 0;

        while result.len() < length {
            if !result.is_empty() {
                result.push('\n');
            }

            let snippet = &filtered_snippets[idx % filtered_snippets.len()];
            result.push_str(snippet.template);
            idx += 1;
        }

        result.chars().take(length).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_generation() {
        let gen = CodeSymbolGenerator::new(ProgrammingLanguage::TypeScript);
        let content = gen.generate(1, 50);

        assert!(!content.is_empty());
        assert!(content.len() <= 50);
    }

    #[test]
    fn test_rust_generation() {
        let gen = CodeSymbolGenerator::new(ProgrammingLanguage::Rust);
        let content = gen.generate(1, 50);

        assert!(!content.is_empty());
        assert!(content.len() <= 50);
    }

    #[test]
    fn test_python_generation() {
        let gen = CodeSymbolGenerator::new(ProgrammingLanguage::Python);
        let content = gen.generate(1, 50);

        assert!(!content.is_empty());
        assert!(content.len() <= 50);
    }

    #[test]
    fn test_progressive_difficulty() {
        let gen = CodeSymbolGenerator::new(ProgrammingLanguage::TypeScript);

        let level1 = gen.generate(1, 100);
        let level6 = gen.generate(6, 100);

        // Both should generate content
        assert!(!level1.is_empty());
        assert!(!level6.is_empty());

        // Level 6 should have more variety (more snippets available)
        // We can't directly test complexity, but we can ensure it generates
    }

    #[test]
    fn test_deterministic_generation() {
        let gen = CodeSymbolGenerator::new(ProgrammingLanguage::Rust);

        let content1 = gen.generate(3, 80);
        let content2 = gen.generate(3, 80);

        // Same level and length should produce same content
        assert_eq!(content1, content2);
    }

    #[test]
    fn test_language_specific_content() {
        let ts_gen = CodeSymbolGenerator::new(ProgrammingLanguage::TypeScript);
        let ts_content = ts_gen.generate(6, 150);

        // TypeScript should have => arrows or : type annotations
        assert!(ts_content.contains("=>") || ts_content.contains(":"));

        let rust_gen = CodeSymbolGenerator::new(ProgrammingLanguage::Rust);
        let rust_content = rust_gen.generate(6, 150);

        // Rust should have recognizable Rust syntax
        assert!(
            rust_content.contains("fn")
                || rust_content.contains("let")
                || rust_content.contains("vec")
        );

        let py_gen = CodeSymbolGenerator::new(ProgrammingLanguage::Python);
        let py_content = py_gen.generate(6, 150);

        // Python should have : colons for function definitions or def keyword
        assert!(py_content.contains(":") || py_content.contains("def"));
    }

    #[test]
    fn test_all_levels_generate_content() {
        let gen = CodeSymbolGenerator::new(ProgrammingLanguage::TypeScript);

        for level in 1..=6 {
            let content = gen.generate(level, 60);
            assert!(
                !content.is_empty(),
                "Level {} should generate content",
                level
            );
        }
    }
}
