use crate::content::lesson::FingerPairType;
use crate::keyboard::azerty::{AzertyLayout, Finger, RowType};
use rand::seq::SliceRandom;

/// Extract keys assigned to a specific finger pair at a given difficulty level
pub fn get_finger_pair_keys(
    layout: &AzertyLayout,
    finger_pair: FingerPairType,
    level: u8,
    with_shift: bool,
) -> Vec<char> {
    let (left_finger, right_finger) = match finger_pair {
        FingerPairType::Pinky => (Finger::LeftPinky, Finger::RightPinky),
        FingerPairType::Ring => (Finger::LeftRing, Finger::RightRing),
        FingerPairType::Middle => (Finger::LeftMiddle, Finger::RightMiddle),
        FingerPairType::Index => (Finger::LeftIndex, Finger::RightIndex),
    };

    let allowed_rows = match level {
        1 => vec![RowType::Home],
        2 => vec![RowType::Home, RowType::Top, RowType::Bottom],
        3 => vec![
            RowType::Number,
            RowType::Top,
            RowType::Home,
            RowType::Bottom,
        ],
        _ => vec![],
    };

    let mut keys = Vec::new();

    for row in &layout.rows {
        if !allowed_rows.contains(&row.row_type) {
            continue;
        }

        for key in &row.keys {
            // Skip placeholder keys (null characters, newlines, modifier symbols)
            if key.base == '\0' || key.base == '\n' {
                continue;
            }

            // Include key if it matches either finger
            if key.finger == left_finger || key.finger == right_finger {
                // Add base character
                keys.push(key.base);

                // Add shift variant if with_shift is true and it exists
                if with_shift {
                    if let Some(shifted) = key.shift_variant {
                        // Avoid adding duplicates (some shift variants might equal base chars)
                        if shifted != key.base && !keys.contains(&shifted) {
                            keys.push(shifted);
                        }
                    }
                }
            }
        }
    }

    // Remove duplicates and sort for consistency
    keys.sort();
    keys.dedup();
    keys
}

/// Generate finger-pair drill content using 3-phase pattern
/// If with_shift is true, uses weighted distribution (50% lower, 40% upper, 10% symbols)
pub fn generate_finger_drills(keys: &[char], length: usize, with_shift: bool) -> String {
    if keys.is_empty() {
        return String::new();
    }

    if with_shift {
        generate_shift_drills(keys, length)
    } else {
        generate_base_drills(keys, length)
    }
}

/// Generate drills with only base characters (3-phase pattern)
fn generate_base_drills(keys: &[char], length: usize) -> String {
    let mut result = String::new();
    let mut patterns = Vec::new();

    // Phase 1: Single key repetitions (warm-up)
    // Pattern: "ff dd jj kk"
    for &key in keys {
        patterns.push(format!("{}{}", key, key));
    }

    // Phase 2: Adjacent pairs and reversals
    // Pattern: "fd df fj jf"
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            patterns.push(format!("{}{}", keys[i], keys[j]));
            patterns.push(format!("{}{}", keys[j], keys[i]));
        }
    }

    // Phase 3: Triplets with permutations (if enough keys)
    // Pattern: "fdj dfj jfd"
    if keys.len() >= 3 {
        for i in 0..keys.len() {
            for j in 0..keys.len() {
                for k in 0..keys.len() {
                    if i != j && j != k && i != k {
                        patterns.push(format!("{}{}{}", keys[i], keys[j], keys[k]));
                    }
                }
            }
        }
    }

    // Generate content by cycling through patterns
    let mut idx = 0;
    while result.len() < length {
        if !result.is_empty() {
            result.push(' ');
            // Check if adding space would exceed length
            if result.len() >= length {
                break;
            }
        }
        let pattern = &patterns[idx % patterns.len()];
        // Check if adding pattern would exceed length
        if result.len() + pattern.len() > length {
            break;
        }
        result.push_str(pattern);
        idx += 1;
    }

    result
}

