# Code Symbols - Task Tracking

> **Purpose**: Implementation progress for code symbol practice feature
> **Module**: `src/content/`
> **Status**: ⏳ PLANNED (Phase 2+)

## Phase 2+: Implementation

### Data Structures (src/content/code_symbols.rs)
- [ ] Define `ProgrammingLanguage` enum (TypeScript, Rust, Python, Generic)
- [ ] Define `SymbolCategory` enum (Brackets, BasicOps, Comparison, Arrows, Compound, Special)
- [ ] Define `CodeSnippet` struct with language, category, template, difficulty
- [ ] Create TypeScript snippet library (20+ snippets across 6 levels)
- [ ] Create Rust snippet library (20+ snippets across 6 levels)
- [ ] Create Python snippet library (20+ snippets across 6 levels)
- [ ] Create Generic snippet library (language-agnostic symbols)

### AZERTY Symbol Mapping (src/keyboard/symbol_map.rs)
- [ ] Define `AzertySymbolMap` struct
- [ ] Map shift symbols (shift + number row)
- [ ] Map AltGr symbols (AltGr combinations)
- [ ] Implement symbol difficulty rating (1-3 based on key combo)
- [ ] Create visual reference guide for AZERTY symbols

### Content Generation (src/content/code_generator.rs)
- [ ] Implement `CodeSymbolGenerator` struct
- [ ] Level 1: Basic brackets generation
- [ ] Level 2: Common operators generation
- [ ] Level 3: Comparison operators generation
- [ ] Level 4: Arrows and special symbols generation
- [ ] Level 5: Compound operators generation
- [ ] Level 6: Realistic code snippets generation
- [ ] Implement category-based filtering
- [ ] Implement difficulty-based progression

### Language-Specific Logic
- [ ] TypeScript: Arrow functions, type annotations, spread operator
- [ ] Rust: Lifetimes, turbofish, match arms, closures
- [ ] Python: List comprehensions, f-strings, decorators
- [ ] Ensure syntactically valid snippets for each language

### Lesson Integration (src/content/lesson.rs)
- [ ] Extend `LessonType` enum with `CodeSymbols` variant
- [ ] Implement `Lesson::code_lessons()` for each language
- [ ] Add 6 progressive levels per language (4 languages x 6 levels = 24 lessons)
- [ ] Add code lessons to menu structure

### UI Integration (src/app.rs, src/ui/render.rs)
- [ ] Add code symbols category to lesson menu
- [ ] Submenu for language selection
- [ ] Display syntax highlighting hints (optional, Phase 3)
- [ ] Show AZERTY symbol hints for difficult symbols
- [ ] Update session saving with code lesson type

### Testing
- [ ] Test snippet template validity
- [ ] Test progressive difficulty (Level 1 < Level 6 complexity)
- [ ] Test language-specific patterns
- [ ] Test symbol frequency distribution
- [ ] Test AZERTY symbol mapping accuracy
- [ ] Integration test for all 24 lessons
- [ ] Manual testing with real developers

### Documentation
- [ ] Update README with code symbols feature
- [ ] Add AZERTY symbol quick reference guide
- [ ] Document language-specific patterns
- [ ] Add examples to CLAUDE.md

## Phase 3: Advanced Features

### Syntax Validation
- [ ] Integrate language parsers (tree-sitter)
- [ ] Validate generated snippets compile/parse
- [ ] Auto-correct malformed snippets

### Custom Code Import
- [ ] Import snippets from GitHub repos
- [ ] Extract patterns from user's codebase
- [ ] Generate practice from real project files
- [ ] Privacy-preserving local processing

### Additional Languages
- [ ] Go support
- [ ] Java support
- [ ] C++ support
- [ ] C# support
- [ ] Shell scripting (bash, zsh)
- [ ] Config files (JSON, YAML, TOML)

### Symbol Hints
- [ ] Visual keyboard layout overlay
- [ ] Highlight symbol location on AZERTY
- [ ] Show finger positioning for AltGr symbols
- [ ] Practice mode for difficult symbol combinations

## Implementation Notes

### Priority Order
1. Start with Generic symbols (language-agnostic)
2. Add TypeScript (web development popularity)
3. Add Rust (aligned with project language)
4. Add Python (broad appeal)

### Estimated Complexity
- **Data structures**: Low (template-based, no parsing)
- **Symbol mapping**: Medium (AZERTY complexity)
- **Generation logic**: Medium (ensure valid syntax)
- **UI integration**: Low (extend existing menu)
- **Overall**: Medium-High complexity, 4-5 days of work

### Dependencies
- No external dependencies for Phase 2+
- Optional: tree-sitter for syntax validation (Phase 3)
- Optional: syntax highlighting library (Phase 3)

### Testing Strategy
- Unit tests for each language generator
- Validate snippet syntax manually
- Test symbol difficulty ratings
- Integration tests for menu flow
- User testing with developers from each language community

### AZERTY Considerations
**Critical symbols requiring AltGr:**
- `{` `}` - AltGr + 4, AltGr + =
- `[` `]` - AltGr + 5, AltGr + °
- `@` - AltGr + 0
- `#` - AltGr + 3
- `|` - AltGr + 6
- `` ` `` - AltGr + 7
- `\` - AltGr + 8

**High-difficulty combinations:**
- `->`  : - then >
- `::` : Shift + . twice
- `=>` : = then Shift + <
- `[]` : AltGr + 5, AltGr + °

These should be introduced progressively in later levels.

### Language-Specific Focus

**TypeScript priorities:**
- Arrow functions `=>`
- Type annotations `:`
- Generics `<T>`
- Optional chaining `?.`
- Nullish coalescing `??`
- Template literals `` ` `` `${ }`

**Rust priorities:**
- Lifetimes `'a`
- Turbofish `::<T>`
- Match arms `=>`
- Closures `|x|`
- References `&`, `&mut`
- Path separator `::`

**Python priorities:**
- List comprehensions `[x for x in arr]`
- Dict comprehensions `{k: v for ...}`
- f-strings `f"{var}"`
- Decorators `@decorator`
- Type hints `: int`
- Walrus operator `:=`
