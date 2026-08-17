#![feature(test)]
extern crate test;
#[cfg(test)]
mod check {
    use spel_right::{English, SpellChecker};
    use test::Bencher;

    #[bench]
    fn iter_100000_correct_words(b: &mut Bencher) {
        let checker = SpellChecker::<English>::new();
        let bench_words: [&str; 30] = [
            "the",
            "quick",
            "brown",
            "fox",
            "jumps",
            "over",
            "lazy",
            "dog",
            "and",
            "sees",
            "nothing",
            "important",
            "about",
            "this",
            "statement",
            "programming",
            "algorithm",
            "structure",
            "compiler",
            "testing",
            "application",
            "language",
            "keyboard",
            "monitor",
            "software",
            "hardware",
            "network",
            "database",
            "system",
            "function",
        ];

        let mut words = Vec::with_capacity(100_000);
        let len = bench_words.len();
        for i in 0..100_000 {
            words.push(bench_words[i % len]);
        }

        b.iter(|| checker.batch_check(&words));
    }

    #[bench]
    fn par_iter_100000_correct_words(b: &mut Bencher) {
        let checker = SpellChecker::<English>::new();
        let bench_words: [&str; 30] = [
            "the",
            "quick",
            "brown",
            "fox",
            "jumps",
            "over",
            "lazy",
            "dog",
            "and",
            "sees",
            "nothing",
            "important",
            "about",
            "this",
            "statement",
            "programming",
            "algorithm",
            "structure",
            "compiler",
            "testing",
            "application",
            "language",
            "keyboard",
            "monitor",
            "software",
            "hardware",
            "network",
            "database",
            "system",
            "function",
        ];

        let mut words = Vec::with_capacity(100_000);
        let len = bench_words.len();
        for i in 0..100_000 {
            words.push(bench_words[i % len]);
        }

        b.iter(|| checker.batch_par_check(&words));
    }
}
