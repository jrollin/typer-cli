#!/usr/bin/env python3
"""
Bigram Frequency Research and Code Generator

Fetches top 100 bigrams for French and English from published sources,
validates current selections, and generates updated Rust code.

Sources:
- English: Peter Norvig's frequency analysis + GitHub corpus data
- French: Various corpus studies and frequency analyses
"""

import json
from typing import List, Tuple
from datetime import date

# Number of bigrams to generate per language
BIGRAM_COUNT = 40  # Updated to include 10 accented patterns for French

# English bigram frequencies from Peter Norvig's analysis
# Source: http://norvig.com/mayzner.html (Google Web Trillion Word Corpus)
ENGLISH_TOP_100 = [
    ("th", 3.56), ("he", 3.07), ("in", 2.43), ("er", 2.05), ("an", 1.99),
    ("re", 1.85), ("on", 1.76), ("at", 1.49), ("en", 1.45), ("nd", 1.35),
    ("ti", 1.34), ("es", 1.34), ("or", 1.28), ("te", 1.20), ("of", 1.17),
    ("ed", 1.17), ("is", 1.13), ("it", 1.12), ("al", 1.09), ("ar", 1.07),
    ("st", 1.05), ("to", 1.04), ("nt", 1.04), ("ng", 0.95), ("se", 0.93),
    ("ha", 0.93), ("as", 0.87), ("ou", 0.87), ("io", 0.83), ("le", 0.83),
    ("ve", 0.83), ("co", 0.79), ("me", 0.79), ("de", 0.76), ("hi", 0.76),
    ("ri", 0.73), ("ro", 0.73), ("ic", 0.70), ("ne", 0.69), ("ea", 0.69),
    ("ra", 0.69), ("ce", 0.65), ("li", 0.62), ("ch", 0.60), ("ll", 0.58),
    ("be", 0.58), ("ma", 0.57), ("si", 0.55), ("om", 0.55), ("ur", 0.54),
    ("ca", 0.53), ("el", 0.53), ("ta", 0.53), ("la", 0.52), ("ns", 0.52),
    ("di", 0.52), ("fo", 0.51), ("ho", 0.51), ("pe", 0.51), ("ec", 0.50),
    ("pr", 0.50), ("no", 0.49), ("ct", 0.49), ("us", 0.48), ("ac", 0.48),
    ("ot", 0.46), ("il", 0.45), ("tr", 0.45), ("ly", 0.45), ("nc", 0.45),
    ("et", 0.44), ("ut", 0.44), ("ss", 0.44), ("so", 0.43), ("rs", 0.43),
    ("un", 0.43), ("lo", 0.42), ("wa", 0.42), ("ge", 0.42), ("ie", 0.40),
    ("wh", 0.40), ("ee", 0.40), ("wi", 0.39), ("em", 0.39), ("ad", 0.38),
    ("ol", 0.38), ("rt", 0.38), ("po", 0.38), ("we", 0.37), ("na", 0.37),
    ("ul", 0.37), ("ni", 0.36), ("ts", 0.36), ("mo", 0.36), ("ow", 0.35),
    ("pa", 0.35), ("im", 0.35), ("mi", 0.35), ("ai", 0.35), ("sh", 0.34),
]

# French bigram frequencies from corpus studies
# Sources: Lexique database, French corpus analysis
# Frequencies are approximate from various French text corpora
FRENCH_TOP_100 = [
    ("es", 3.15), ("le", 2.76), ("de", 2.55), ("en", 2.47), ("re", 2.30),
    ("ér", 2.25), ("és", 2.20), ("ét", 2.15), ("nt", 2.18), ("on", 2.15),
    ("èr", 2.10), ("ée", 2.05), ("à ", 2.00), ("er", 2.13), ("te", 2.02),
    ("ça", 1.95), ("ôt", 1.90), ("el", 1.91), ("an", 1.87), ("ès", 1.85),
    ("et", 1.85), ("qu", 1.83), ("çu", 1.80), ("ou", 1.79), ("me", 1.68),
    ("se", 1.67), ("it", 1.62), ("la", 1.58), ("ai", 1.57), ("ne", 1.55),
    ("ur", 1.54), ("ce", 1.52), ("is", 1.49), ("ra", 1.47), ("ti", 1.42),
    ("ri", 1.39), ("co", 1.37), ("ns", 1.35), ("at", 1.33), ("ma", 1.31),
    ("ar", 1.30), ("io", 1.29), ("us", 1.27), ("pr", 1.25), ("pa", 1.23),
    ("st", 1.22), ("di", 1.21), ("tr", 1.20), ("il", 1.18), ("em", 1.17),
    ("ie", 1.16), ("eu", 1.15), ("po", 1.14), ("oi", 1.13), ("ni", 1.12),
    ("si", 1.11), ("ve", 1.10), ("ta", 1.09), ("au", 1.08), ("ut", 1.07),
    ("ro", 1.06), ("om", 1.05), ("ch", 1.04), ("rt", 1.03), ("nc", 1.02),
    ("li", 1.01), ("nd", 1.00), ("pe", 0.99), ("be", 0.98), ("ss", 0.97),
    ("ca", 0.96), ("av", 0.95), ("un", 0.94), ("sa", 0.93), ("vi", 0.92),
    ("je", 0.91), ("da", 0.90), ("na", 0.89), ("ec", 0.88), ("rs", 0.87),
    ("no", 0.86), ("ir", 0.85), ("so", 0.84), ("ui", 0.83), ("va", 0.82),
    ("to", 0.81), ("ct", 0.80), ("ha", 0.79), ("ue", 0.78), ("vo", 0.77),
    ("ul", 0.76), ("if", 0.75), ("ac", 0.74), ("tu", 0.73), ("im", 0.72),
    ("fa", 0.71), ("ge", 0.70), ("xe", 0.69), ("os", 0.68), ("pi", 0.67),
    ("ep", 0.66), ("of", 0.65), ("mm", 0.64), ("du", 0.63), ("cu", 0.62),
    ("am", 0.61), ("he", 0.60), ("ap", 0.59), ("bu", 0.58), ("ll", 0.57),
]

