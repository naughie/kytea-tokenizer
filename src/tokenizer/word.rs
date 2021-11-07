use crate::kytea::{DELIM, ESCAPE};
use crate::PoS;

use super::Surface;

#[derive(Debug, PartialEq, Eq)]
pub struct Word<'a> {
    pub inner: &'a str,
}

impl<'a> From<&'a str> for Word<'a> {
    fn from(inner: &'a str) -> Self {
        Self { inner }
    }
}

impl<'a> Word<'a> {
    pub fn is_ascii_whitespace(&self) -> bool {
        let inner = self.inner.as_bytes();
        !inner.is_empty() && inner[0] == b' ' || inner[0] == ESCAPE && inner[1] == DELIM
    }

    pub fn pushed_to(&self, s: &mut String) {
        if self.inner.is_empty() {
            return;
        }

        if !self.is_ascii_whitespace() {
            let (_, eow) = self.find_end_of_surface_and_pos();
            s.push_str(&self.inner[..eow]);
        }
    }

    pub fn find_next_slash(&self, start: usize) -> usize {
        let mut prev_char = 0u8;
        for (i, &c) in self.inner[start..].as_bytes().iter().enumerate() {
            if c == b'/' && prev_char != ESCAPE {
                return start + i;
            }

            prev_char = if c == ESCAPE && prev_char == ESCAPE {
                0
            } else {
                c
            };
        }
        self.inner.len()
    }

    pub fn find_end_of_surface_and_pos(&self) -> (usize, usize) {
        let first = self.find_next_slash(0);
        let eow = if first == self.inner.len() {
            first
        } else {
            self.find_next_slash(first + 1)
        };
        (first, eow)
    }

    pub fn surface_and_pos(&self) -> (Surface<'a>, Option<PoS>) {
        let (first, eow) = self.find_end_of_surface_and_pos();
        let surface = Surface(&self.inner[..first]);
        let pos = if first == self.inner.len() {
            None
        } else {
            Some(&self.inner[(first + 1)..eow])
        };
        let pos = pos.and_then(|pos| pos.parse().ok());
        (surface, pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_pushed_word_eq(word: &str, expected: &str) {
        let mut s = String::new();
        let word = Word::from(word);
        word.pushed_to(&mut s);
        assert_eq!(s, expected);
    }

    #[test]
    fn test_word() {
        assert_pushed_word_eq("", "");
        assert_pushed_word_eq(" /補助記号", "");
        assert_pushed_word_eq("\\\t/補助記号", "");

        assert_pushed_word_eq("吾輩/名詞", "吾輩/名詞");
        assert_pushed_word_eq("吾輩/名詞/わがはい", "吾輩/名詞");
        assert_pushed_word_eq("吾輩/", "吾輩/");
        assert_pushed_word_eq("吾輩/名詞/", "吾輩/名詞");
        assert_pushed_word_eq("/名詞/わがはい", "/名詞");

        assert_pushed_word_eq("\\/吾輩/名詞/", "\\/吾輩/名詞");
        assert_pushed_word_eq("吾\\\\/輩/名詞/", "吾\\\\/輩");
        assert_pushed_word_eq("吾\\\\\\/輩/名詞/", "吾\\\\\\/輩/名詞");
    }

    fn assert_word_eq(word: &str, expected_surface: &str, expected_pos: Option<PoS>) {
        let (surface, pos) = Word::from(word).surface_and_pos();
        assert_eq!(surface, expected_surface);
        assert_eq!(pos, expected_pos);
    }

    #[test]
    fn test_get_surface_and_pos() {
        assert_word_eq("", "", None);
        assert_word_eq("吾輩", "吾輩", None);
        assert_word_eq("吾輩/", "吾輩", None);
        assert_word_eq("吾輩/名詞", "吾輩", Some(PoS::名詞));
        assert_word_eq("/名詞", "", Some(PoS::名詞));
        assert_word_eq("吾輩/名詞/", "吾輩", Some(PoS::名詞));
        assert_word_eq("吾輩/IllegalPoS", "吾輩", None);
    }
}
