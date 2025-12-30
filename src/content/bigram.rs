/// Bigram training support for typing practice
/// Bigrams are common two-letter combinations that improve typing fluency
/// Language for natural bigrams
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    French,
    English,
}

/// Type of bigram practice
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BigramType {
    Natural, // Language bigrams (qu, th, er)
    Code,    // Programming symbols (-> :: =>)
}

/// A single bigram with frequency and example words
#[derive(Debug, Clone)]
pub struct Bigram {
    pub pattern: String,
    /// Phase 3: Natural language frequency weighting for future spaced repetition algorithms
    #[allow(dead_code)]
    pub frequency: f32, // 0.0 to 1.0, higher = more common (for future use)
    pub examples: Vec<String>,
}

impl Bigram {
    pub fn new(pattern: &str, frequency: f32, examples: &[&str]) -> Self {
        Self {
            pattern: pattern.to_string(),
            frequency,
            examples: examples.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// French language bigrams (frequency-ordered)
///
/// Source: Lexique database & French corpus studies
/// Frequencies normalized to 0.70-1.00 range for typing practice
/// Top 30 bigrams selected from top 100 corpus analysis
/// Last updated: 2025-12-30
pub fn french_bigrams() -> Vec<Bigram> {
    vec![
        // Corpus: 3.15%
        Bigram::new(
            "es",
            1.00,
            &[
                "les", "des", "mes", "ses", "ces", "tes", "test", "esprit", "reste", "gestes",
            ],
        ),
        // Corpus: 2.76%
        Bigram::new(
            "le",
            0.99,
            &[
                "le", "les", "lequel", "lent", "lecteur", "relever", "lever", "lettre", "parler",
                "aile",
            ],
        ),
        // Corpus: 2.55%
        Bigram::new(
            "de",
            0.98,
            &[
                "de", "des", "depuis", "devant", "dedans", "dehors", "demain", "devenir", "dessin",
                "monde",
            ],
        ),
        // Corpus: 2.47%
        Bigram::new(
            "en",
            0.97,
            &[
                "en",
                "ment",
                "bien",
                "rien",
                "encore",
                "enfant",
                "pendant",
                "moment",
                "content",
                "seulement",
            ],
        ),
        // Corpus: 2.30%
        Bigram::new(
            "re",
            0.96,
            &[
                "re",
                "entre",
                "faire",
                "reste",
                "être",
                "prendre",
                "regarder",
                "rendre",
                "représenter",
                "rencontre",
            ],
        ),
        // Corpus: 2.18%
        Bigram::new(
            "nt",
            0.95,
            &[
                "ment",
                "lent",
                "content",
                "sont",
                "maintenant",
                "avant",
                "enfant",
                "moment",
                "pendant",
                "souvent",
            ],
        ),
        // Accented: ér
        Bigram::new(
            "ér",
            0.94,
            &[
                "général",
                "opération",
                "américain",
                "numéro",
                "littéraire",
                "intérieur",
                "supérieur",
                "matériel",
                "érable",
                "vénérer",
            ],
        ),
        // Accented: és
        Bigram::new(
            "és",
            0.93,
            &[
                "présent",
                "résultat",
                "désir",
                "désormais",
                "président",
                "réserver",
                "réseau",
                "désigner",
                "résoudre",
                "désespoir",
            ],
        ),
        // Accented: ét
        Bigram::new(
            "ét",
            0.92,
            &[
                "été",
                "état",
                "étude",
                "détail",
                "société",
                "variété",
                "éternité",
                "étrange",
                "étoile",
                "étage",
            ],
        ),
        // Corpus: 2.15%
        Bigram::new(
            "on",
            0.91,
            &[
                "on", "bon", "son", "non", "dont", "long", "selon", "maison", "raison", "garçon",
            ],
        ),
        // Accented: èr
        Bigram::new(
            "èr",
            0.90,
            &[
                "père",
                "mère",
                "frère",
                "dernière",
                "manière",
                "lumière",
                "matière",
                "rivière",
                "prière",
                "première",
            ],
        ),
        // Accented: ée
        Bigram::new(
            "ée",
            0.89,
            &[
                "année", "journée", "idée", "armée", "entrée", "soirée", "durée", "pensée",
                "vallée", "musée",
            ],
        ),
        // Accented: à
        Bigram::new(
            "à ",
            0.88,
            &[
                "à la", "à le", "à ce", "à nous", "à vous", "à tout", "à Paris", "à moi", "à lui",
                "à elle",
            ],
        ),
        // Corpus: 2.13%
        Bigram::new(
            "er",
            0.87,
            &[
                "premier", "dernier", "aller", "mer", "cher", "hier", "hiver", "verre", "terre",
                "guerre",
            ],
        ),
        // Accented: ça
        Bigram::new(
            "ça",
            0.86,
            &[
                "ça",
                "français",
                "plaça",
                "traça",
                "commença",
                "avança",
                "lança",
                "effaça",
                "fiança",
                "menaça",
            ],
        ),
        // Corpus: 2.02%
        Bigram::new(
            "te",
            0.85,
            &[
                "te", "vite", "petite", "juste", "texte", "tente", "temps", "cette", "termine",
                "contenu",
            ],
        ),
        // Accented: ôt
        Bigram::new(
            "ôt",
            0.84,
            &[
                "tôt",
                "bientôt",
                "côté",
                "aussitôt",
                "tantôt",
                "sitôt",
                "plutôt",
                "dépôt",
                "impôt",
                "entrepôt",
            ],
        ),
        // Corpus: 1.91%
        Bigram::new(
            "el",
            0.83,
            &[
                "el", "tel", "bel", "cruel", "elle", "belle", "celle", "quelle", "nouvelle",
                "naturel",
            ],
        ),
        // Accented: ès
        Bigram::new(
            "ès",
            0.82,
            &[
                "très", "après", "près", "auprès", "exprès", "progrès", "congrès", "accès",
                "succès", "procès",
            ],
        ),
        // Corpus: 1.87%
        Bigram::new(
            "an",
            0.81,
            &[
                "an", "dans", "avant", "sans", "blanc", "grand", "ancien", "France", "manger",
                "changer",
            ],
        ),
        // Accented: çu
        Bigram::new(
            "çu",
            0.80,
            &[
                "reçu", "aperçu", "déçu", "conçu", "perçu", "reçue", "aperçue", "déçue", "conçue",
                "perçue",
            ],
        ),
        // Corpus: 1.85%
        Bigram::new(
            "et",
            0.79,
            &[
                "et", "cette", "paquet", "sujet", "petit", "bretelle", "secret", "complet",
                "projet", "objet",
            ],
        ),
        // Corpus: 1.83%
        Bigram::new(
            "qu",
            0.78,
            &[
                "que", "qui", "quoi", "quelque", "quel", "question", "pourquoi", "quand",
                "qualité", "quinze",
            ],
        ),
        // Corpus: 1.79%
        Bigram::new(
            "ou",
            0.77,
            &[
                "pour", "vous", "nous", "tout", "jour", "four", "ouvrir", "sous", "rouge", "lourd",
            ],
        ),
        // Corpus: 1.68%
        Bigram::new(
            "me",
            0.76,
            &[
                "me",
                "même",
                "femme",
                "homme",
                "moment",
                "merci",
                "mesure",
                "membre",
                "permettre",
                "commencement",
            ],
        ),
        // Corpus: 1.67%
        Bigram::new(
            "se",
            0.75,
            &[
                "se",
                "cesse",
                "penser",
                "promesse",
                "selon",
                "semaine",
                "base",
                "service",
                "ensemble",
                "heureusement",
            ],
        ),
        // Corpus: 1.62%
        Bigram::new(
            "it",
            0.74,
            &[
                "petit",
                "écrit",
                "dit",
                "fait",
                "suite",
                "politique",
                "situation",
                "habiter",
                "titre",
                "site",
            ],
        ),
        // Corpus: 1.58%
        Bigram::new(
            "la",
            0.73,
            &[
                "la", "laver", "place", "classe", "village", "blanc", "plan", "plat", "large",
                "plage",
            ],
        ),
        // Corpus: 1.57%
        Bigram::new(
            "ai",
            0.72,
            &[
                "ai", "mais", "fait", "jamais", "vrai", "laid", "aigle", "aider", "faire", "maison",
            ],
        ),
        // Corpus: 1.55%
        Bigram::new(
            "ne",
            0.71,
            &[
                "ne", "une", "personne", "bonne", "jeune", "semaine", "donner", "peine", "lune",
                "nettoyer",
            ],
        ),
        // Corpus: 1.54%
        Bigram::new(
            "ur",
            0.70,
            &[
                "pour", "jour", "toujours", "sur", "dure", "autour", "futur", "mesure", "nature",
                "figure",
            ],
        ),
        // Corpus: 1.52%
        Bigram::new(
            "ce",
            0.69,
            &[
                "ce", "cette", "celle", "ceci", "celle-ci", "France", "centre", "cela", "cesser",
                "accepter",
            ],
        ),
        // Corpus: 1.49%
        Bigram::new(
            "is",
            0.68,
            &[
                "mais",
                "dis",
                "fois",
                "jamais",
                "maison",
                "choisir",
                "histoire",
                "avis",
                "réaliser",
                "frais",
            ],
        ),
        // Corpus: 1.47%
        Bigram::new(
            "ra",
            0.67,
            &[
                "sera", "aura", "dira", "fera", "France", "travail", "courage", "grand", "traiter",
                "bravo",
            ],
        ),
        // Corpus: 1.42%
        Bigram::new(
            "ti",
            0.66,
            &[
                "action",
                "nation",
                "question",
                "information",
                "attention",
                "position",
                "condition",
                "tradition",
                "relation",
                "situation",
            ],
        ),
        // Corpus: 1.39%
        Bigram::new(
            "ri",
            0.65,
            &[
                "écrire", "prise", "esprit", "prix", "crier", "ouvrir", "sourire", "marine", "riz",
                "origine",
            ],
        ),
        // Corpus: 1.37%
        Bigram::new(
            "co",
            0.64,
            &[
                "comme",
                "encore",
                "corps",
                "école",
                "coin",
                "coeur",
                "compte",
                "accord",
                "économie",
                "découvrir",
            ],
        ),
        // Corpus: 1.35%
        Bigram::new(
            "ns",
            0.63,
            &[
                "dans",
                "sans",
                "cons",
                "ensemble",
                "considérer",
                "conseil",
                "construction",
                "transport",
                "penser",
                "ainsi",
            ],
        ),
        // Corpus: 1.33%
        Bigram::new(
            "at",
            0.62,
            &[
                "état",
                "chat",
                "bataille",
                "nature",
                "quatre",
                "atelier",
                "plateau",
                "attention",
                "atteindre",
                "attendre",
            ],
        ),
        // Corpus: 1.31%
        Bigram::new(
            "ma",
            0.61,
            &[
                "mais",
                "main",
                "matin",
                "manger",
                "maison",
                "maintenant",
                "image",
                "demain",
                "demande",
                "manquer",
            ],
        ),
    ]
}

/// English language bigrams (frequency-ordered)
///
/// Source: Peter Norvig (http://norvig.com/mayzner.html)
/// Frequencies normalized to 0.70-1.00 range for typing practice
/// Top 30 bigrams selected from top 100 corpus analysis
/// Last updated: 2025-12-30
pub fn english_bigrams() -> Vec<Bigram> {
    vec![
        // Corpus: 3.56%
        Bigram::new(
            "th",
            1.00,
            &[
                "the", "that", "with", "this", "think", "other", "three", "month", "through",
                "thought",
            ],
        ),
        // Corpus: 3.07%
        Bigram::new(
            "he",
            0.99,
            &[
                "he", "the", "when", "where", "she", "then", "them", "here", "there", "these",
            ],
        ),
        // Corpus: 2.43%
        Bigram::new(
            "in",
            0.98,
            &[
                "in", "thing", "nothing", "into", "think", "since", "find", "during", "living",
                "being",
            ],
        ),
        // Corpus: 2.05%
        Bigram::new(
            "er",
            0.97,
            &[
                "her", "over", "after", "never", "under", "ever", "there", "other", "where",
                "mother",
            ],
        ),
        // Corpus: 1.99%
        Bigram::new(
            "an",
            0.96,
            &[
                "an", "and", "can", "than", "many", "any", "want", "hand", "plan", "change",
            ],
        ),
        // Corpus: 1.85%
        Bigram::new(
            "re",
            0.95,
            &[
                "are", "were", "here", "there", "where", "before", "really", "great", "three",
                "free",
            ],
        ),
        // Corpus: 1.76%
        Bigram::new(
            "on",
            0.94,
            &[
                "on", "one", "upon", "only", "long", "among", "second", "person", "money", "reason",
            ],
        ),
        // Corpus: 1.49%
        Bigram::new(
            "at",
            0.93,
            &[
                "at", "that", "what", "late", "water", "great", "state", "date", "create", "matter",
            ],
        ),
        // Corpus: 1.45%
        Bigram::new(
            "en",
            0.92,
            &[
                "been", "when", "then", "open", "even", "often", "seven", "taken", "happen",
                "listen",
            ],
        ),
        // Corpus: 1.35%
        Bigram::new(
            "nd",
            0.91,
            &[
                "and",
                "end",
                "send",
                "kind",
                "find",
                "hand",
                "second",
                "understand",
                "behind",
                "friend",
            ],
        ),
        // Corpus: 1.34%
        Bigram::new(
            "ti",
            0.90,
            &[
                "time",
                "nation",
                "action",
                "question",
                "until",
                "still",
                "situation",
                "position",
                "mention",
                "attention",
            ],
        ),
        // Corpus: 1.34%
        Bigram::new(
            "es",
            0.89,
            &[
                "yes", "these", "best", "rest", "less", "does", "test", "question", "process",
                "business",
            ],
        ),
        // Corpus: 1.28%
        Bigram::new(
            "or",
            0.88,
            &[
                "or",
                "for",
                "more",
                "work",
                "world",
                "before",
                "word",
                "important",
                "order",
                "history",
            ],
        ),
        // Corpus: 1.20%
        Bigram::new(
            "te",
            0.87,
            &[
                "water", "after", "test", "system", "often", "state", "later", "create", "white",
                "write",
            ],
        ),
        // Corpus: 1.17%
        Bigram::new(
            "of",
            0.86,
            &[
                "of",
                "off",
                "offer",
                "office",
                "often",
                "coffee",
                "profile",
                "professor",
                "soft",
                "profit",
            ],
        ),
        // Corpus: 1.17%
        Bigram::new(
            "ed",
            0.84,
            &[
                "used", "called", "asked", "moved", "worked", "played", "tried", "needed",
                "wanted", "started",
            ],
        ),
        // Corpus: 1.13%
        Bigram::new(
            "is",
            0.83,
            &[
                "is", "this", "his", "list", "visit", "exist", "finish", "listen", "history",
                "discuss",
            ],
        ),
        // Corpus: 1.12%
        Bigram::new(
            "it",
            0.82,
            &[
                "it", "with", "wait", "write", "white", "little", "without", "within", "city",
                "visit",
            ],
        ),
        // Corpus: 1.09%
        Bigram::new(
            "al",
            0.81,
            &[
                "all", "also", "always", "already", "almost", "although", "really", "special",
                "social", "final",
            ],
        ),
        // Corpus: 1.07%
        Bigram::new(
            "ar",
            0.80,
            &[
                "are", "start", "part", "large", "art", "hard", "party", "year", "near", "market",
            ],
        ),
        // Corpus: 1.05%
        Bigram::new(
            "st",
            0.79,
            &[
                "just", "first", "most", "last", "still", "best", "rest", "test", "cost", "list",
            ],
        ),
        // Corpus: 1.04%
        Bigram::new(
            "to",
            0.78,
            &[
                "to", "into", "together", "today", "story", "history", "tomorrow", "toward",
                "total", "motor",
            ],
        ),
        // Corpus: 1.04%
        Bigram::new(
            "nt",
            0.77,
            &[
                "want",
                "went",
                "into",
                "point",
                "until",
                "front",
                "recent",
                "different",
                "important",
                "present",
            ],
        ),
        // Corpus: 0.95%
        Bigram::new(
            "ng",
            0.76,
            &[
                "thing", "long", "going", "among", "young", "during", "sing", "nothing", "bring",
                "evening",
            ],
        ),
        // Corpus: 0.93%
        Bigram::new(
            "se",
            0.75,
            &[
                "see", "use", "these", "house", "those", "whose", "case", "close", "sense", "cause",
            ],
        ),
        // Corpus: 0.93%
        Bigram::new(
            "ha",
            0.74,
            &[
                "have", "that", "what", "shall", "has", "had", "than", "change", "perhaps", "hand",
            ],
        ),
        // Corpus: 0.87%
        Bigram::new(
            "as",
            0.73,
            &[
                "as", "was", "has", "ask", "last", "past", "easy", "reason", "season", "please",
            ],
        ),
        // Corpus: 0.87%
        Bigram::new(
            "ou",
            0.72,
            &[
                "you", "your", "about", "out", "our", "could", "would", "should", "house",
                "through",
            ],
        ),
        // Corpus: 0.83%
        Bigram::new(
            "io",
            0.71,
            &[
                "nation",
                "question",
                "action",
                "motion",
                "million",
                "position",
                "attention",
                "education",
                "situation",
                "organization",
            ],
        ),
        // Corpus: 0.83%
        Bigram::new(
            "le",
            0.70,
            &[
                "people", "little", "able", "table", "less", "problem", "possible", "example",
                "simple", "whole",
            ],
        ),
    ]
}

/// Code/programming bigrams (frequency-ordered)
pub fn code_bigrams() -> Vec<Bigram> {
    vec![
        Bigram::new("->", 0.95, &["x -> y", "fn() ->", "|x| -> x"]),
        Bigram::new("::", 0.90, &["std::", "Vec::", "Self::"]),
        Bigram::new("=>", 0.88, &["x => x", "match => {", "() => {}"]),
        Bigram::new("!=", 0.85, &["x != y", "!= null", "!= 0"]),
        Bigram::new("==", 0.83, &["x == y", "== null", "== 0"]),
        Bigram::new("<=", 0.80, &["x <= y", "<= 10", "<= max"]),
        Bigram::new(">=", 0.78, &["x >= y", ">= 0", ">= min"]),
        Bigram::new("&&", 0.75, &["x && y", "&& true", "if x &&"]),
        Bigram::new("||", 0.72, &["x || y", "|| false", "if x ||"]),
        Bigram::new("//", 0.70, &["// comment", "// TODO", "// FIXME"]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_french_bigrams_frequency_order() {
        let bigrams = french_bigrams();

        // Verify descending frequency order
        for i in 0..bigrams.len() - 1 {
            assert!(
                bigrams[i].frequency >= bigrams[i + 1].frequency,
                "Bigrams should be ordered by frequency"
            );
        }
    }

    #[test]
    fn test_english_bigrams_frequency_order() {
        let bigrams = english_bigrams();

        for i in 0..bigrams.len() - 1 {
            assert!(bigrams[i].frequency >= bigrams[i + 1].frequency);
        }
    }

    #[test]
    fn test_code_bigrams_frequency_order() {
        let bigrams = code_bigrams();

        for i in 0..bigrams.len() - 1 {
            assert!(bigrams[i].frequency >= bigrams[i + 1].frequency);
        }
    }

    #[test]
    fn test_bigram_structure() {
        let bigrams = french_bigrams();

        assert!(!bigrams.is_empty());

        // Check first bigram
        let first = &bigrams[0];
        assert_eq!(first.pattern, "es");
        assert_eq!(first.frequency, 1.00);
        assert!(!first.examples.is_empty());
        assert!(first.examples.contains(&"les".to_string()));
    }

    #[test]
    fn test_all_bigrams_have_examples() {
        let all_bigrams = vec![french_bigrams(), english_bigrams(), code_bigrams()];

        for bigram_set in all_bigrams {
            for bigram in bigram_set {
                assert!(
                    !bigram.examples.is_empty(),
                    "Bigram '{}' should have examples",
                    bigram.pattern
                );
            }
        }
    }

    #[test]
    fn test_examples_contain_bigrams() {
        let all = vec![french_bigrams(), english_bigrams()];
        for set in all {
            for bigram in set {
                for example in &bigram.examples {
                    assert!(
                        example.to_lowercase().contains(&bigram.pattern),
                        "Example '{}' should contain bigram '{}'",
                        example,
                        bigram.pattern
                    );
                }
            }
        }
    }
}
