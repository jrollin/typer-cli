# Code Symbols - Task Tracking

> **Purpose**: Implementation progress for code symbol practice feature
> **Module**: `src/content/`
> **Status**: ⏳ PLANNED (Phase 2+)

## Phase 2+: Implementation ✓ COMPLETED

### Data Structures (src/content/code_symbols.rs) ✓
- [x] Define `ProgrammingLanguage` enum (TypeScript, Rust, Python)
- [x] Define `SymbolCategory` enum (Brackets, BasicOps, Comparison, Arrows, Compound, Special)
- [x] Define `CodeSnippet` struct with category, template, difficulty
- [x] Create TypeScript snippet library (18 snippets across 6 levels)
- [x] Create Rust snippet library (18 snippets across 6 levels)
- [x] Create Python snippet library (18 snippets across 6 levels)

### Content Generation (src/content/code_generator.rs) ✓
- [x] Implement `CodeSymbolGenerator` struct
- [x] Progressive level filtering (difficulty 1-6)
- [x] Level 1: Basic brackets snippets
- [x] Level 2: Common operators snippets
- [x] Level 3: Comparison operators snippets
- [x] Level 4: Arrows and special symbols snippets
- [x] Level 5: Compound operators snippets
- [x] Level 6: Realistic code snippets
- [x] Deterministic generation for testing

### Language-Specific Logic ✓
- [x] TypeScript: Arrow functions, type annotations, const/let
- [x] Rust: Functions with ->, match arms, lifetimes, impl blocks
- [x] Python: List/dict comprehensions, f-strings, decorators, def
- [x] All snippets syntactically valid for their language

### Lesson Integration (src/content/lesson.rs) ✓
- [x] Extend `LessonType` enum with `CodeSymbols` variant
- [x] Implement `Lesson::code_symbol_lessons()` factory
- [x] 6 progressive levels per language (3 languages x 6 levels = 18 lessons)
- [x] Integrate with ContentGenerator trait

### UI Integration (src/app.rs) ✓
- [x] Add all code symbol lessons to app lesson list
- [x] 33 total lessons now (6 home + 9 bigrams + 18 code)
- [x] Automatic integration with existing menu
- [x] Session saving works with code lesson titles

### Testing ✓
- [x] Test TypeScript generation (7 tests)
- [x] Test Rust generation (7 tests)
- [x] Test Python generation (7 tests)
- [x] Test progressive difficulty
- [x] Test language-specific patterns
- [x] Test deterministic generation
- [x] Test all levels generate content
- [x] 12 new tests added (total: 56 tests, all passing)

### Documentation
- [x] Update tasks.md with implementation status
- [ ] Update README with code symbols feature
- [ ] Add examples to CLAUDE.md
- [ ] Document AZERTY symbol considerations

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
- [x] Visual keyboard layout overlay (Phase 3 ✓)
- [x] Highlight symbol location on AZERTY (Phase 3 ✓)
- [x] Show finger positioning for AltGr symbols (Phase 3.6 ✓)
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

### AZERTY Considerations (Phase 3.6 ✓)
**AltGr symbols now fully supported with keyboard highlighting:**
- `{` `}` - AltGr + ', AltGr + = ✓
- `[` `]` - AltGr + (, AltGr + ) ✓
- `@` - AltGr + à ✓
- `#` - AltGr + " ✓
- `|` - AltGr + - ✓
- `` ` `` - AltGr + è ✓
- `\` - AltGr + _ ✓
- `^` - AltGr + ç ✓
- `~` - AltGr + é ✓

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
