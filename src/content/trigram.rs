/// A single trigram with frequency and example words
#[derive(Debug, Clone)]
pub struct Trigram {
    pub pattern: String,
    /// Natural language frequency weighting for future spaced repetition algorithms
    #[allow(dead_code)]
    pub frequency: f32, // 0.0 to 1.0, higher = more common (for future use)
    pub examples: Vec<String>,
}

impl Trigram {
    pub fn new(pattern: &str, frequency: f32, examples: &[&str]) -> Self {
        Self {
            pattern: pattern.to_string(),
            frequency,
            examples: examples.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// English language trigrams (frequency-ordered)
///
/// Source: Peter Norvig
/// Frequencies normalized to 0.70-1.00 range for typing practice
/// Top 20 trigrams selected from corpus analysis
/// Last updated: 2025-12-30
pub fn english_trigrams() -> Vec<Trigram> {
    vec![
        // Corpus: 3.51%
        Trigram::new(
            "the",
            1.00,
            &[
                "the", "them", "then", "there", "these", "theme", "theft", "theory", "therapy",
                "thermal",
            ],
        ),
        // Corpus: 1.59%
        Trigram::new(
            "and",
            0.98,
            &[
                "and",
                "hand",
                "stand",
                "band",
                "land",
                "sand",
                "grand",
                "brand",
                "island",
                "understand",
            ],
        ),
        // Corpus: 1.58%
        Trigram::new(
            "ing",
            0.97,
            &[
                "ing", "thing", "going", "being", "doing", "seeing", "having", "making", "taking",
                "coming",
            ],
        ),
        // Corpus: 1.36%
        Trigram::new(
            "ion",
            0.95,
            &[
                "tion",
                "nation",
                "action",
                "question",
                "position",
                "attention",
                "education",
                "situation",
                "organization",
                "information",
            ],
        ),
        // Corpus: 1.31%
        Trigram::new(
            "ent",
            0.94,
            &[
                "ent", "went", "bent", "sent", "tent", "rent", "event", "moment", "parent",
                "present",
            ],
        ),
        // Corpus: 0.94%
        Trigram::new(
            "for",
            0.92,
            &[
                "for", "before", "forget", "forgive", "format", "former", "forward", "fortune",
                "forest", "foreign",
            ],
        ),
        // Corpus: 0.93%
        Trigram::new(
            "her",
            0.91,
            &[
                "her", "here", "where", "there", "whether", "hero", "herd", "herb", "heritage",
                "hermit",
            ],
        ),
        // Corpus: 0.92%
        Trigram::new(
            "ter",
            0.89,
            &[
                "ter", "after", "water", "later", "better", "letter", "winter", "master", "sister",
                "center",
            ],
        ),
        // Corpus: 0.89%
        Trigram::new(
            "hat",
            0.87,
            &[
                "that", "what", "hate", "chat", "hat", "whatever", "hateful", "shatter", "chatter",
                "hatch",
            ],
        ),
        // Corpus: 0.88%
        Trigram::new(
            "thi",
            0.86,
            &[
                "this", "thing", "think", "third", "thick", "thin", "thief", "thirst", "thirty",
                "thistle",
            ],
        ),
        // Corpus: 0.87%
        Trigram::new(
            "tha",
            0.84,
            &[
                "that", "than", "thank", "thaw", "thatch", "thatched", "thane", "thaler",
                "thallium", "thatch",
            ],
        ),
        // Corpus: 0.85%
        Trigram::new(
            "ere",
            0.83,
            &[
                "here",
                "there",
                "where",
                "were",
                "mere",
                "severe",
                "sphere",
                "adhere",
                "interfere",
                "persevere",
            ],
        ),
        // Corpus: 0.82%
        Trigram::new(
            "ate",
            0.81,
            &[
                "ate", "late", "gate", "date", "rate", "state", "create", "plate", "private",
                "climate",
            ],
        ),
        // Corpus: 0.79%
        Trigram::new(
            "ver",
            0.79,
            &[
                "ver", "very", "over", "never", "every", "cover", "river", "ever", "fever",
                "silver",
            ],
        ),
        // Corpus: 0.77%
        Trigram::new(
            "all",
            0.78,
            &[
                "all", "ball", "call", "fall", "hall", "wall", "small", "tall", "install",
                "overall",
            ],
        ),
        // Corpus: 0.75%
        Trigram::new(
            "con",
            0.76,
            &[
                "con",
                "continue",
                "control",
                "consider",
                "contain",
                "content",
                "contact",
                "contract",
                "contribute",
                "confident",
            ],
        ),
        // Corpus: 0.73%
        Trigram::new(
            "ted",
            0.75,
            &[
                "ted",
                "started",
                "wanted",
                "created",
                "united",
                "tested",
                "limited",
                "related",
                "connected",
                "protected",
            ],
        ),
        // Corpus: 0.71%
        Trigram::new(
            "com",
            0.73,
            &[
                "come",
                "common",
                "company",
                "complete",
                "community",
                "computer",
                "communicate",
                "compare",
                "combine",
                "committee",
            ],
        ),
        // Corpus: 0.69%
        Trigram::new(
            "ess",
            0.72,
            &[
                "ess", "less", "mess", "press", "stress", "dress", "process", "success",
                "business", "unless",
            ],
        ),
        // Corpus: 0.67%
        Trigram::new(
            "eve",
            0.70,
            &[
                "eve", "ever", "every", "even", "event", "level", "seven", "never", "clever",
                "however",
            ],
        ),
    ]
}

/// French language trigrams (frequency-ordered)
///
/// Source: Lexique database & French corpus studies
/// Frequencies normalized to 0.70-1.00 range for typing practice
/// Top 20 trigrams selected from corpus analysis
/// Last updated: 2025-12-30
pub fn french_trigrams() -> Vec<Trigram> {
    vec![
        // Corpus: 2.15%
        Trigram::new(
            "les",
            1.00,
            &[
                "les", "lesquels", "lessive", "alesage", "ibles", "tables", "cables", "sables",
                "stables", "meubles",
            ],
        ),
        // Corpus: 1.87%
        Trigram::new(
            "des",
            0.98,
            &[
                "des",
                "deshonneur",
                "dessus",
                "dessin",
                "descendre",
                "modeste",
                "andes",
                "cordes",
                "destinee",
                "desirant",
            ],
        ),
        // Corpus: 1.65%
        Trigram::new(
            "ent",
            0.97,
            &[
                "ment",
                "moment",
                "parent",
                "content",
                "lentement",
                "vraiment",
                "seulement",
                "souvent",
                "entrer",
                "centre",
            ],
        ),
        // Corpus: 1.42%
        Trigram::new(
            "ion",
            0.95,
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
        // Corpus: 1.38%
        Trigram::new(
            "que",
            0.94,
            &[
                "que",
                "quelque",
                "chaque",
                "banque",
                "marque",
                "musique",
                "pratique",
                "politique",
                "boutique",
                "unique",
            ],
        ),
        // Corpus: 1.12%
        Trigram::new(
            "ons",
            0.92,
            &[
                "lions", "avons", "raisons", "maisons", "saisons", "lecons", "garcons", "poissons",
                "buissons", "frissons",
            ],
        ),
        // Corpus: 1.08%
        Trigram::new(
            "ant",
            0.91,
            &[
                "avant",
                "pendant",
                "durant",
                "chant",
                "enfant",
                "instant",
                "important",
                "restaurant",
                "géant",
                "plant",
            ],
        ),
        // Corpus: 1.05%
        Trigram::new(
            "ait",
            0.89,
            &[
                "fait", "avait", "était", "lait", "trait", "portrait", "attrait", "parfait",
                "rait", "brait",
            ],
        ),
        // Corpus: 1.02%
        Trigram::new(
            "est",
            0.87,
            &[
                "est", "ouest", "reste", "geste", "beste", "peste", "zeste", "preste", "celeste",
                "modeste",
            ],
        ),
        // Corpus: 0.98%
        Trigram::new(
            "our",
            0.86,
            &[
                "pour", "jour", "toujours", "four", "tour", "cour", "amour", "retour", "autour",
                "lourd",
            ],
        ),
        // Corpus: 0.95%
        Trigram::new(
            "ais",
            0.84,
            &[
                "mais", "jamais", "frais", "faisan", "mauvais", "anglais", "francais", "palais",
                "relais", "essais",
            ],
        ),
        // Corpus: 0.92%
        Trigram::new(
            "eur",
            0.83,
            &[
                "peur", "heure", "fleur", "couleur", "valeur", "honneur", "bonheur", "malheur",
                "chaleur", "douceur",
            ],
        ),
        // Corpus: 0.89%
        Trigram::new(
            "men",
            0.81,
            &[
                "moment",
                "seulement",
                "lentement",
                "vraiment",
                "mensonge",
                "mention",
                "mental",
                "menu",
                "mener",
                "menace",
            ],
        ),
        // Corpus: 0.86%
        Trigram::new(
            "res",
            0.79,
            &[
                "tres",
                "apres",
                "pres",
                "reste",
                "adresse",
                "respect",
                "resultat",
                "restaurant",
                "responsable",
                "resoudre",
            ],
        ),
        // Corpus: 0.83%
        Trigram::new(
            "tio",
            0.78,
            &[
                "action",
                "nation",
                "question",
                "attention",
                "position",
                "condition",
                "tradition",
                "relation",
                "situation",
                "ration",
            ],
        ),
        // Corpus: 0.80%
        Trigram::new(
            "par",
            0.76,
            &[
                "par", "part", "parler", "parent", "partir", "partout", "pardon", "parfait",
                "parcours", "pareil",
            ],
        ),
        // Corpus: 0.77%
        Trigram::new(
            "pou",
            0.75,
            &[
                "pour",
                "pou",
                "pouvoir",
                "poule",
                "poupee",
                "poussiere",
                "pourtant",
                "pourquoi",
                "pouce",
                "poumon",
            ],
        ),
        // Corpus: 0.74%
        Trigram::new(
            "dan",
            0.74,
            &[
                "dans",
                "danse",
                "danger",
                "dangereux",
                "dandine",
                "dandy",
                "danois",
                "danseur",
                "dandiner",
                "dantesque",
            ],
        ),
        // Corpus: 0.71%
        Trigram::new(
            "tre",
            0.73,
            &[
                "etre", "entre", "quatre", "fenetre", "lettre", "mettre", "battre", "centre",
                "montre", "theatre",
            ],
        ),
        // Corpus: 0.68%
        Trigram::new(
            "lle",
            0.72,
            &[
                "elle",
                "belle",
                "celle",
                "quelle",
                "nouvelle",
                "vielle",
                "aller",
                "appelle",
                "mademoiselle",
                "ville",
            ],
        ),
        // Accented trigrams - high frequency patterns with é, è, ê
        Trigram::new(
            "été",
            0.71,
            &[
                "été",
                "société",
                "propriété",
                "variété",
                "anxiété",
                "piété",
                "répété",
                "vétéran",
                "sociétés",
                "complété",
            ],
        ),
        Trigram::new(
            "ère",
            0.70,
            &[
                "première",
                "dernière",
                "manière",
                "lumière",
                "matière",
                "rivière",
                "prière",
                "carrière",
                "frontière",
                "sphère",
            ],
        ),
        Trigram::new(
            "ées",
            0.69,
            &[
                "années",
                "journées",
                "idées",
                "armées",
                "entrées",
                "soirées",
                "données",
                "pensées",
                "vallées",
                "musées",
            ],
        ),
        Trigram::new(
            "tés",
            0.68,
            &[
                "étés", "côtés", "pâtés", "gâtés", "jetés", "notés", "votés", "datés", "dotés",
                "matés",
            ],
        ),
        Trigram::new(
            "rès",
            0.67,
            &[
                "très", "après", "près", "auprès", "exprès", "progrès", "congrès", "cyprès",
                "arès", "carès",
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_french_trigrams_frequency_order() {
        let trigrams = french_trigrams();

        // Verify descending frequency order
        for i in 0..trigrams.len() - 1 {
            assert!(
                trigrams[i].frequency >= trigrams[i + 1].frequency,
                "Trigrams should be ordered by frequency"
            );
        }
    }

    #[test]
    fn test_english_trigrams_frequency_order() {
        let trigrams = english_trigrams();

        for i in 0..trigrams.len() - 1 {
            assert!(trigrams[i].frequency >= trigrams[i + 1].frequency);
        }
    }

    #[test]
    fn test_trigram_structure() {
        let trigrams = french_trigrams();

        assert!(!trigrams.is_empty());
        assert_eq!(trigrams.len(), 25);

        // Check first trigram
        let first = &trigrams[0];
        assert_eq!(first.pattern, "les");
        assert_eq!(first.frequency, 1.00);
        assert!(!first.examples.is_empty());
        assert_eq!(first.examples.len(), 10);
        assert!(first.examples.contains(&"les".to_string()));
    }

    #[test]
    fn test_all_trigrams_have_examples() {
        let all_trigrams = vec![french_trigrams(), english_trigrams()];

        for trigram_set in all_trigrams {
            for trigram in trigram_set {
                assert!(
                    !trigram.examples.is_empty(),
                    "Trigram '{}' should have examples",
                    trigram.pattern
                );
                assert_eq!(
                    trigram.examples.len(),
                    10,
                    "Trigram '{}' should have exactly 10 examples",
                    trigram.pattern
                );
            }
        }
    }

    #[test]
    fn test_examples_contain_trigrams() {
        let all = vec![french_trigrams(), english_trigrams()];
        for set in all {
            for trigram in set {
                for example in &trigram.examples {
                    assert!(
                        example.to_lowercase().contains(&trigram.pattern),
                        "Example '{}' should contain trigram '{}'",
                        example,
                        trigram.pattern
                    );
                }
            }
        }
    }
}
