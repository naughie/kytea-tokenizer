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

impl<'a> DoubleEndedIterator for Words<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let pos = self.rfind_eow()?.get();
        self.inner = &self.inner[..pos];

        let pos = self.rfind_sow();
        let inner = &self.inner[pos..];
        self.inner = &self.inner[..pos];

        Some(Word { inner })
    }
}

impl<'a> From<&'a str> for Words<'a> {
    fn from(inner: &'a str) -> Self {
        Self { inner }
    }
}

impl Words<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn enumerate(&self) -> impl DoubleEndedIterator<Item = (usize, u8)> + '_ {
        self.inner.as_bytes().iter().copied().enumerate()
    }

    #[inline]
    fn renumerate(&self) -> impl Iterator<Item = (usize, u8)> + '_ {
        self.enumerate().rev()
    }

    fn find_sow(&self) -> usize {
        let mut it = self.enumerate().skip_while(|&(_, c)| c == DELIM);

        if let Some((i, _)) = it.next() {
            i
        } else {
            self.len()
        }
    }

    fn find_eow(&self) -> Option<NonZeroUsize> {
        if self.is_empty() {
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

    #[inline]
    fn iso_parity(i: usize, j: usize) -> bool {
        (i & 1) == (j & 1)
    }

    fn rfind_eow(&self) -> Option<NonZeroUsize> {
        let mut it = self.renumerate().skip_while(|&(_, c)| c == DELIM);

        if let Some((i, c)) = it.next() {
            if c == ESCAPE {
                // [^ESCAPE] ESCAPE ESCAPE* ESCAPE \t*
                //             ^              ^
                //             j              i
                let last = match it.filter(|&(_, c)| c == ESCAPE).last() {
                    Some((j, _)) if !Self::iso_parity(i, j) => i + 1,
                    _ => i + 2,
                };
                // SAFETY: i + 1 > 0
                unsafe { Some(NonZeroUsize::new_unchecked(last)) }
            } else {
                // SAFETY: i + 1 > 0
                unsafe { Some(NonZeroUsize::new_unchecked(i + 1)) }
            }
        } else {
            None
        }
    }

    fn rfind_sow(&self) -> usize {
        let mut delim = 0;
        let mut delim_found = false;
        let mut broken = false;

        for (i, c) in self.renumerate() {
            if delim_found && c != ESCAPE {
                // [^ESCAPE] ESCAPE* DELIM
                //     ^               ^
                //     i             delim
                if Self::iso_parity(i, delim) {
                    // # of ESCAPE's is odd
                    delim_found = false;
                } else {
                    // # of ESCAPE's is even
                    broken = true;
                    break;
                }
            }
            if c == DELIM {
                delim_found = true;
                delim = i;
            }
        }

        if delim_found && (broken || Self::iso_parity(delim, 0)) {
            delim + 1
        } else {
            0
        }
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

    #[test]
    fn test_words_rev() {
        let s = "";
        let mut words = Words::from(s).rev();
        assert_eq!(words.next(), None);

        let s = "吾輩/名詞\tは/助詞\t\t猫/名詞\t /補助記号";
        let mut words = Words::from(s).rev();
        assert_eq!(words.next(), Some(Word::from(" /補助記号")));
        assert_eq!(words.next(), Some(Word::from("猫/名詞")));
        assert_eq!(words.next(), Some(Word::from("は/助詞")));
        assert_eq!(words.next(), Some(Word::from("吾輩/名詞")));
        assert_eq!(words.next(), None);

        let s = "\t\tab\t\\\t/補助記号\t\\/\\\t\t\\\\\\\t/\\\\\t";
        let mut words = Words::from(s).rev();
        assert_eq!(words.next(), Some(Word::from("\\\\\\\t/\\\\")));
        assert_eq!(words.next(), Some(Word::from("\\/\\\t")));
        assert_eq!(words.next(), Some(Word::from("\\\t/補助記号")));
        assert_eq!(words.next(), Some(Word::from("ab")));
        assert_eq!(words.next(), None);

        let s = "\ta";
        let mut words = Words::from(s).rev();
        assert_eq!(words.next(), Some(Word::from("a")));
        assert_eq!(words.next(), None);

        let s = "\\\ta";
        let mut words = Words::from(s).rev();
        assert_eq!(words.next(), Some(Word::from("\\\ta")));
        assert_eq!(words.next(), None);

        let s = "\\\\\ta";
        let mut words = Words::from(s).rev();
        assert_eq!(words.next(), Some(Word::from("a")));
        assert_eq!(words.next(), Some(Word::from("\\\\")));
        assert_eq!(words.next(), None);
    }

    #[test]
    fn test_words_mixed() {
        let s = "吾輩/名詞\tは/助詞\t\t猫/名詞\t /補助記号";
        let mut words = Words::from(s);
        assert_eq!(words.next(), Some(Word::from("吾輩/名詞")));
        assert_eq!(words.next_back(), Some(Word::from(" /補助記号")));
        assert_eq!(words.next(), Some(Word::from("は/助詞")));
        assert_eq!(words.next_back(), Some(Word::from("猫/名詞")));
        assert_eq!(words.next(), None);
        assert_eq!(words.next_back(), None);
    }
}