/// Generate drills with shift variants (50% lower, 40% upper, 10% symbols)
fn generate_shift_drills(keys: &[char], length: usize) -> String {
    let mut rng = rand::thread_rng();

    // Separate keys into lowercase, uppercase, and symbols
    let lowercase: Vec<char> = keys.iter().filter(|c| c.is_lowercase()).copied().collect();
    let uppercase: Vec<char> = keys.iter().filter(|c| c.is_uppercase()).copied().collect();
    let symbols: Vec<char> = keys
        .iter()
        .filter(|c| !c.is_alphabetic())
        .copied()
        .collect();

    // Build weighted pool
    let mut pool = Vec::new();

    // 50% lowercase
    for _ in 0..50 {
        if !lowercase.is_empty() {
            pool.extend_from_slice(&lowercase);
        }
    }

    // 40% uppercase
    for _ in 0..40 {
        if !uppercase.is_empty() {
            pool.extend_from_slice(&uppercase);
        }
    }

    // 10% symbols
    for _ in 0..10 {
        if !symbols.is_empty() {
            pool.extend_from_slice(&symbols);
        }
    }

    if pool.is_empty() {
        // Fallback if no valid keys
        return String::new();
    }

    // Generate drill with 3-phase pattern using weighted pool
    let mut result = String::new();
    let mut patterns = Vec::new();

    // Phase 1: Repetitions
    for _ in 0..20 {
        let c = pool.choose(&mut rng).unwrap();
        patterns.push(format!("{}{}", c, c));
    }

    // Phase 2: Pairs
    for _ in 0..30 {
        let c1 = pool.choose(&mut rng).unwrap();
        let c2 = pool.choose(&mut rng).unwrap();
        patterns.push(format!("{}{}", c1, c2));
    }

    // Phase 3: Triplets
    for _ in 0..50 {
        let c1 = pool.choose(&mut rng).unwrap();
        let c2 = pool.choose(&mut rng).unwrap();
        let c3 = pool.choose(&mut rng).unwrap();
        patterns.push(format!("{}{}{}", c1, c2, c3));
    }

    // Generate content
    let mut idx = 0;
    while result.len() < length {
        if !result.is_empty() {
            result.push(' ');
            // Check if adding space would exceed length
            if result.len() >= length {
                break;
            }
        }
        let pattern = &patterns[idx % patterns.len()];
        // Check if adding pattern would exceed length
        if result.len() + pattern.len() > length {
            break;
        }
        result.push_str(pattern);
        idx += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_home_row_no_shift() {
        let layout = AzertyLayout::new();
        let keys = get_finger_pair_keys(&layout, FingerPairType::Middle, 1, false);

        // Expected: d (left home), k (right home)
        assert!(keys.contains(&'d'));
        assert!(keys.contains(&'k'));
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_middle_extended_no_shift() {
        let layout = AzertyLayout::new();
        let keys = get_finger_pair_keys(&layout, FingerPairType::Middle, 2, false);

        // Should include home (d, k), top (e, i), bottom (c, ;)
        assert!(keys.contains(&'d'));
        assert!(keys.contains(&'k'));
        assert!(keys.contains(&'e'));
        assert!(keys.contains(&'i'));
        assert!(keys.contains(&'c'));
        assert!(keys.contains(&';'));
        assert_eq!(keys.len(), 6);
    }

    #[test]
    fn test_middle_all_keys_no_shift() {
        let layout = AzertyLayout::new();
        let keys = get_finger_pair_keys(&layout, FingerPairType::Middle, 3, false);

        // Should include numbers: 4 (base '), 8 (base _)
        assert!(keys.contains(&'\''));
        assert!(keys.contains(&'_'));
        // Plus all level 2 keys
        assert!(keys.len() >= 8);
    }

    #[test]
    fn test_middle_all_keys_with_shift() {
        let layout = AzertyLayout::new();
        let keys = get_finger_pair_keys(&layout, FingerPairType::Middle, 3, true);

        // Should include both base and shift variants
        assert!(keys.contains(&'d'));
        assert!(keys.contains(&'D'));
        assert!(keys.contains(&'\''));
        assert!(keys.contains(&'4')); // shift of '
        assert!(keys.contains(&'_'));
        assert!(keys.contains(&'8')); // shift of _
    }

    #[test]
    fn test_all_finger_pairs_level_1() {
        let layout = AzertyLayout::new();
        for pair in [
            FingerPairType::Pinky,
            FingerPairType::Ring,
            FingerPairType::Middle,
            FingerPairType::Index,
        ] {
            let keys = get_finger_pair_keys(&layout, pair, 1, false);
            assert!(!keys.is_empty(), "Level 1 should have keys for {:?}", pair);
        }
    }

    #[test]
    fn test_level_progression() {
        let layout = AzertyLayout::new();
        let l1 = get_finger_pair_keys(&layout, FingerPairType::Index, 1, false);
        let l2 = get_finger_pair_keys(&layout, FingerPairType::Index, 2, false);
        let l3 = get_finger_pair_keys(&layout, FingerPairType::Index, 3, false);

        assert!(
            l2.len() > l1.len(),
            "Level 2 should have more keys than Level 1"
        );
        assert!(
            l3.len() > l2.len(),
            "Level 3 should have more keys than Level 2"
        );
    }

    #[test]
    fn test_no_placeholder_keys() {
        let layout = AzertyLayout::new();
        for pair in [
            FingerPairType::Pinky,
            FingerPairType::Ring,
            FingerPairType::Middle,
            FingerPairType::Index,
        ] {
            for level in 1..=3 {
                let keys = get_finger_pair_keys(&layout, pair, level, false);
                assert!(!keys.contains(&'\0'), "Should not contain null character");
                assert!(!keys.contains(&'\n'), "Should not contain newline");
            }
        }
    }

    #[test]
    fn test_generate_base_drills() {
        let keys = vec!['d', 'k'];
        let content = generate_finger_drills(&keys, 50, false);

        assert!(!content.is_empty());
        assert!(content.len() <= 50);
        assert!(content.contains('d') && content.contains('k'));
    }

    #[test]
    fn test_generate_shift_drills() {
        let keys = vec!['d', 'D', 'k', 'K'];
        let content = generate_finger_drills(&keys, 100, true);

        assert!(!content.is_empty());
        assert!(content.len() <= 100);
        // Should contain both lowercase and uppercase
        assert!(content.contains('d') || content.contains('D'));
        assert!(content.contains('k') || content.contains('K'));
    }

    #[test]
    fn test_content_generation_all_lessons() {
        let layout = AzertyLayout::new();
        for pair in [
            FingerPairType::Pinky,
            FingerPairType::Ring,
            FingerPairType::Middle,
            FingerPairType::Index,
        ] {
            for level in 1..=3 {
                for with_shift in [false, true] {
                    let keys = get_finger_pair_keys(&layout, pair, level, with_shift);
                    let content = generate_finger_drills(&keys, 100, with_shift);
                    assert!(
                        !content.is_empty(),
                        "Content should be generated for {:?} level {} shift={}",
                        pair,
                        level,
                        with_shift
                    );
                    assert!(content.len() <= 100);
                }
            }
        }
    }
}
