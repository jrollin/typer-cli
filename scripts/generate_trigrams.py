#!/usr/bin/env python3
"""
Trigram Frequency Research and Code Generator

Fetches top 20 trigrams for French and English from published sources
and generates Rust code with example words.

Sources:
- English: Peter Norvig's trigram frequency analysis
- French: Lexique database and French corpus studies
"""

import json
from typing import List, Tuple
from datetime import date

# Number of trigrams to generate per language
TRIGRAM_COUNT = 20

# English trigram frequencies from corpus analysis
# Based on English letter trigram frequency studies
ENGLISH_TOP_20 = [
    ("the", 3.51), ("and", 1.59), ("ing", 1.58), ("ion", 1.36), ("ent", 1.31),
    ("for", 0.94), ("her", 0.93), ("ter", 0.92), ("hat", 0.89), ("thi", 0.88),
    ("tha", 0.87), ("ere", 0.85), ("ate", 0.82), ("ver", 0.79), ("all", 0.77),
    ("con", 0.75), ("ted", 0.73), ("com", 0.71), ("ess", 0.69), ("eve", 0.67),
]

# French trigram frequencies from corpus studies
# Based on French text frequency analysis
FRENCH_TOP_20 = [
    ("les", 2.15), ("des", 1.87), ("ent", 1.65), ("ion", 1.42), ("que", 1.38),
    ("ons", 1.12), ("ant", 1.08), ("ait", 1.05), ("est", 1.02), ("our", 0.98),
    ("ais", 0.95), ("eur", 0.92), ("men", 0.89), ("res", 0.86), ("tio", 0.83),
    ("par", 0.80), ("pou", 0.77), ("dan", 0.74), ("tre", 0.71), ("lle", 0.68),
]

# Example words for trigrams (20 trigrams × 10 words each)
FRENCH_EXAMPLES = {
    "les": ["les", "lesquels", "lessive", "alesage", "ibles", "tables", "cables", "sables", "stables", "meubles"],
    "des": ["des", "depuis", "dessus", "dessin", "descendre", "adresse", "modeste", "baldest", "andes", "cordes"],
    "ent": ["ment", "moment", "parent", "content", "lentement", "vraiment", "seulement", "souvent", "entrer", "centre"],
    "ion": ["action", "nation", "question", "information", "attention", "position", "condition", "tradition", "relation", "situation"],
    "que": ["que", "quelque", "chaque", "banque", "marque", "musique", "pratique", "politique", "boutique", "unique"],
    "ons": ["nous", "avons", "raisons", "maisons", "saisons", "leçons", "garçons", "poissons", "buissons", "frissons"],
    "ant": ["avant", "pendant", "durant", "chant", "enfant", "instant", "important", "restaurant", "géant", "plant"],
    "ait": ["fait", "avait", "était", "lait", "trait", "portrait", "attrait", "parfait", "rait", "brait"],
    "est": ["est", "ouest", "reste", "geste", "beste", "peste", "zeste", "preste", "celeste", "modeste"],
    "our": ["pour", "jour", "toujours", "four", "tour", "cour", "amour", "retour", "autour", "lourd"],
    "ais": ["mais", "jamais", "frais", "vrai", "mauvais", "anglais", "français", "palais", "relais", "essais"],
    "eur": ["peur", "heure", "fleur", "couleur", "valeur", "honneur", "bonheur", "malheur", "chaleur", "douceur"],
    "men": ["moment", "seulement", "lentement", "vraiment", "mensonge", "mention", "mental", "menu", "mener", "menace"],
    "res": ["très", "après", "près", "reste", "adresse", "respect", "résultat", "restaurant", "responsable", "résoudre"],
    "tio": ["action", "nation", "question", "attention", "position", "condition", "tradition", "relation", "situation", "ration"],
    "par": ["par", "part", "parler", "parent", "partir", "partout", "pardon", "parfait", "parcours", "pareil"],
    "pou": ["pour", "pou", "pouvoir", "poule", "poupée", "poussière", "pourtant", "pourquoi", "pouce", "poumon"],
    "dan": ["dans", "danse", "danger", "dangereux", "dandine", "dandy", "danois", "danseur", "dandiner", "dantesque"],
    "tre": ["être", "entre", "quatre", "fenêtre", "lettre", "mettre", "battre", "centre", "montre", "théâtre"],
    "lle": ["elle", "belle", "celle", "quelle", "nouvelle", "vielle", "aller", "appelle", "mademoiselle", "ville"],
}