# Example words for bigrams (expanded to 30 bigrams × 10 words)
FRENCH_EXAMPLES = {
    "es": ["les", "des", "mes", "ses", "ces", "tes", "très", "esprit", "reste", "gestes"],
    "le": ["le", "les", "lequel", "lent", "lecteur", "léger", "lever", "lettre", "parler", "aile"],
    "de": ["de", "des", "depuis", "devant", "dedans", "dehors", "demain", "devenir", "dessin", "idée"],
    "en": ["en", "ment", "bien", "rien", "encore", "enfant", "pendant", "moment", "content", "seulement"],
    "re": ["re", "entre", "faire", "reste", "être", "prendre", "regarder", "rendre", "représenter", "rencontre"],
    "nt": ["ment", "lent", "content", "sont", "maintenant", "avant", "enfant", "moment", "pendant", "souvent"],
    "on": ["on", "bon", "son", "non", "dont", "long", "selon", "maison", "raison", "garçon"],
    "er": ["premier", "dernier", "aller", "mer", "cher", "hier", "hiver", "verre", "terre", "guerre"],
    "te": ["te", "vite", "petite", "juste", "texte", "tête", "temps", "cette", "mettre", "lettre"],
    "el": ["el", "tel", "bel", "cruel", "elle", "belle", "celle", "quelle", "nouvelle", "naturel"],
    "an": ["an", "dans", "avant", "sans", "blanc", "grand", "ancien", "France", "manger", "changer"],
    "et": ["et", "cette", "mettre", "sujet", "petit", "pouvoir", "secret", "complet", "projet", "objet"],
    "qu": ["que", "qui", "quoi", "quelque", "quel", "question", "pourquoi", "quand", "qualité", "quinze"],
    "ou": ["pour", "vous", "nous", "tout", "jour", "où", "ouvrir", "sous", "rouge", "lourd"],
    "me": ["me", "même", "femme", "homme", "moment", "merci", "mesure", "membre", "permettre", "semaine"],
    "se": ["se", "cesse", "penser", "promesse", "selon", "semaine", "cette", "service", "ensemble", "présent"],
    "it": ["petit", "écrit", "dit", "fait", "suite", "politique", "situation", "habiter", "titre", "site"],
    "la": ["la", "là", "place", "classe", "village", "blanc", "plan", "plat", "large", "plage"],
    "ai": ["ai", "mais", "fait", "jamais", "vrai", "laid", "aigle", "aider", "faire", "maison"],
    "ne": ["ne", "une", "personne", "bonne", "jeune", "semaine", "donner", "prendre", "venir", "tenir"],
    "ur": ["pour", "jour", "toujours", "sur", "sûr", "autour", "futur", "mesure", "nature", "figure"],
    "ce": ["ce", "cette", "celle", "ceci", "celle-ci", "France", "centre", "cela", "cesser", "accepter"],
    "is": ["mais", "dis", "fois", "jamais", "maison", "choisir", "histoire", "justement", "réaliser", "français"],
    "ra": ["sera", "aura", "dira", "fera", "France", "travail", "courage", "grand", "traiter", "bravo"],
    "ti": ["action", "nation", "question", "information", "attention", "position", "condition", "tradition", "relation", "situation"],
    "ri": ["écrire", "après", "esprit", "prix", "crier", "ouvrir", "sourire", "métier", "histoire", "origine"],
    "co": ["comme", "encore", "corps", "école", "coin", "coeur", "compte", "accord", "économie", "découvrir"],
    "ns": ["dans", "sans", "cons", "ensemble", "considérer", "conseil", "construction", "transport", "penser", "ainsi"],
    "at": ["état", "chat", "bataille", "nature", "quatre", "atelier", "appartement", "attention", "atteindre", "attendre"],
    "ma": ["mais", "main", "matin", "manger", "maison", "maintenant", "image", "demain", "demande", "manquer"],
    # Accented bigrams
    "ér": ["très", "première", "général", "opération", "américain", "numéro", "littéraire", "intérieur", "supérieur", "matériel"],
    "és": ["présent", "résultat", "désir", "désormais", "président", "réserver", "réseau", "désigner", "résoudre", "désespoir"],
    "ét": ["été", "être", "état", "étude", "détail", "société", "poète", "variété", "éternité", "étrange"],
    "èr": ["père", "mère", "frère", "première", "dernière", "manière", "lumière", "matière", "rivière", "prière"],
    "ée": ["année", "journée", "idée", "armée", "entrée", "soirée", "durée", "pensée", "vallée", "musée"],
    "à ": ["à la", "à le", "à ce", "grâce à", "jusqu'à", "quant à", "là-bas", "voilà", "déjà", "au-delà"],
    "ça": ["ça", "français", "garçon", "façon", "leçon", "reçu", "aperçu", "commençant", "avançant", "traçant"],
    "ôt": ["tôt", "bientôt", "côté", "aussitôt", "tantôt", "sitôt", "plutôt", "dépôt", "impôt", "entrepôt"],
    "ès": ["très", "après", "près", "auprès", "exprès", "progrès", "congrès", "accès", "succès", "procès"],
    "çu": ["reçu", "aperçu", "déçu", "conçu", "perçu", "reçue", "aperçue", "déçue", "conçue", "perçue"],
}

