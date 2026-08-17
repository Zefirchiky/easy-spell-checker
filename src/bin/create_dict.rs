use std::{collections::HashSet, fs};

use filess::Txt;
use spel_right::English;

fn main() {
    let all_words =
        fs::read_to_string(Txt::new("/home/rei/dev/projects/spelright/words_alpha.txt"))
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();
    let parsed_words =
        fs::read_to_string(Txt::new("/home/rei/dev/projects/novel-compression-protocol/dataset/words.txt"))
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
    let freqs = fs::read_to_string(Txt::new("/home/rei/dev/projects/novel-compression-protocol/dataset/freqs.txt"))
        .unwrap()
        .lines()
        .map(|f| f.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (words, freqs): (Vec<String>, Vec<usize>) = parsed_words.into_iter().zip(freqs).filter(|(word, _)| all_words.contains(word)).collect();

    spel_right::Dict::<English>::new().add_ascii_words(words, freqs).unwrap().save();
}