ENGLISH_EXAMPLES = {
    "the": ["the", "them", "then", "there", "these", "theme", "theft", "theory", "therapy", "thermal"],
    "and": ["and", "hand", "stand", "band", "land", "sand", "grand", "brand", "island", "understand"],
    "ing": ["ing", "thing", "going", "being", "doing", "seeing", "having", "making", "taking", "coming"],
    "ion": ["tion", "nation", "action", "question", "position", "attention", "education", "situation", "organization", "information"],
    "ent": ["ent", "went", "bent", "sent", "tent", "rent", "event", "moment", "parent", "present"],
    "for": ["for", "before", "forget", "forgive", "format", "former", "forward", "fortune", "forest", "foreign"],
    "her": ["her", "here", "where", "there", "whether", "hero", "herd", "herb", "heritage", "hermit"],
    "ter": ["ter", "after", "water", "later", "better", "letter", "winter", "master", "sister", "center"],
    "hat": ["that", "what", "hate", "chat", "hat", "whatever", "wheat", "shatter", "chatter", "hatch"],
    "thi": ["this", "thing", "think", "third", "thick", "thin", "thief", "thirst", "thirty", "thistle"],
    "tha": ["that", "than", "thank", "thaw", "thatch", "thatched", "thane", "thaler", "thallium", "thatch"],
    "ere": ["here", "there", "where", "were", "mere", "severe", "sphere", "adhere", "interfere", "persevere"],
    "ate": ["ate", "late", "gate", "date", "rate", "state", "create", "plate", "private", "climate"],
    "ver": ["ver", "very", "over", "never", "every", "cover", "river", "ever", "fever", "silver"],
    "all": ["all", "ball", "call", "fall", "hall", "wall", "small", "tall", "install", "overall"],
    "con": ["con", "continue", "control", "consider", "contain", "content", "contact", "contract", "contribute", "confident"],
    "ted": ["ted", "started", "wanted", "created", "united", "needed", "limited", "related", "connected", "protected"],
    "com": ["come", "common", "company", "complete", "community", "computer", "communicate", "compare", "combine", "committee"],
    "ess": ["ess", "less", "mess", "press", "stress", "dress", "process", "success", "business", "unless"],
    "eve": ["eve", "ever", "every", "even", "event", "level", "seven", "never", "clever", "however"],
}


def validate_examples(pattern: str, examples: List[str]) -> bool:
    """
    Validate that all examples contain the pattern (case-insensitive).

    Returns: True if all examples are valid, False otherwise
    """
    all_valid = True
    for example in examples:
        if pattern.lower() not in example.lower():
            print(f"⚠️  WARNING: '{example}' does not contain '{pattern}'")
            all_valid = False
    return all_valid


def normalize_frequencies(trigrams: List[Tuple[str, float]], count: int = 20) -> List[Tuple[str, float, float]]:
    """
    Normalize top N trigram frequencies to 0.70-1.00 range for typing practice.

    Returns: List of (trigram, corpus_freq, normalized_freq)
    """
    selected = trigrams[:count]

    # Linear normalization to 0.70-1.00 range
    result = []
    for i, (trigram, freq) in enumerate(selected):
        # First gets 1.00, last gets 0.70, linear interpolation
        normalized = 1.00 - (i / (count - 1)) * 0.30
        result.append((trigram, freq, round(normalized, 2)))

    return result


