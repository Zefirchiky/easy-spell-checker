use rayon::{iter::{IntoParallelRefIterator, ParallelIterator}, slice::ParallelSlice};

use crate::{
    WordGroup, WordId, ascii, matching, spell_checkers::SpellCheckerTrait,
};

#[derive(Debug)]
pub struct SpellChecker {
    pub(crate) groups: Vec<ascii::WordGroup>,
    pub(crate) _freqs: Vec<usize>,
    pub max_dist: usize,
}

impl SpellChecker {
    pub fn new(groups: Vec<ascii::WordGroup>, freqs: Vec<usize>, max_dist: usize) -> Self {
        Self {
            groups,
            _freqs: freqs,
            max_dist,
            // added_words: vec![],
            // added_words_treshhold: 20,
        }
    }
}

impl SpellCheckerTrait for SpellChecker {
    type Group = ascii::WordGroup;

    fn max_distance(&self) -> usize {
        self.max_dist
    }
    
    fn get(&self, word: WordId) -> Option<&str> {
        let wg = self.groups.get(word.len)?;
        if word.offset >= wg.blob.len() {
            None
        } else {
            Some(&wg.blob[word.offset..word.offset + word.len])
        }
    }

    fn get_unchecked(&self, word: WordId) -> &str {
        let wg = &self.groups[word.len];
        &wg.blob[word.offset..word.offset + word.len]
    }
    
    fn find(&self, word: &str) -> Option<WordId> {
        let group = self.groups.get(word.len())?;
        Some(WordId {
            len: group.len,
            offset: group.find(word)?.0,
        })
    }

    #[inline(always)]
    fn match_word_with_group<'a>(&self, group: &'a Self::Group, word: &str) -> Vec<(&'a str, usize)> {
        let word = word.as_bytes();
        let word_len = word.len();
        let dif = word_len as isize - group.len as isize;   // < 0 if word is smaller, > 0 if word is bigger
        let abs_dif = dif.abs() as usize;
    
        let max_del = (-dif).max(0) as usize;
        let max_ins = dif.max(0) as usize;
        let max_sub = (self.max_dist - abs_dif).max(0);

        let words_per_batch = (crate::L1_CACHE_TARGET_BYTES / group.len).clamp(128, 4096);
        let batch_bytes = group.len * words_per_batch;

        if self.max_dist == abs_dif {
            if dif > 0 {
                group
                    .blob
                    .as_bytes()
                    .par_chunks(batch_bytes)
                    .flat_map_iter(|batch| {
                        batch.chunks_exact(group.len).filter_map(move |candidate| {
                            if candidate[0] != word[0]
                                && candidate[candidate.len() - 1] != word[word_len - 1]
                            {
                                return None;
                            }
            
                            let (is_ok, dist) =
                                matching::matches_insertion_only(word, candidate, max_del);
            
                            if is_ok {
                                // SAFETY: Dataset guarantees valid UTF-8 aligned to group.len
                                // Trust
                                Some((unsafe { std::str::from_utf8_unchecked(candidate) }, dist))
                            } else {
                                None
                            }
                        })
                    })
                    .collect()
            } else {
                group
                    .blob
                    .as_bytes()
                    .par_chunks(batch_bytes)
                    .flat_map_iter(|batch| {
                        batch.chunks_exact(group.len).filter_map(move |candidate| {
                            if candidate[0] != word[0]
                                && candidate[candidate.len() - 1] != word[word_len - 1]
                            {
                                return None;
                            }
            
                            let (is_ok, dist) =
                                matching::matches_deletion_only(word, candidate, max_del);
            
                            if is_ok {
                                // SAFETY: Dataset guarantees valid UTF-8 aligned to group.len
                                // Trust
                                Some((unsafe { std::str::from_utf8_unchecked(candidate) }, dist))
                            } else {
                                None
                            }
                        })
                    })
                    .collect()
            }
        } else {
            group
                .blob
                .as_bytes()
                .par_chunks(batch_bytes)
                .flat_map_iter(|batch| {
                    batch.chunks_exact(group.len).filter_map(move |candidate| {
                        let (is_ok, dist) =
                            matching::matches_single(candidate, word, max_del, max_ins, max_sub);
        
                        if is_ok {
                            // SAFETY: Dataset guarantees valid UTF-8 aligned to group.len
                            // Trust
                            Some((unsafe { std::str::from_utf8_unchecked(candidate) }, dist))
                        } else {
                            None
                        }
                    })
                })
                .collect()
        }
    }
    
    fn suggest_for_word(&self, word: &str) -> Vec<(&str, usize)> {
        if word.len() > self.groups.len() + self.max_dist {
            return vec![];
        }
        
        let word_len = word.len();
        let min_len = word_len.saturating_sub(self.max_dist - 1);
        let max_len = (word_len + self.max_dist).min(self.groups.len());
        
        self.groups[min_len..max_len]
            .as_ref()
            .par_iter()
            .filter(|group| group.count > 0)
            .flat_map(|g| self.match_word_with_group(g, word))
            .collect()
    }
}
