use std::marker::PhantomData;

use filess::{
    Dir, Json, Txt,
    traits::{FileTrait, ModelFile},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    DictMetadata, Language, dict_metadata,
    spell_checkers::{SpellCheckerTrait, ascii},
};

#[derive(Serialize, Deserialize)]
pub struct SpellChecker<L: Language> {
    pub(crate) dict_meta: DictMetadata,
    #[serde(skip)]
    pub(crate) ascii_checker: Option<ascii::SpellChecker>,
    // pub(crate) norm_checker: Option<normalized::SpellChecker>,
    // pub(crate) utf8_checker: Option<utf8::SpellChecker>,
    pub(crate) language: PhantomData<L>,
}

impl<L: Language> SpellChecker<L> {
    pub fn new() -> Self {
        Self::new_with_dir(crate::PROJECT_DIR.data_dir().join(L::name()).into()).unwrap()
    }

    pub fn new_with_dir(dir: Dir) -> Result<Self, <Json as ModelFile>::Error> {
        let mfile = Json::new(dir.join("metadata.json"));
        assert!(mfile.exists(), "Metadata doesn't exist");
        let meta = mfile.load_model::<dict_metadata::DictMetadata>()?;

        let wfile = Json::new(dir.join("ascii/words.json"));
        assert!(wfile.exists(), "Words file doesn't exist");
        let groups = wfile.load_model::<Vec<ascii::WordGroup>>()?;

        let ffile = Txt::new(dir.join("ascii/freq.txt"));
        assert!(ffile.exists(), "Frequencies file doesn't exist");
        let freqs = String::from_utf8(ffile.load()?)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Ok(Self {
            dict_meta: meta,
            ascii_checker: Some(ascii::SpellChecker::new(groups, freqs, crate::MAX_DIST)),
            language: PhantomData,
        })
    }

    pub fn set_max_dist(&mut self, dist: usize) {
        if let Some(checker) = &mut self.ascii_checker {
            checker.max_dist = dist;
        }
    }

    pub fn check(&self, word: &str) -> bool {
        // FIXME: Branching may add overhead, compiling this for each language with their checkers would be best
        if let Some(checker) = &self.ascii_checker
            && checker.check(word)
        {
            return true;
        }
        // if let Some(checker) = &self.norm_checker && checker.check(word) { return true; }
        // if let Some(checker) = &self.utf8_checker && checker.check(word) { return true; }
        false
    }

    pub fn batch_check<'a>(&self, words: &'a [&str]) -> Vec<(&'a str, bool)> {
        words.iter().map(|&word| (word, self.check(word))).collect()
    }

    pub fn batch_par_check<'a>(&self, words: &'a [&str]) -> Vec<(&'a str, bool)> {
        words
            .par_iter()
            .map(|&word| (word, self.check(word)))
            .collect()
    }

    pub fn suggest(&self, word: &str) -> Vec<&str> {
        let mut words = vec![];
        if let Some(checker) = &self.ascii_checker {
            words.extend(checker.suggest(&word));
        }
        // if let Some(checker) = &self.norm_checker {
        //     words.extend(checker.suggest(&word));
        // }
        // if let Some(checker) = &self.utf8_checker {
        //     words.extend(checker.suggest(&word));
        // }
        words
    }

    /// Suggests words for each `word` in the given `words` vector based on the maximum difference specified in the constructor.
    ///
    /// If a `word` is found in the dataset, returns a vector with the given `word`.
    ///
    /// If a `word` is not found in the dataset, `SpellChecker::suggest_for_word()` will be used.
    pub fn batch_suggest<'a>(
        &self,
        words: &'a [&str],
    ) -> Vec<(&'a str, Vec<&str>)> {
        self.batch_suggest_iter(words).collect()
    }

    /// Iterates over each `word` in the given `words` vector and calls the given `callback` function with the suggestions for each word.
    ///
    /// The `callback` function will be called with two arguments: the original `word`, and a vector of suggestions for that word.
    ///
    /// The suggestions vector will be sorted by the distance, with the closest words first.
    /// If the `word` is found in the dataset, the suggestions vector will contain the given `word`.
    ///
    /// The `callback` function will be called for each `word` in the given `words` vector.
    pub fn batch_suggest_with<F>(&self, words: &[&str], mut callback: F)
    where
        F: FnMut(&str, Vec<&str>),
    {
        words.iter().for_each(move |word| {
            let suggestions = self.suggest(word);
            callback(word, suggestions)
        });
    }

    /// Iterates over each `word` in the given `words` vector and calls `suggest` function with the given `word` and `take_first_x`.
    ///
    /// The `suggest` function will return a vector of suggestions for each word, sorted by the distance, with the closest words first.
    ///
    /// The `suggest` function will also return the given `word` if it is found in the dataset.
    ///
    /// The function returns an iterator over the suggestions vectors.
    pub fn batch_suggest_iter<'a>(
        &self,
        words: &'a [&str],
    ) -> impl Iterator<Item = (&'a str, Vec<&str>)> {
        words
            .iter()
            .map(move |&word| (word, self.suggest(word)))
    }

    /// Iterates over each `word` in the given `words` vector and calls `suggest` function with the given `word` and `take_first_x`.
    ///
    /// The `suggest` function will return a vector of suggestions for each word, sorted by the distance, with the closest words first.
    ///
    /// The `suggest` function will also return the given `word` if it is found in the dataset.
    ///
    /// The function returns an iterator over the suggestions vectors.
    ///
    /// This function is the same as `batch_suggest`, but it uses rayon's parallel iterator, which means it will use all available CPU cores in parallel to suggest words for all given words.
    ///
    /// The function returns a vector of suggestions vectors.
    ///
    /// The function is parallel, and will use all available CPU cores in parallel.
    pub fn batch_par_suggest<'a>(
        &self,
        words: &'a [&str],
    ) -> Vec<(&'a str, Vec<&str>)> {
        self.batch_par_suggest_iter(words).collect()
    }

    /// Iterates over each `word` in the given `words` vector and calls the given `callback` function with the suggestions for each word.
    ///
    /// The `callback` function will be called with two arguments: the original `word`, and a vector of suggestions for that word.
    ///
    /// The suggestions vector will be sorted by the distance, with the closest words first.
    /// If the `word` is found in the dataset, the suggestions vector will contain the given `word`.
    ///
    /// The `callback` function will be called for each `word` in the given `words` vector.
    ///
    /// The function is parallel, and will use all available CPU cores in parallel.
    pub fn batch_par_suggest_with<F>(&self, words: &[&str], callback: F)
    where
        F: FnMut(&str, Vec<&str>) + Send + Sync + Clone,
    {
        words.par_iter().for_each_with(callback, move |cb, word| {
            let suggestions = self.suggest(word);
            cb(word, suggestions)
        });
    }

    /// Iterates over each `word` in the given `words` vector and calls `suggest` function with the given `word` and `take_first_x`.
    ///
    /// The `suggest` function will return a vector of suggestions for each word, sorted by the distance, with the closest words first.
    ///
    /// If the `word` is found in the dataset, the suggestions vector will contain the given `word`.
    ///
    /// The `suggest` function will take the first `take_first_x` elements of the suggestions vector.
    ///
    /// The function returns a parallel iterator over the suggestions vectors.
    pub fn batch_par_suggest_iter<'a>(
        &self,
        words: &'a [&str],
    ) -> impl ParallelIterator<Item = (&'a str, Vec<&str>)> {
        words
            .par_iter()
            .map(move |&word| (word, self.suggest(word)))
    }
}
