mod dict;
mod dict_metadata;
mod english;
mod language;
mod load_dict;
mod matching;
mod spell_checker;
mod spell_checkers;
mod word_group;

use std::sync::LazyLock;

pub use dict::Dict;
pub use dict_metadata::DictMetadata;
pub use english::English;
pub use language::Language;
pub use spell_checker::SpellChecker;
pub use spell_checkers::*;
pub use word_group::WordGroup;

pub use filess;

static PROJECT_DIR: LazyLock<directories::ProjectDirs> =
    LazyLock::new(|| directories::ProjectDirs::from("org", "Kuroda", "SpelRight").unwrap());
const DICT_VERSION: usize = 1;
const MAX_DIST: usize = 3;

#[derive(Debug, Clone)]
pub enum Encoding {
    Ascii,
    Normalized,
    Utf8,
}

#[derive(Debug, Clone, Copy)]
pub struct WordId {
    pub len: usize,
    pub offset: usize,
}