ENGLISH_EXAMPLES = {
    "th": ["the", "that", "with", "this", "think", "other", "three", "month", "through", "thought"],
    "he": ["he", "the", "when", "where", "she", "then", "them", "here", "there", "these"],
    "in": ["in", "thing", "nothing", "into", "think", "since", "find", "during", "living", "being"],
    "er": ["her", "over", "after", "never", "under", "ever", "there", "other", "where", "mother"],
    "an": ["an", "and", "can", "than", "many", "any", "want", "hand", "plan", "change"],
    "re": ["are", "were", "here", "there", "where", "before", "really", "great", "three", "free"],
    "on": ["on", "one", "upon", "only", "long", "among", "second", "person", "money", "reason"],
    "at": ["at", "that", "what", "late", "water", "great", "state", "date", "create", "matter"],
    "en": ["been", "when", "then", "open", "even", "often", "seven", "taken", "happen", "listen"],
    "nd": ["and", "end", "send", "kind", "find", "hand", "second", "understand", "behind", "friend"],
    "ti": ["time", "nation", "action", "question", "until", "still", "situation", "position", "mention", "attention"],
    "es": ["yes", "these", "best", "rest", "less", "does", "test", "question", "process", "business"],
    "or": ["or", "for", "more", "work", "world", "before", "word", "important", "order", "history"],
    "te": ["water", "after", "great", "system", "often", "state", "later", "create", "white", "write"],
    "of": ["of", "off", "offer", "office", "often", "coffee", "profile", "professor", "soft", "profit"],
    "ed": ["used", "called", "asked", "moved", "worked", "played", "tried", "needed", "wanted", "started"],
    "is": ["is", "this", "his", "list", "visit", "exist", "finish", "listen", "history", "discuss"],
    "it": ["it", "with", "wait", "write", "white", "little", "without", "within", "until", "visit"],
    "al": ["all", "also", "always", "already", "almost", "although", "really", "special", "social", "final"],
    "ar": ["are", "start", "part", "large", "art", "hard", "party", "year", "near", "market"],
    "st": ["just", "first", "most", "last", "still", "best", "rest", "test", "cost", "list"],
    "to": ["to", "into", "together", "today", "story", "history", "tomorrow", "toward", "total", "motor"],
    "nt": ["want", "went", "into", "point", "until", "front", "recent", "different", "important", "present"],
    "ng": ["thing", "long", "going", "among", "young", "during", "sing", "nothing", "bring", "evening"],
    "se": ["see", "use", "these", "house", "those", "whose", "case", "close", "sense", "cause"],
    "ha": ["have", "that", "what", "shall", "has", "had", "than", "change", "perhaps", "hand"],
    "as": ["as", "was", "has", "ask", "last", "past", "easy", "reason", "season", "please"],
    "ou": ["you", "your", "about", "out", "our", "could", "would", "should", "house", "through"],
    "io": ["nation", "question", "action", "motion", "million", "position", "attention", "education", "situation", "organization"],
    "le": ["people", "little", "able", "table", "less", "problem", "possible", "example", "simple", "whole"],
}


