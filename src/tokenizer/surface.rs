use super::Word;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Surface<'a>(pub &'a str);

impl<'a> Surface<'a> {
    pub fn is_ascii_whitespace(&self) -> bool {
        Word::from(self.0).is_ascii_whitespace()
    }
}

impl<'a, 'b> PartialEq<&'b str> for Surface<'a> {
    #[inline]
    fn eq(&self, &other: &&'b str) -> bool {
        self.0 == other
    }
}
