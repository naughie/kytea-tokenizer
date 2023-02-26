use crate::parser::Tags;
use crate::{ESCAPE, WORD_DELIM as DELIM};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Surface<'a>(pub &'a str);

impl<'a> Surface<'a> {
    pub fn is_ascii_whitespace(self) -> bool {
        let inner = self.0.as_bytes();
        !inner.is_empty() && inner[0].is_ascii_whitespace()
            || inner[0] == ESCAPE && inner[1] == DELIM
    }

    #[inline]
    pub const fn as_ptr(self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub const fn as_bytes(self) -> &'a [u8] {
        self.0.as_bytes()
    }

    #[inline]
    pub const fn as_str(self) -> &'a str {
        self.0
    }

    #[inline]
    pub const fn len(self) -> usize {
        self.0.len()
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0.is_empty()
    }
}

impl<'a, 'b> PartialEq<&'b str> for Surface<'a> {
    #[inline]
    fn eq(&self, &other: &&'b str) -> bool {
        self.0 == other
    }
}

impl<'a> From<Surface<'a>> for String {
    #[inline]
    fn from(Surface(s): Surface) -> Self {
        Self::from(s)
    }
}

impl AsRef<str> for Surface<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::ops::Deref for Surface<'_> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.0
    }
}

impl<'a> Tags<'a> for Surface<'a> {
    fn from_tags<I: Iterator<Item = &'a str>>(tags: &mut I) -> Self {
        if let Some(tag) = tags.next() {
            Self(tag)
        } else {
            Self("")
        }
    }
}
