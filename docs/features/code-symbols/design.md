# Code Symbols - Design Document

> **Purpose**: Technical design for programming symbol practice
> **Module**: `src/content/` (extension)
> **Previous Step**: See `requirements.md` for code symbol requirements
> **Related**: See `../bigram-training/design.md` for similar pattern-based generation

## Overview

The code symbols system generates realistic programming practice content focused on symbols, operators, and syntactic elements across different programming languages.

## Architecture

### Data Structures

```rust
// Programming language enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgrammingLanguage {
    TypeScript,
    Rust,
    Python,
    Generic,  // Language-agnostic symbols
}

// Symbol category for progressive learning
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolCategory {
    Brackets,     // () [] {} <>
    BasicOps,     // + - * / = !
    Comparison,   // == != < > <= >=
    Arrows,       // -> => ::
    Compound,     // && || ++ -- += -=
    Special,      // . , ; : ? @ #
}

// Code snippet template
#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub language: ProgrammingLanguage,
    pub category: SymbolCategory,
    pub template: String,
    pub difficulty: u8,  // 1-5
}

// Extended Lesson type
pub enum LessonType {
    HomeRow { level: usize },
    Bigram { /* ... */ },
    CodeSymbols {
        language: ProgrammingLanguage,
        level: usize,
    },
}
```

### Symbol Templates by Language

**TypeScript/JavaScript:**
```rust
const TYPESCRIPT_SNIPPETS: &[CodeSnippet] = &[
    CodeSnippet {
        language: ProgrammingLanguage::TypeScript,
        category: SymbolCategory::Brackets,
        template: "const arr: number[] = [1, 2, 3];",
        difficulty: 1,
    },
    CodeSnippet {
        language: ProgrammingLanguage::TypeScript,
        category: SymbolCategory::Arrows,
        template: "const fn = (a: number, b: number): number => a + b;",
        difficulty: 3,
    },
    CodeSnippet {
        language: ProgrammingLanguage::TypeScript,
        category: SymbolCategory::Comparison,
        template: "if (x !== null && x > 0) { return true; }",
        difficulty: 2,
    },
    // ... more templates
];
```

**Rust:**
```rust
const RUST_SNIPPETS: &[CodeSnippet] = &[
    CodeSnippet {
        language: ProgrammingLanguage::Rust,
        category: SymbolCategory::Arrows,
        template: "fn main() -> Result<(), Error> {}",
        difficulty: 3,
    },
    CodeSnippet {
        language: ProgrammingLanguage::Rust,
        category: SymbolCategory::Brackets,
        template: "let vec = vec![1, 2, 3];",
        difficulty: 1,
    },
    CodeSnippet {
        language: ProgrammingLanguage::Rust,
        category: SymbolCategory::Special,
        template: "impl Trait for Type {}",
        difficulty: 2,
    },
    CodeSnippet {
        language: ProgrammingLanguage::Rust,
        category: SymbolCategory::Arrows,
        template: "match x { 0 => \"zero\", _ => \"other\" }",
        difficulty: 4,
    },
    // ... more templates
];
```

**Python:**
```rust
const PYTHON_SNIPPETS: &[CodeSnippet] = &[
    CodeSnippet {
        language: ProgrammingLanguage::Python,
        category: SymbolCategory::Brackets,
        template: "arr = [1, 2, 3]",
        difficulty: 1,
    },
    CodeSnippet {
        language: ProgrammingLanguage::Python,
        category: SymbolCategory::Special,
        template: "def main():",
        difficulty: 1,
    },
    CodeSnippet {
        language: ProgrammingLanguage::Python,
        category: SymbolCategory::Comparison,
        template: "if x != 0 and x > 0:",
        difficulty: 2,
    },
    CodeSnippet {
        language: ProgrammingLanguage::Python,
        category: SymbolCategory::Brackets,
        template: "dict = {\"key\": \"value\", \"num\": 42}",
        difficulty: 3,
    },
    // ... more templates
];
```

## Content Generation

### Progressive Level Design

