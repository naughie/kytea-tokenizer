use super::Word;
use crate::kytea::{DELIM, ESCAPE};

use std::num::NonZeroUsize;

#[derive(Debug, PartialEq, Eq)]
pub struct Words<'a> {
    inner: &'a str,
}

impl<'a> Iterator for Words<'a> {
    type Item = Word<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.find_sow();
        self.inner = &self.inner[pos..];

        let pos = self.find_eow()?.get();
        let inner = &self.inner[..pos];
        self.inner = &self.inner[pos..];

        Some(Word { inner })
    }
}

impl<'a> From<&'a str> for Words<'a> {
    fn from(inner: &'a str) -> Self {
        Self { inner }
    }
}

impl Words<'_> {
    fn enumerate(&self) -> impl Iterator<Item = (usize, u8)> + '_ {
        self.inner.as_bytes().iter().copied().enumerate()
    }

    fn find_sow(&self) -> usize {
        let mut it = self.enumerate().skip_while(|&(_, c)| c == DELIM);

        if let Some((i, _)) = it.next() {
            i
        } else {
            self.inner.len()
        }
    }

    fn find_eow(&self) -> Option<NonZeroUsize> {
        if self.inner.is_empty() {
            return None;
        }

        let mut prev_char = 0u8;
        for (i, c) in self.enumerate() {
            if c == DELIM && prev_char != ESCAPE {
                // SAFETY: self.inner[0] != DELIM
                return unsafe { Some(NonZeroUsize::new_unchecked(i)) };
            }

            prev_char = if c == ESCAPE && prev_char == ESCAPE {
                0
            } else {
                c
            };
        }

        // SAFETY: self.inner.len() > 0
        unsafe { Some(NonZeroUsize::new_unchecked(self.inner.len())) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_words() {
        let s = "";
        let mut words = Words::from(s);
        assert_eq!(words.next(), None);

        let s = "吾輩/名詞\tは/助詞\t\t猫/名詞\t /補助記号";
        let mut words = Words::from(s);
        assert_eq!(words.next(), Some(Word::from("吾輩/名詞")));
        assert_eq!(words.next(), Some(Word::from("は/助詞")));
        assert_eq!(words.next(), Some(Word::from("猫/名詞")));
        assert_eq!(words.next(), Some(Word::from(" /補助記号")));
        assert_eq!(words.next(), None);

        let s = "ab\t\\\t/補助記号\t\\/\\\t\t\\\\\\\t/\\\\\t";
        let mut words = Words::from(s);
        assert_eq!(words.next(), Some(Word::from("ab")));
        assert_eq!(words.next(), Some(Word::from("\\\t/補助記号")));
        assert_eq!(words.next(), Some(Word::from("\\/\\\t")));
        assert_eq!(words.next(), Some(Word::from("\\\\\\\t/\\\\")));
        assert_eq!(words.next(), None);
    }
}