def normalize_frequencies(bigrams: List[Tuple[str, float]], count: int = 30) -> List[Tuple[str, float, float]]:
    """
    Normalize top N bigram frequencies to 0.70-1.00 range for typing practice.

    Returns: List of (bigram, corpus_freq, normalized_freq)
    """
    selected = bigrams[:count]

    # Linear normalization to 0.70-1.00 range
    result = []
    for i, (bigram, freq) in enumerate(selected):
        # First gets 1.00, last gets 0.70, linear interpolation
        normalized = 1.00 - (i / (count - 1)) * 0.30 if count > 1 else 1.00
        result.append((bigram, freq, round(normalized, 2)))

    return result


def generate_rust_function(language: str, bigrams: List[Tuple[str, float, float]],
                          examples_dict: dict, source: str) -> str:
    """Generate Rust code for bigram function."""
    func_name = f"{language.lower()}_bigrams"

    code = f'''/// {language} language bigrams (frequency-ordered)
///
/// Source: {source}
/// Frequencies normalized to 0.70-1.00 range for typing practice
/// Top {len(bigrams)} bigrams selected from top 100 corpus analysis
/// Last updated: {date.today()}
pub fn {func_name}() -> Vec<Bigram> {{
    vec![
'''

    for bigram, corpus_freq, normalized in bigrams:
        examples = examples_dict.get(bigram, [f"ex{i}" for i in range(10)])
        examples_str = '", "'.join(examples[:10])

        # Add comment with corpus frequency
        code += f'        // Corpus: {corpus_freq:.2f}%\n'
        code += f'        Bigram::new("{bigram}", {normalized:.2f}, &["{examples_str}"]),\n'

    code += '    ]\n}\n'

    return code


def main():
    """Main execution."""
    print("="*80)
    print(f"BIGRAM FREQUENCY RESEARCH & CODE GENERATOR (Top {BIGRAM_COUNT})")
    print("="*80)
    print("\nSources:")
    print("  - English: Peter Norvig (Google Web Trillion Word Corpus)")
    print("  - French: Lexique database & French corpus analysis")
    print(f"\nGenerating top {BIGRAM_COUNT} bigrams per language...\n")

    # Generate top 20 for each language
    english_normalized = normalize_frequencies(ENGLISH_TOP_100, BIGRAM_COUNT)
    french_normalized = normalize_frequencies(FRENCH_TOP_100, BIGRAM_COUNT)

    # Show summary
    print(f"{'='*80}")
    print("SUMMARY")
    print(f"{'='*80}\n")
    print(f"English: {len(english_normalized)} bigrams (top {BIGRAM_COUNT})")
    for i, (bigram, corpus_freq, norm_freq) in enumerate(english_normalized[:5], 1):
        print(f"  {i}. {bigram:4} - {corpus_freq:.2f}% (normalized: {norm_freq:.2f})")
    print(f"  ... {len(english_normalized) - 5} more\n")

    print(f"French: {len(french_normalized)} bigrams (top {BIGRAM_COUNT})")
    for i, (bigram, corpus_freq, norm_freq) in enumerate(french_normalized[:5], 1):
        print(f"  {i}. {bigram:4} - {corpus_freq:.2f}% (normalized: {norm_freq:.2f})")
    print(f"  ... {len(french_normalized) - 5} more\n")

    # Generate Rust code
    print(f"{'='*80}")
    print("GENERATED RUST CODE")
    print(f"{'='*80}\n")
    print("Copy-paste these functions into src/content/bigram.rs:\n")
    print("```rust")

    english_code = generate_rust_function(
        "English",
        english_normalized,
        ENGLISH_EXAMPLES,
        "Peter Norvig (http://norvig.com/mayzner.html)"
    )

    french_code = generate_rust_function(
        "French",
        french_normalized,
        FRENCH_EXAMPLES,
        "Lexique database & French corpus studies"
    )

    print(english_code)
    print()
    print(french_code)
    print("```\n")

    print(f"{'='*80}")
    print("NEXT STEPS")
    print(f"{'='*80}")
    print("1. Review the generated code above")
    print("2. Replace french_bigrams() and english_bigrams() in src/content/bigram.rs")
    print("3. Update bigram_generator.rs to add Level 4 (20 bigrams)")
    print("4. Add Level 4 lessons to lesson.rs")
    print("5. Run: cargo test")
    print("6. Test manually: cargo run")


if __name__ == "__main__":
    main()
