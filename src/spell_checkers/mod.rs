pub mod ascii;
pub mod normalized;
pub mod simple_len_group;
pub mod utf8;

pub use simple_len_group::words_to_groups;

use crate::{WordId, matching};

pub enum SpellCheckerTypes {
    Ascii(ascii::SpellChecker),
    Normalized(normalized::SpellChecker),
    Utf8(utf8::SpellChecker),
}

pub trait SpellCheckerTrait {
    type Group: WordGroup;
    
    fn get(&self, word: WordId) -> Option<&str>;
    fn get_unchecked(&self, word: WordId) -> &str;
    fn find(&self, word: &str) -> Option<WordId>;
    fn suggest_for_word(&self, word: &str) -> Vec<(&str, usize)>;
    
    fn word_to_matching(word: &str) -> &[impl Eq] {
        word.as_bytes()
    }
    
    #[inline]
    fn match_word_with_group(&self, group: Self::Group, word: &str) -> Vec<(&str, usize)> {
        let word_len = word.len();
        let dif = group.len as isize - word_len as isize;
        let abs_dif = dif.abs() as usize;

        let max_del = dif.max(0) as usize;
        let max_ins = (-dif).max(0) as usize;
        let max_chg = self.max_dif - abs_dif;
        
        group
            .blob
            .as_bytes()
            .par_chunks(group.len)
            .filter_map(|candidate| {
                if abs_dif == self.max_dif {
                    if candidate[0] != word[0] && candidate[candidate.len() - 1] != word[word_len - 1] {
                        return None;
                    }
                }

                let (is_ok, dist) =
                    matching::matches_single(candidate, Self::word_to_matching(word), max_del, max_ins, max_chg);
                if is_ok {
                    // Dataset will always be valid, and chars are based on len group. Cant have invalid utf-8.
                    // Trust
                    Some((unsafe { std::str::from_utf8_unchecked(candidate) }, dist))
                } else {
                    None
                }
            })
            .collect()
    }
        
    fn check(&self, word: &str) -> bool {
        self.find(word).is_some()
    }

    fn suggest_with_distances(&self, word: &str) -> Vec<(&str, usize)> {
        let word = word.to_lowercase();

        if let Some(word) = self.find(&word) {
            return vec![(self.get_unchecked(word), 0)];
        }

        let word_bytes = word.as_bytes();
        let mut result = self.suggest_for_word(word_bytes);

        if result.len() > 1 {
            result.par_sort_unstable_by_key(|(_, dist)| *dist);
            result.reverse();
        }

        result
    }
    
    /// Suggests words for a given `word` based on the maximum difference.
    ///
    /// If the `word` is found in the dataset, returns a vector with the given `word`.
    ///
    /// If the `word` is not found in the dataset, `SpellChecker::suggest_for_word()` will be used.
    ///
    /// Returns the result vector, sorted by the distance, and takes the first `take_first_x` elements.
    fn suggest(&self, word: &str, take_first_x: usize) -> Vec<&str> {
        let result = self.suggest_with_distances(word);
        if take_first_x == 0 {
            result.into_iter().map(|(word, _)| word).collect()
        } else {
            result
                .into_iter()
                .take(take_first_x)
                .map(|(word, _)| word)
                .collect()
        }
    }
}
