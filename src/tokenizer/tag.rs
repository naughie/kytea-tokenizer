use crate::kytea::ESCAPE;
use crate::kytea::TAG_DELIM;

pub trait Tags<'a> {
    fn from_tags<I: Iterator<Item = &'a str>>(tags: &mut I) -> Self;
}

macro_rules! impl_tags {
    ($($ty:ident),* $(,)?) => {
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        impl<'a, $($ty: Tags<'a>,)*> Tags<'a> for ($($ty,)*) {
            fn from_tags<I: Iterator<Item = &'a str>>(tags: &mut I) -> Self {
                $(
                    let $ty = <$ty as Tags<'a>>::from_tags(tags);
                )*
                ($($ty,)*)
            }
        }
    };
}

impl_tags! {}
impl_tags! { T1 }
impl_tags! { T1, T2 }
impl_tags! { T1, T2, T3 }
impl_tags! { T1, T2, T3, T4 }
impl_tags! { T1, T2, T3, T4, T5 }
impl_tags! { T1, T2, T3, T4, T5, T6 }
impl_tags! { T1, T2, T3, T4, T5, T6, T7 }
impl_tags! { T1, T2, T3, T4, T5, T6, T7, T8 }

impl<'a> Tags<'a> for &'a str {
    fn from_tags<I: Iterator<Item = &'a str>>(tags: &mut I) -> Self {
        if let Some(tag) = tags.next() {
            tag
        } else {
            ""
        }
    }
}

impl<'a> Tags<'a> for String {
    fn from_tags<I: Iterator<Item = &'a str>>(tags: &mut I) -> Self {
        let tag = <&str as Tags>::from_tags(tags);
        Self::from(tag)
    }
}

use crate::{PoS, Surface};
pub type DefaultTags<'a> = (Surface<'a>, PoS, &'a str);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct TagIterator<'a> {
    inner: &'a str,
}

impl<'a> Iterator for TagIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }

        let ind = self.find_next_slash();
        let ret = &self.inner[..ind];

        self.inner = if ind == self.inner.len() {
            ""
        } else {
            &self.inner[(ind + 1)..]
        };

        Some(ret)
    }
}

impl<'a> TagIterator<'a> {
    #[inline]
    pub(crate) fn from(inner: &'a str) -> Self {
        Self { inner }
    }

    fn find_next_slash(self) -> usize {
        let mut prev_char = 0u8;
        for (i, &c) in self.inner.as_bytes().iter().enumerate() {
            if c == TAG_DELIM && prev_char != ESCAPE {
                return i;
            }

            prev_char = if c == ESCAPE && prev_char == ESCAPE {
                0
            } else {
                c
            };
        }
        self.inner.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag_iter() {
        let mut it = TagIterator::from("");
        assert_eq!(it.next(), None);

        let mut it = TagIterator::from("abc");
        assert_eq!(it.next(), Some("abc"));
        assert_eq!(it.next(), None);

        let mut it = TagIterator::from("abc/");
        assert_eq!(it.next(), Some("abc"));
        assert_eq!(it.next(), None);

        let mut it = TagIterator::from("abc/def//ghi");
        assert_eq!(it.next(), Some("abc"));
        assert_eq!(it.next(), Some("def"));
        assert_eq!(it.next(), Some(""));
        assert_eq!(it.next(), Some("ghi"));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_tags() {
        use crate::PoS;
        use crate::Surface;

        type Test<'a> = (Surface<'a>, PoS, String);

        let mut tags = TagIterator::from("a/名詞");
        let tags = Test::from_tags(&mut tags);
        assert_eq!(tags, (Surface("a"), PoS::名詞, String::new()));

        let mut tags = TagIterator::from("a/名詞/b");
        let tags = Test::from_tags(&mut tags);
        assert_eq!(tags, (Surface("a"), PoS::名詞, String::from("b")));

        let mut tags = TagIterator::from("a/名詞/b/c");
        let tags = Test::from_tags(&mut tags);
        assert_eq!(tags, (Surface("a"), PoS::名詞, String::from("b")));

        let mut tags = TagIterator::from("a//b/c");
        let tags = Test::from_tags(&mut tags);
        assert_eq!(tags, (Surface("a"), PoS::None, String::from("b")));
    }
}