**Level 1: Basic Brackets**
Focus on parentheses, square brackets, curly braces
```rust
fn generate_level_1(language: ProgrammingLanguage) -> String {
    let snippets = match language {
        TypeScript => vec![
            "()", "[]", "{}",
            "const x = (a + b);",
            "const arr = [1, 2, 3];",
            "const obj = { key: value };",
        ],
        Rust => vec![
            "()", "[]", "{}",
            "let x = (a + b);",
            "let vec = vec![1, 2, 3];",
            "struct Point { x: i32, y: i32 }",
        ],
        Python => vec![
            "()", "[]", "{}",
            "x = (a + b)",
            "arr = [1, 2, 3]",
            "dict = {\"key\": \"value\"}",
        ],
    };

    snippets.join(" ")
}
```

**Level 2: Common Operators**
Addition, subtraction, assignment, equality
```rust
fn generate_level_2(language: ProgrammingLanguage) -> String {
    // Basic operators in variable assignments and expressions
    match language {
        TypeScript => "let x = 5; x = x + 3; x += 2; x -= 1; const y = x * 2;",
        Rust => "let x = 5; let x = x + 3; let y = x * 2 - 1;",
        Python => "x = 5; x = x + 3; x += 2; x -= 1; y = x * 2",
    }
}
```

**Level 3: Comparison Operators**
```rust
fn generate_level_3(language: ProgrammingLanguage) -> String {
    match language {
        TypeScript => "if (x == 5) {} if (x != 0) {} if (x > 0 && x < 10) {}",
        Rust => "if x == 5 {} if x != 0 {} if x > 0 && x < 10 {}",
        Python => "if x == 5: pass; if x != 0: pass; if x > 0 and x < 10:",
    }
}
```

**Level 4: Arrows and Special**
```rust
fn generate_level_4(language: ProgrammingLanguage) -> String {
    match language {
        TypeScript => "const fn = (x) => x * 2; arr.map(x => x + 1);",
        Rust => "fn add(x: i32) -> i32 {} |x| -> x + 1; std::vec::Vec",
        Python => "lambda x: x * 2; def fn() -> int:",
    }
}
```

**Level 5: Compound Operators**
```rust
fn generate_level_5(language: ProgrammingLanguage) -> String {
    match language {
        TypeScript => "x++; y--; x += 5; y *= 2; x ||= default; y &&= value;",
        Rust => "x += 5; y -= 2; z *= 3; w /= 4;",
        Python => "x += 5; y -= 2; z *= 3; w //= 4;",
    }
}
```

**Level 6: Realistic Code**
Complete functions and complex patterns
```rust
fn generate_level_6(language: ProgrammingLanguage) -> String {
    match language {
        TypeScript => r#"
function sum(arr: number[]): number {
    return arr.reduce((acc, x) => acc + x, 0);
}
const result = arr.filter(x => x > 0).map(x => x * 2);
if (value !== null && value !== undefined) {
    console.log(`Value: ${value}`);
}
"#,
        Rust => r#"
fn sum(arr: &[i32]) -> i32 {
    arr.iter().fold(0, |acc, &x| acc + x)
}
let result: Vec<_> = arr.iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)
    .collect();
"#,
        Python => r#"
def sum(arr: list[int]) -> int:
    return sum(x for x in arr)

result = [x * 2 for x in arr if x > 0]
if value is not None and value != 0:
    print(f"Value: {value}")
"#,
    }.trim().to_string()
}
```

## Code Generator Implementation

```rust
pub struct CodeSymbolGenerator {
    language: ProgrammingLanguage,
    snippets: Vec<CodeSnippet>,
}

impl CodeSymbolGenerator {
    pub fn new(language: ProgrammingLanguage) -> Self {
        let snippets = match language {
            ProgrammingLanguage::TypeScript => TYPESCRIPT_SNIPPETS.to_vec(),
            ProgrammingLanguage::Rust => RUST_SNIPPETS.to_vec(),
            ProgrammingLanguage::Python => PYTHON_SNIPPETS.to_vec(),
            ProgrammingLanguage::Generic => GENERIC_SNIPPETS.to_vec(),
        };

        Self { language, snippets }
    }

    pub fn generate(&self, level: usize, length: usize) -> String {
        let filtered_snippets: Vec<_> = self.snippets
            .iter()
            .filter(|s| s.difficulty <= level as u8)
            .collect();

        let mut result = String::new();
        let mut idx = 0;

        while result.len() < length {
            if !result.is_empty() {
                result.push(' ');
            }

            let snippet = &filtered_snippets[idx % filtered_snippets.len()];
            result.push_str(&snippet.template);
            idx += 1;
        }

        result.chars().take(length).collect()
    }

    // Generate focused practice for specific symbol category
    pub fn generate_category(&self, category: SymbolCategory, length: usize) -> String {
        let filtered: Vec<_> = self.snippets
            .iter()
            .filter(|s| s.category == category)
            .collect();

        // Similar generation logic...
    }
}
```

