use crate::parser::{Surface, WordIterator};

use std::iter::Enumerate;
use std::ops::ControlFlow;

#[cfg(all(feature = "json", not(feature = "tantivy")))]
use serde::{Deserialize, Serialize};

#[cfg(feature = "tantivy")]
use tantivy::tokenizer::{BoxTokenStream, TokenStream, Tokenizer};

#[cfg(feature = "tantivy")]
pub use tantivy::tokenizer::Token;
#[cfg(not(feature = "tantivy"))]
/// Token
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Token {
    /// Offset (byte index) of the first character of the token.
    /// Offsets shall not be modified by token filters.
    pub offset_from: usize,
    /// Offset (byte index) of the last character of the token + 1.
    /// The text that generated the token should be obtained by
    /// &text[token.offset_from..token.offset_to]
    pub offset_to: usize,
    /// Position, expressed in number of tokens.
    pub position: usize,
    /// Actual text content of the token.
    pub text: String,
}

#[cfg(not(feature = "tantivy"))]
impl Default for Token {
    #[inline]
    fn default() -> Self {
        Self {
            offset_from: 0,
            offset_to: 0,
            position: usize::MAX,
            text: String::new(),
        }
    }
}

pub struct TokenStreamParseOnly<'a> {
    original: &'a str,
    tokenized_text: Enumerate<WordIterator<'a, Surface<'a>>>,
    pub current_token: Token,
}

impl<'a> TokenStreamParseOnly<'a> {
    fn set_token(token: &mut Token, i: usize, surface: Surface<'a>, orig: &'a str) {
        token.text.clear();
        token.text.push_str(surface.as_str());
        token.position = i;
        // SAFETY: `orig` and `surface` are both parts of the same text, i.e. the `orig`.
        let offset_from = unsafe { surface.as_ptr().offset_from(orig.as_ptr()) } as usize;
        token.offset_from = offset_from;
        token.offset_to = offset_from + surface.len();
    }

    #[inline]
    pub fn from_tokenized_text(tokenized_text: &'a str) -> Self {
        Self {
            original: tokenized_text,
            tokenized_text: WordIterator::from_lines(tokenized_text).enumerate(),
            current_token: Token::default(),
        }
    }

    pub fn advance_token(&mut self) -> ControlFlow<()> {
        if let Some((i, surface)) = self.tokenized_text.next() {
            Self::set_token(&mut self.current_token, i, surface, self.original);
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    }
}

#[cfg(feature = "tantivy")]
impl<'a> TokenStream for TokenStreamParseOnly<'a> {
    #[inline]
    fn advance(&mut self) -> bool {
        self.advance_token().is_continue()
    }

    #[inline]
    fn token(&self) -> &Token {
        &self.current_token
    }

    #[inline]
    fn token_mut(&mut self) -> &mut Token {
        &mut self.current_token
    }
}

#[cfg(feature = "tantivy")]
#[derive(Debug, Clone, Copy)]
pub struct ParseOnly;

#[cfg(feature = "tantivy")]
impl Tokenizer for ParseOnly {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let stream = TokenStreamParseOnly::from_tokenized_text(text);
        stream.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "tantivy")]
    fn token(offset_from: usize, offset_to: usize, position: usize, text: &str) -> Token {
        Token {
            offset_from,
            offset_to,
            position,
            text: text.to_string(),
            position_length: 1,
        }
    }
    #[cfg(not(feature = "tantivy"))]
    fn token(offset_from: usize, offset_to: usize, position: usize, text: &str) -> Token {
        Token {
            offset_from,
            offset_to,
            position,
            text: text.to_string(),
        }
    }

    #[test]
    fn parse_only() {
        let mut stream = TokenStreamParseOnly::from_tokenized_text("a");
        assert!(stream.advance_token().is_continue());
        assert_eq!(&stream.current_token, &token(0, 1, 0, "a"));
        assert!(stream.advance_token().is_break());

        let mut stream = TokenStreamParseOnly::from_tokenized_text("a/記号/Ａ");
        assert!(stream.advance_token().is_continue());
        assert_eq!(&stream.current_token, &token(0, 1, 0, "a"));
        assert!(stream.advance_token().is_break());

        let mut stream = TokenStreamParseOnly::from_tokenized_text("a/記号/Ａ\tb/記号/Ｂ");
        assert!(stream.advance_token().is_continue());
        assert_eq!(&stream.current_token, &token(0, 1, 0, "a"));
        assert!(stream.advance_token().is_continue());
        assert_eq!(&stream.current_token, &token(13, 14, 1, "b"));
        assert!(stream.advance_token().is_break());
    }
}
