/// Checks if a word matches a given candidate with at most the given maximum amount of `deletions`, `insertions` and `substitution`.
///
/// Returns a tuple of `(bool, u16)` where the boolean is `true` if the word matches the candidate, and the `u16` is the total number of operations done to match the two words.
///
/// The algorithm first finds the matching prefix of the two words using `SIMD` if available, and then continues with a scalar algorithm from the mismatch point.
///
/// The maximum amount of `deletions`, `insertions` and `substitutions` are given as mutable parameters, and are decreased by one each time an operation is done.
///
/// If the word matches the candidate with at most the given maximum amount of operations, the function returns true and the total number of operations done.
/// Otherwise, it returns `false` and `0`.
#[inline(always)]
pub fn matches_single<T: Eq>(
    word: &[T],
    candidate: &[T],
    mut max_deletions: usize,
    mut max_insertions: usize,
    mut max_substitutions: usize,
) -> (bool, usize) {
    let wlen = word.len();
    let clen = candidate.len();

    let mut wi = 0; // word index
    let mut ci = 0; // current index

    while wi < wlen && ci < clen {
        if word[wi] == candidate[ci] {
            wi += 1;
            ci += 1;
        } else if max_deletions > 0 && wi + 1 < wlen && word[wi + 1] == candidate[ci] {
            max_deletions -= 1;
            wi += 1;
        } else if max_insertions > 0 && ci + 1 < clen && word[wi] == candidate[ci + 1] {
            max_insertions -= 1;
            ci += 1;
        } else if max_substitutions > 0 {
            max_substitutions -= 1;
            wi += 1;
            ci += 1;
        } else {
            return (false, 0);
        }
    }

    let remaining_word = wlen - wi;
    let remaining_candidate = clen - ci;

    if remaining_word <= max_deletions && remaining_candidate <= max_insertions {
        (
            true,
            max_deletions - remaining_word + max_insertions - remaining_candidate
                + max_substitutions,
        )
    } else {
        (false, 0)
    }
}

/// Checks if `candidate` can be formed by deleting at most `max_deletions` bytes from `word`.
#[inline(always)]
pub fn matches_deletion_only(word: &[u8], candidate: &[u8], mut max_deletions: usize) -> (bool, usize) {
    let wlen = word.len();
    let clen = candidate.len();

    let mut wi = 0;
    let mut ci = 0;

    while wi < wlen && ci < clen {
        if max_deletions != usize::MAX {
            if word[wi] != candidate[ci] {
                max_deletions -= 1;
                wi += 1;
            } else {
                wi += 1;
                ci += 1;
            }
        } else {
            return (false, 0)
        }
    }

    (true,max_deletions - wlen - wi)
}

/// Checks if `candidate` can be formed by inserting at most `max_insertions` bytes into `word`.
#[inline(always)]
pub fn matches_insertion_only(word: &[u8], candidate: &[u8], max_insertions: usize) -> (bool, usize) {
    // An insertion in candidate relative to word is identical to a deletion from candidate relative to word
    matches_deletion_only(candidate, word, max_insertions)
}

#[cfg(test)]
mod matching {
    use crate::matching::{match_word::{matches_deletion_only, matches_insertion_only}, matches_single};

    #[test]
    fn single() {
        assert_eq!(matches_single("traanslatin".as_bytes(), "translation".as_bytes(), 2, 2, 0), (true, 2));
    }

    #[test]
    fn only_deletions() {
        assert_eq!(matches_deletion_only("transglagtioon".as_bytes(), "translation".as_bytes(), 3), (true, 0));
    }

    #[test]
    fn only_insertions() {
        assert_eq!(matches_insertion_only("tranlain".as_bytes(), "translation".as_bytes(), 3), (true, 0));
    }
}
