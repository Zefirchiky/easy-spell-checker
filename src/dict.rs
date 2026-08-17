use std::marker::PhantomData;

use filess::{
    Dir, Json, Txt,
    traits::{FileTrait, ModelFile},
};

use crate::{DICT_VERSION, DictMetadata, Language, spell_checkers::ascii};

#[derive(Debug, Default)]
pub struct Dict<L: Language> {
    word_amount: usize,
    ascii_words: Vec<ascii::WordGroup>,
    ascii_freqs: Vec<usize>,
    // norm: Option<normalized::SpellChecker>,
    // utf8: Option<utf8::SpellChecker>,
    lang: PhantomData<L>,
}

impl<L: Language> Dict<L> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_ascii_words(&mut self, words: Vec<String>, freqs: Vec<usize>) -> Option<&mut Self> {
        let len = words.len();
        let (groups, freqs) = ascii::words_to_groups(words, freqs)?;
        self.word_amount += len;
        self.ascii_words = groups;
        self.ascii_freqs = freqs;
        Some(self)
    }

    /// WIP
    ///
    /// Normalized words will be checked against their ascii counterparts
    /// and mapped to utf-8 forms
    pub fn add_norm_words(&mut self, _words: Vec<String>) -> Option<&mut Self> {
        // self.word_amount += words.len();
        // let ascii_groups = words_to_groups(
        //     words
        //         .par_iter()
        //         .map(|w: &str| {
        //             w.nfd()
        //                 .filter(|ch| ch.is_ascii())
        //                 .collect::<String>()
        //         })
        //         .collect()
        // )?;
        // for word in words {

        // }
        // self.norm = Some(words);
        Some(self)
    }

    pub fn add_utf8_words(&mut self, _words: Vec<String>) -> Option<&mut Self> {
        // self.word_amount += words.len();
        // let groups = words_to_groups(words)?;
        // self.utf8 = Some(utf8::SpellChecker::new(groups));
        Some(self)
    }

    pub fn save(&self) {
        let lang_dir: Dir = crate::PROJECT_DIR.data_dir().join(L::name()).into();
        Json::new(lang_dir.join("metadata.json"))
            .save_model(&DictMetadata {
                language_short: L::name().into(),
                language_full: L::full_name().into(),
                version: DICT_VERSION,
                words_amount: self.word_amount,
                included_encodings: vec!["ascii".into()],
            })
            .unwrap();
        
        let ascii_dir: Dir = lang_dir.join("ascii").into();
        Json::new(ascii_dir.join("words.json"))
            .save_model(&self.ascii_words)
            .unwrap();
        Txt::new(ascii_dir.join("freq.txt"))
            .save(
                &self
                    .ascii_freqs
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .unwrap();
    }
}
