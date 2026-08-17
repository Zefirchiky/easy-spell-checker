use rayon::{iter::{IntoParallelIterator, ParallelIterator}, slice::ParallelSliceMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WordGroup {
    pub(crate) blob: String,
    pub(crate) len: usize,
    pub(crate) count: usize,
}

impl crate::WordGroup for WordGroup {
    fn blob(&self) -> &str {
        &self.blob
    }

    fn len(&self) -> usize {
        self.len
    }

    fn count(&self) -> usize {
        self.count
    }
}


pub fn words_to_groups(words: Vec<String>, freqs: Vec<usize>) -> Option<(Vec<WordGroup>, Vec<usize>)> {
    if words.len() != freqs.len() {
        return None;
    }
    
    let mut words_freqs: Vec<(String, usize)> = words.into_iter().zip(freqs).collect();
    words_freqs = words_freqs
        .into_par_iter()
        .filter(|(w, _)| !w.is_empty())
        .map(|(w, f)| (w.to_lowercase(), f))
        .collect();
    if words_freqs.is_empty() {
        return None;
    }

    words_freqs.par_sort_unstable_by(|(w1, _), (w2, _)| w1.len().cmp(&w2.len()).then(w1.cmp(w2))); // TODO: Check if unstable preserves needed order
    let (words, freqs): (Vec<String>, Vec<usize>) = words_freqs.into_iter().unzip();
    let biggest_len = words.last().unwrap().len();

    let mut groups: Vec<WordGroup> = Vec::with_capacity(biggest_len);
    for i in 1..=biggest_len {
        let mut wg = WordGroup::default();
        wg.len = i;
        groups.push(wg)
    }

    for word in words {
        let group = groups.get_mut(word.len() - 1).unwrap();
        group.blob.push_str(&word);
        group.count += 1;
    }

    Some((groups, freqs))
}