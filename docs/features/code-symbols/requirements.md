# Code Symbols - Requirements

> **Purpose**: Captures requirements for programming symbol practice
> **Module**: `src/content/` (extension)
> **Next Step**: See `design.md` for code generation strategy

## Code Symbol Practice Requirements

### R25: Code Symbol Training Mode
THE SYSTEM SHALL provide dedicated lessons for practicing programming symbols and operators

### R26: Symbol Category Organization
THE SYSTEM SHALL organize symbols into logical categories:
- Brackets: `()`, `[]`, `{}`, `<>`
- Operators: `+`, `-`, `*`, `/`, `%`, `=`, `!`, `&`, `|`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Arrows: `->`, `=>`, `<-`
- Special: `::`, `.`, `,`, `;`, `:`, `?`
- Compound: `&&`, `||`, `++`, `--`, `+=`, `-=`

### R27: Language-Specific Symbol Patterns
WHEN a user selects a programming language mode
THE SYSTEM SHALL generate realistic code patterns for that language

**TypeScript/JavaScript:**
```
const x: number = 42;
const arr = [1, 2, 3];
const obj = { key: "value" };
const fn = (a, b) => a + b;
if (x !== null && x > 0) {}
```

**Rust:**
```
fn main() -> Result<(), Error> {}
let x: i32 = 42;
let vec = vec![1, 2, 3];
match x { 0 => "zero", _ => "other" }
impl Trait for Type {}
```

**Python:**
```
def main():
    x = 42
    arr = [1, 2, 3]
    dict = {"key": "value"}
    if x != 0 and x > 0:
```

### R28: Progressive Symbol Difficulty
THE SYSTEM SHALL organize code symbol lessons in progressive difficulty:
- Level 1: Basic brackets `()` `[]` `{}`
- Level 2: Common operators `=` `+` `-` `*` `/`
- Level 3: Comparison operators `==` `!=` `<` `>`
- Level 4: Arrows and special `->` `=>` `::` `.`
- Level 5: Compound operators `&&` `||` `+=` `++`
- Level 6: Mixed realistic code snippets

### R29: Balanced Pair Practice
WHEN practicing bracket symbols
THE SYSTEM SHALL emphasize proper pairing and nesting
- `()` opening and closing
- `[]` array/index notation
- `{}` block/object notation
- Nested combinations: `({[]})`, `[{}]`, etc.

### R30: Realistic Code Context
WHEN generating code symbol practice
THE SYSTEM SHALL use realistic code patterns not random symbol sequences
- Valid function signatures
- Proper variable declarations
- Correct operator usage
- Syntactically valid snippets

### R31: Symbol Frequency by Language
THE SYSTEM SHALL weight symbol frequency based on language-specific usage patterns
- TypeScript: Heavy use of `:`, `=>`, `{}`
- Rust: Heavy use of `::`, `->`, `<>`, `|`
- Python: Heavy use of `:`, `[]`, `()`

## Integration Requirements

### R32: Code Mode Selection
THE SYSTEM SHALL add code symbol training options to the lesson selection menu
- Code Symbols - TypeScript
- Code Symbols - Rust
- Code Symbols - Python
- Code Symbols - Generic

### R33: Symbol Statistics Tracking
WHEN a user completes a code symbol lesson
THE SYSTEM SHALL track per-symbol accuracy and speed metrics
- Which symbols were typed correctly
- Most problematic symbol pairs
- Speed improvement per symbol type

## Keyboard Layout Considerations

### R34: AZERTY Symbol Access
THE SYSTEM SHALL account for AZERTY keyboard symbol placement
- Number row requires shift for numbers
- Special symbols on various shift combinations
- AltGr requirements for certain symbols (`@`, `#`, `{`, `}`, etc.)

### R35: Symbol Location Hints (Phase 3+)
FUTURE: Display hints for hard-to-reach symbols on AZERTY
FUTURE: Show finger-to-symbol mapping
FUTURE: Practice sessions focused on AltGr symbols

## Future Phase Requirements

### R36: Custom Code Snippets (Phase 3+)
FUTURE: Allow users to practice with their own code snippets
FUTURE: Import from GitHub repositories
FUTURE: Extract common patterns from user's codebase

### R37: Multi-Language Support (Phase 3+)
FUTURE: Add Go, Java, C++, C# language modes
FUTURE: Shell scripting mode (bash, zsh)
FUTURE: Configuration file mode (JSON, YAML, TOML)