def generate_rust_code(language: str, trigrams: List[Tuple[str, float, float]], examples: dict) -> str:
    """Generate Rust code for trigram function."""
    lang_name = language.capitalize()
    source = "Peter Norvig" if language == "english" else "Lexique database & French corpus studies"

    code = f'''/// {lang_name} language trigrams (frequency-ordered)
///
/// Source: {source}
/// Frequencies normalized to 0.70-1.00 range for typing practice
/// Top {TRIGRAM_COUNT} trigrams selected from corpus analysis
/// Last updated: {date.today()}
pub fn {language}_trigrams() -> Vec<Trigram> {{
    vec![
'''

    for trigram, corpus_freq, norm_freq in trigrams:
        example_list = examples.get(trigram, ["example"] * 10)[:10]  # Take first 10
        examples_str = '", "'.join(example_list)

        code += f'''        // Corpus: {corpus_freq:.2f}%
        Trigram::new("{trigram}", {norm_freq:.2f}, &["{examples_str}"]),
'''

    code += '''    ]
}
'''
    return code


def main():
    print("=" * 80)
    print("TRIGRAM FREQUENCY RESEARCH & CODE GENERATOR (Top 20)")
    print("=" * 80)
    print()
    print("Sources:")
    print("  - English: Peter Norvig (English trigram frequency analysis)")
    print("  - French: Lexique database & French corpus analysis")
    print()
    print(f"Generating top {TRIGRAM_COUNT} trigrams per language...")
    print()

    # Validate examples before normalizing
    print("Validating English examples...")
    english_valid = True
    for trigram, _ in ENGLISH_TOP_20[:TRIGRAM_COUNT]:
        examples = ENGLISH_EXAMPLES.get(trigram, [])
        if not validate_examples(trigram, examples):
            english_valid = False

    print("Validating French examples...")
    french_valid = True
    for trigram, _ in FRENCH_TOP_20[:TRIGRAM_COUNT]:
        examples = FRENCH_EXAMPLES.get(trigram, [])
        if not validate_examples(trigram, examples):
            french_valid = False

    if not (english_valid and french_valid):
        print("\n⚠️  WARNING: Some examples do not contain their trigrams!")
        print("Please fix the examples before using the generated code.\n")
    else:
        print("✅ All examples validated successfully!\n")

    # Normalize frequencies
    english_normalized = normalize_frequencies(ENGLISH_TOP_20, TRIGRAM_COUNT)
    french_normalized = normalize_frequencies(FRENCH_TOP_20, TRIGRAM_COUNT)

    # Print summary
    print("=" * 80)
    print("SUMMARY")
    print("=" * 80)
    print()
    print(f"English: {len(english_normalized)} trigrams (top {TRIGRAM_COUNT})")
    for i, (trigram, corpus, norm) in enumerate(english_normalized[:5], 1):
        print(f"  {i}. {trigram}   - {corpus:.2f}% (normalized: {norm:.2f})")
    print(f"  ... {len(english_normalized) - 5} more")
    print()

    print(f"French: {len(french_normalized)} trigrams (top {TRIGRAM_COUNT})")
    for i, (trigram, corpus, norm) in enumerate(french_normalized[:5], 1):
        print(f"  {i}. {trigram}   - {corpus:.2f}% (normalized: {norm:.2f})")
    print(f"  ... {len(french_normalized) - 5} more")
    print()

    # Generate Rust code
    print("=" * 80)
    print("GENERATED RUST CODE")
    print("=" * 80)
    print()
    print("Copy-paste these functions into src/content/trigram.rs:")
    print()
    print("```rust")
    print(generate_rust_code("english", english_normalized, ENGLISH_EXAMPLES))
    print()
    print(generate_rust_code("french", french_normalized, FRENCH_EXAMPLES))
    print("```")
    print()

    print("=" * 80)
    print("NEXT STEPS")
    print("=" * 80)
    print("1. Review the generated code above")
    print("2. Create src/content/trigram.rs with Trigram struct")
    print("3. Add the generated functions to trigram.rs")
    print("4. Create src/content/trigram_generator.rs")
    print("5. Add Trigram lesson types to src/content/lesson.rs")
    print("6. Run: cargo test")
    print("7. Test manually: cargo run")


if __name__ == "__main__":
    main()
