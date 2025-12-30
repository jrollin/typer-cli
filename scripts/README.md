# Content Generation Scripts

This directory contains scripts for generating Rust code for bigrams, trigrams, and common words used in typer-cli's pattern training.

## Overview

Bigrams are 2-letter combinations (like "th", "qu", "es") that appear frequently in natural language. Training with common bigrams improves typing fluency and muscle memory for real-world text.

## Data Sources

### English Bigrams
- **Source**: Peter Norvig's Google Web Trillion Word Corpus analysis
- **URL**: http://norvig.com/mayzner.html
- **Coverage**: Top 100 bigrams analyzed from web corpus
- **Reliability**: Industry standard, widely cited

### French Bigrams
- **Source**: Lexique database & French corpus studies
- **URL**: http://www.lexique.org
- **Coverage**: 31 million word corpus of contemporary French texts
- **Reliability**: Academic research, validated frequency data

## Current Configuration

- **Bigrams per language**: 30 (expanded from original 10)
- **Example words per bigram**: 10 (expanded from original 4)
- **Total data**: 30 bigrams × 10 examples × 2 languages = **600 example words**
- **Normalization range**: 0.70-1.00 (for future spaced repetition)

## File Structure

```
scripts/
├── README.md                    # This file
├── generate_bigrams.py          # Bigram code generation
├── generate_trigrams.py         # Trigram code generation
└── generate_common_words.py     # Common words code generation
```

## Usage

### Generate Updated Bigram Code

Run the script to generate Rust code with corpus-backed bigram data:

```bash
cd /home/julien/projects/typer-cli
python3 scripts/generate_bigrams.py
```

### Output

The script generates:
1. **Summary**: Top 30 bigrams for each language with frequencies
2. **Rust Code**: Ready-to-paste functions for `src/content/bigram.rs`
3. **Metadata**: Source attribution, last updated date, corpus frequencies

### Example Output Structure

```rust
/// English language bigrams (frequency-ordered)
///
/// Source: Peter Norvig (http://norvig.com/mayzner.html)
/// Frequencies normalized to 0.70-1.00 range for typing practice
/// Top 30 bigrams selected from top 100 corpus analysis
/// Last updated: 2025-12-30
pub fn english_bigrams() -> Vec<Bigram> {
    vec![
        // Corpus: 3.56%
        Bigram::new("th", 1.00, &["the", "that", "with", "this", "think", "other", "three", "month", "through", "thought"]),
        // ... 29 more
    ]
}
```

## Integration Steps

After running the script:

1. **Review Output**: Check that bigrams and examples make sense
2. **Copy Functions**: Replace `french_bigrams()` and `english_bigrams()` in `src/content/bigram.rs`
3. **Update Generator**: Modify `src/content/bigram_generator.rs` level selection:
   ```rust
   fn select_bigrams_for_level(&self, level: usize) -> Vec<&Bigram> {
       let count = match level {
           1 => 5,   // Top 5 (unchanged)
           2 => 10,  // Top 10 (was 7)
           3 => 15,  // Top 15 (was 10)
           4 => 30,  // NEW: All 30
           _ => 5,
       };
       self.bigrams.iter().take(count).collect()
   }
   ```
4. **Add Lessons**: Add Level 4 to `src/content/lesson.rs` lesson generation
5. **Test**: Run `cargo test` and manual testing

## Data Validation

### French Accentuation Policy

All French examples **MUST** use proper accent marks to ensure authentic typing practice:

**Required accent marks:**
- `é, è, ê` (acute, grave, circumflex e)
- `à, â` (grave, circumflex a)
- `ç` (cedilla)
- `ô, ù, î, ï, œ` (other diacritics)

**Rationale:**
1. **Authentic French typing practice** - Users learn to type real French words
2. **Consistency with bigram examples** - All scripts use same standards
3. **Better language learning** - Reinforces proper spelling
4. **Aligns with real-world usage** - Matches actual French texts

**Validation:**
- Scripts include automatic validation via `validate_examples()` function
- Run tests after integration: `cargo test` validates examples contain patterns
- Manual review recommended for accented patterns to ensure proper matching

**Note:** Some validation warnings for accented patterns (e.g., "très" contains "es" but validator may not match due to accent normalization) are expected and should be manually verified.

---

### Frequency Normalization

Frequencies are normalized using linear interpolation:

```python
normalized = 1.00 - (position / (count - 1)) * 0.30
# Result: 1.00 (rank 1) → 0.70 (rank 30)
```

This creates:
- Meaningful spacing for future spaced repetition algorithms
- Consistent 0.70-1.00 range across all pattern types
- Descending order (required by tests)

### Example Word Selection Criteria

Example words must:
- Contain the target bigram/trigram (validated by test)
- Be common in contemporary usage
- Vary in word length (short → long)
- Include different word types (nouns, verbs, adjectives)
- Be appropriate for typing practice (no profanity)
- **For French:** Include proper accent marks (see policy above)

## Frequency Data

### English Top 10

