mod surface;
pub use surface::Surface;

mod tag;
use tag::TagIterator;
pub use tag::{DefaultTags, Tags};

mod words;
pub use words::Words;

use std::iter::FlatMap;
use std::marker::PhantomData;
use std::str::Lines;

type WordsFrom<'a> = fn(&'a str) -> Words<'a>;
type FlattenWords<'a> = FlatMap<Lines<'a>, Words<'a>, WordsFrom<'a>>;

#[derive(Debug, Clone)]
pub struct WordIterator<'a, T> {
    word_it: FlattenWords<'a>,
    pub len_last_consumed: usize,
    phantom: PhantomData<T>,
}

impl<'a, T> WordIterator<'a, T> {
    pub fn from_lines(lines: &'a str) -> Self {
        Self {
            word_it: lines.lines().flat_map(Words::from),
            len_last_consumed: 0,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn clone_with_tags<NewT: Tags<'a>>(&self) -> WordIterator<'a, NewT> {
        WordIterator {
            word_it: self.word_it.clone(),
            len_last_consumed: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Tags<'a>> Iterator for WordIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.word_it.next().map(|word| {
            self.len_last_consumed = word.len();
            T::from_tags(&mut TagIterator::from(word))
        })
    }
}

impl<'a, T: Tags<'a>> DoubleEndedIterator for WordIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.word_it.next_back().map(|word| {
            self.len_last_consumed = word.len();
            T::from_tags(&mut TagIterator::from(word))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::PoS;

    fn assert_surface_and_pos(
        res: Option<(Surface<'_>, PoS)>,
        expected_surface: &str,
        expected_pos: PoS,
    ) {
        assert_eq!(res, Some((Surface(expected_surface), expected_pos)));
    }

    #[test]
    fn word_iterator() {
        let words = "\na/名詞\tb/形容詞\nc/d\n\ne/UNK\n";
        let mut it = WordIterator::<(Surface, PoS)>::from_lines(words);
        assert_surface_and_pos(it.next(), "a", PoS::名詞);
        assert_eq!(it.len_last_consumed, 8);
        assert_surface_and_pos(it.next(), "b", PoS::形容詞);
        assert_eq!(it.len_last_consumed, 11);
        assert_surface_and_pos(it.next(), "c", PoS::None);
        assert_eq!(it.len_last_consumed, 3);
        assert_surface_and_pos(it.next(), "e", PoS::UNK);
        assert_eq!(it.len_last_consumed, 5);
        assert!(it.next().is_none());

        let mut it = WordIterator::<Surface>::from_lines(words);
        assert_eq!(it.next(), Some(Surface("a")));
        assert_eq!(it.len_last_consumed, 8);
        assert_eq!(it.next(), Some(Surface("b")));
        assert_eq!(it.len_last_consumed, 11);
        assert_eq!(it.next(), Some(Surface("c")));
        assert_eq!(it.len_last_consumed, 3);
        assert_eq!(it.next(), Some(Surface("e")));
        assert_eq!(it.len_last_consumed, 5);
        assert!(it.next().is_none());
    }
}
