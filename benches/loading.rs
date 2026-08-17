#![feature(test)]
extern crate test;
#[cfg(test)]
mod loading {
    use spel_right::{English, SpellChecker};
    use test::Bencher;

    #[bench]
    fn words_loading_from_file(b: &mut Bencher) {
        b.iter(|| SpellChecker::<English>::new());
    }
}
