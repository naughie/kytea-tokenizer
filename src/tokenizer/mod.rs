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
    type Item = (Surface<'a>, PoS);

    fn next(&mut self) -> Option<Self::Item> {
        self.word_it.next().map(Word::surface_and_pos)
    }
}

impl<'a> DoubleEndedIterator for WordIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.word_it.next_back().map(Word::surface_and_pos)
    }
}

#[cfg(feature = "tantivy")]
pub use tantivy_tokenizer::KyTea;

#[cfg(feature = "tantivy")]
mod tantivy_tokenizer {
    use tantivy::tokenizer::Token;
    use tantivy::tokenizer::Tokenizer;
    use tantivy::tokenizer::{BoxTokenStream, TokenStream};

    use super::{PoS, Surface, WordIterator};

    use std::iter::Enumerate;

    #[derive(Debug, Clone)]
    pub struct KyTea;

    impl Tokenizer for KyTea {
        fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
            KyTeaStream::from(text).into()
        }
    }

    struct KyTeaStream<'a> {
        original: &'a str,
        word_it: Enumerate<WordIterator<'a>>,
        token: Token,
    }

    impl<'a> KyTeaStream<'a> {
        fn from(text: &'a str) -> Self {
            KyTeaStream {
                original: text,
                word_it: WordIterator::from_lines(text).enumerate(),
                token: Token::default(),
            }
        }
    }

    impl<'a> TokenStream for KyTeaStream<'a> {
        fn advance(&mut self) -> bool {
            if let Some((i, (surface, pos))) = self.word_it.next() {
                self.token = to_token(self.original, surface, pos, i);
                true
            } else {
                false
            }
        }

        fn token(&self) -> &Token {
            &self.token
        }

        fn token_mut(&mut self) -> &mut Token {
            &mut self.token
        }
    }

    fn to_token<'a>(
        original: &'a str,
        Surface(surface): Surface<'a>,
        pos: PoS,
        position: usize,
    ) -> Token {
        // SAFETY: `original` and `surface` are both parts of the same text, i.e. the `original`.
        let offset_from = unsafe { surface.as_ptr().offset_from(original.as_ptr()) } as usize;
        let offset_to = offset_from + surface.len();
        let text = if pos != PoS::None {
            let mut word = format!("{}/", surface);
            word.push_str(pos.into());
            word
        } else {
            String::from(surface)
        };
        Token {
            offset_from,
            offset_to,
            position,
            text,
            position_length: 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        let mut it = WordIterator::from_lines(words);
        assert_surface_and_pos(it.next(), "a", PoS::名詞);
        assert_surface_and_pos(it.next(), "b", PoS::形容詞);
        assert_surface_and_pos(it.next(), "c", PoS::None);
        assert_surface_and_pos(it.next(), "e", PoS::UNK);
        assert!(it.next().is_none());
    }
}