| Rank | Bigram | Frequency | Examples |
|------|--------|-----------|----------|
| 1 | th | 3.56% | the, that, with, this, think, other, three, month, through, thought |
| 2 | he | 3.07% | he, the, when, where, she, then, them, here, there, these |
| 3 | in | 2.43% | in, thing, nothing, into, think, since, find, during, living, being |
| 4 | er | 2.05% | her, over, after, never, under, ever, there, other, where, mother |
| 5 | an | 1.99% | an, and, can, than, many, any, want, hand, plan, change |
| 6 | re | 1.85% | are, were, here, there, where, before, really, great, three, free |
| 7 | on | 1.76% | on, one, upon, only, long, among, second, person, money, reason |
| 8 | at | 1.49% | at, that, what, late, water, great, state, date, create, matter |
| 9 | en | 1.45% | been, when, then, open, even, often, seven, taken, happen, listen |
| 10 | nd | 1.35% | and, end, send, kind, find, hand, second, understand, behind, friend |

### French Top 10

| Rank | Bigram | Frequency | Examples |
|------|--------|-----------|----------|
| 1 | es | 3.15% | les, des, mes, ses, ces, tes, très, esprit, reste, gestes |
| 2 | le | 2.76% | le, les, lequel, lent, lecteur, léger, lever, lettre, parler, aile |
| 3 | de | 2.55% | de, des, depuis, devant, dedans, dehors, demain, devenir, dessin, idée |
| 4 | en | 2.47% | en, ment, bien, rien, encore, enfant, pendant, moment, content, seulement |
| 5 | re | 2.30% | re, entre, faire, reste, être, prendre, regarder, rendre, représenter, rencontre |
| 6 | nt | 2.18% | ment, lent, content, sont, maintenant, avant, enfant, moment, pendant, souvent |
| 7 | on | 2.15% | on, bon, son, non, dont, long, selon, maison, raison, garçon |
| 8 | er | 2.13% | premier, dernier, aller, mer, cher, hier, hiver, verre, terre, guerre |
| 9 | te | 2.02% | te, vite, petite, juste, texte, tête, temps, cette, mettre, lettre |
| 10 | el | 1.91% | el, tel, bel, cruel, elle, belle, celle, quelle, nouvelle, naturel |

## Future Enhancements

### Expand to More Bigrams
- Current: 30 per language
- Potential: 50-100 for advanced levels
- Requires: Updating `BIGRAM_COUNT` constant and level selection logic

### Add More Languages
- Spanish: "es", "de", "en", "la", "os"
- German: "en", "er", "ch", "ei", "ie"
- Italian: "la", "re", "le", "di", "to"

### Trigrams (3-letter patterns)
- English: "the", "and", "ing", "ion"
- French: "les", "des", "ent", "que"
- See Phase 2 planning documents

### Common Words (500 most frequent)
- Natural typing practice
- Progressive difficulty levels
- See Phase 3 planning documents

## Maintenance

### When to Update

Update bigram data when:
- Corpus research publishes new frequency data
- Adding support for new languages
- Expanding to more bigrams (e.g., 30 → 50)
- Improving example word quality

### Update Process

1. Update frequency data in `ENGLISH_TOP_100` or `FRENCH_TOP_100`
2. Add/update example words in `ENGLISH_EXAMPLES` or `FRENCH_EXAMPLES`
3. Adjust `BIGRAM_COUNT` if needed
4. Run script: `python3 scripts/generate_bigrams.py`
5. Review and integrate generated code
6. Update tests if needed
7. Document changes in git commit

## Version History

### 2025-12-30: Major Expansion
- **Bigrams**: Expanded from 10 to 30 per language
- **Examples**: Expanded from 4 to 10 per bigram
- **Total data**: 600 example words (up from 80)
- **Levels**: Added Level 4 (30 bigrams) to progression
- **Changes**:
  - Removed comparison with previous selections (cleaner output)
  - All frequencies backed by corpus data
  - French and English fully validated
  - Code bigrams unchanged (deferred to language-specific tracks)

### Original (2024-12)
- 10 bigrams per language
- 4 example words per bigram
- 3 levels (5, 7, 10 bigrams)
- Manual frequency estimates

## Testing

After integration, validate with:

```bash
# All bigram tests
cargo test bigram

# Specific validations
cargo test test_french_bigrams_frequency_order
cargo test test_english_bigrams_frequency_order
cargo test test_examples_contain_bigrams

# Manual testing
cargo run
# Select "French Bigrams - Level 4" or "English Bigrams - Level 4"
```

## References

### Research Papers
- Norvig, P. (2012). "English Letter Frequency Counts: Mayzner Revisited"
- New, B., et al. (2004). "Lexique 2: A New French Lexical Database"

### Online Resources
- [Peter Norvig's N-gram Analysis](http://norvig.com/mayzner.html)
- [Lexique Database](http://www.lexique.org)
- [Google Books Ngram Viewer](https://books.google.com/ngrams)
- [COCA Corpus](https://www.english-corpora.org/coca/)

### Related Documentation
- `docs/features/bigram-training/` - Feature requirements and design
- `CLAUDE.md` - Project phase tracking and roadmap
- `/home/julien/.claude/plans/hidden-leaping-wombat.md` - Expansion planning document

## License

Data sources are used under their respective licenses:
- Peter Norvig's corpus data: Public domain
- Lexique database: Free for research and educational use
- This script and documentation: Same as typer-cli project