## AZERTY Symbol Mapping

### Symbol Access on AZERTY

```rust
pub struct AzertySymbolMap {
    // Regular symbols (shift + number row)
    pub shift_symbols: HashMap<char, &'static str>,
    // AltGr symbols (harder to reach)
    pub altgr_symbols: HashMap<char, &'static str>,
}

impl AzertySymbolMap {
    pub fn new() -> Self {
        let shift_symbols = [
            ('1', "1 (shift)"),
            ('2', "2 (shift)"),
            ('+', "+ (shift)"),
            // ... etc
        ].iter().cloned().collect();

        let altgr_symbols = [
            ('{', "{ (AltGr + 4)"),
            ('}', "} (AltGr + =)"),
            ('[', "[ (AltGr + 5)"),
            (']', "] (AltGr + °)"),
            ('@', "@ (AltGr + 0)"),
            ('#', "# (AltGr + 3)"),
            // ... etc
        ].iter().cloned().collect();

        Self { shift_symbols, altgr_symbols }
    }

    pub fn difficulty(&self, symbol: char) -> u8 {
        if self.altgr_symbols.contains_key(&symbol) {
            3  // AltGr symbols are harder
        } else if self.shift_symbols.contains_key(&symbol) {
            2  // Shift symbols are moderate
        } else {
            1  // Regular keys are easy
        }
    }
}
```

## Menu Integration

```
Code Symbols
├── TypeScript
│   ├── Level 1 - Brackets
│   ├── Level 2 - Basic Operators
│   ├── Level 3 - Comparisons
│   ├── Level 4 - Arrows & Special
│   ├── Level 5 - Compound Operators
│   └── Level 6 - Realistic Code
├── Rust
│   ├── Level 1-6 (same structure)
├── Python
│   ├── Level 1-6 (same structure)
└── Generic
    └── All Symbols Practice
```

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_snippet_syntax_validity() {
        // Verify snippets are syntactically valid
        let gen = CodeSymbolGenerator::new(ProgrammingLanguage::TypeScript);

        for snippet in &gen.snippets {
            assert!(!snippet.template.is_empty());
            // Could use language parsers to validate syntax
        }
    }

    #[test]
    fn test_progressive_difficulty() {
        let gen = CodeSymbolGenerator::new(ProgrammingLanguage::Rust);

        // Level 1 should be easier than Level 6
        let level1 = gen.generate(1, 100);
        let level6 = gen.generate(6, 100);

        // Count special symbols
        let count_symbols = |s: &str| s.chars().filter(|c| !c.is_alphanumeric() && !c.is_whitespace()).count();

        assert!(count_symbols(&level1) < count_symbols(&level6));
    }

    #[test]
    fn test_language_specific_patterns() {
        let ts_gen = CodeSymbolGenerator::new(ProgrammingLanguage::TypeScript);
        let content = ts_gen.generate(4, 100);

        // TypeScript should have arrows
        assert!(content.contains("=>"));

        let py_gen = CodeSymbolGenerator::new(ProgrammingLanguage::Python);
        let content = py_gen.generate(4, 100);

        // Python should have colons
        assert!(content.contains(":"));
    }
}
```

## File Locations

- `src/content/code_symbols.rs` - Symbol definitions and templates
- `src/content/code_generator.rs` - Code generation logic
- `src/keyboard/symbol_map.rs` - AZERTY symbol mapping

## Performance Considerations

- **Template-based**: Pre-defined snippets, no runtime parsing
- **Memory efficient**: Store templates as static strings
- **Fast lookup**: HashMap for category/difficulty filtering
- **Deterministic**: Same level = same content for testing
