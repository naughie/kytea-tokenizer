mod surface;
pub use surface::Surface;

mod word;
pub use word::Word;

mod words;
pub use words::Words;

use crate::PoS;

use std::iter::FlatMap;
use std::str::Lines;

type WordsFrom<'a> = fn(&'a str) -> Words<'a>;
type FlattenWords<'a> = FlatMap<Lines<'a>, Words<'a>, WordsFrom<'a>>;

pub struct WordIterator<'a> {
    word_it: FlattenWords<'a>,
}

impl<'a> WordIterator<'a> {
    pub fn from_lines(lines: &'a str) -> Self {
        Self {
            word_it: lines.lines().flat_map(Words::from),
        }
    }
}

impl<'a> Iterator for WordIterator<'a> {
    type Item = (Surface<'a>, Option<PoS>);

    fn next(&mut self) -> Option<Self::Item> {
        self.word_it.next().map(|word| word.surface_and_pos())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_surface_and_pos(
        res: Option<(Surface<'_>, Option<PoS>)>,
        expected_surface: &str,
        expected_pos: Option<PoS>,
    ) {
        assert_eq!(res, Some((Surface(expected_surface), expected_pos)));
    }

    #[test]
    fn word_iterator() {
        let words = "\na/名詞\tb/形容詞\nc/d\n\ne/UNK\n";
        let mut it = WordIterator::from_lines(words);
        assert_surface_and_pos(it.next(), "a", Some(PoS::名詞));
        assert_surface_and_pos(it.next(), "b", Some(PoS::形容詞));
        assert_surface_and_pos(it.next(), "c", None);
        assert_surface_and_pos(it.next(), "e", Some(PoS::UNK));
        assert!(it.next().is_none());
    }
}
