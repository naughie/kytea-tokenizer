#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use strum::{Display, EnumString, IntoStaticStr};

use num_derive::FromPrimitive;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    PartialOrd,
    Ord,
    Hash,
    EnumString,
    Display,
    IntoStaticStr,
    FromPrimitive,
)]
#[repr(u8)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum PoS {
    名詞,
    動詞,
    接尾辞,
    形容詞,
    代名詞,
    副詞,
    形状詞,
    連体詞,
    接頭辞,
    接続詞,
    感動詞,
    助詞,
    補助記号,
    語尾,
    助動詞,
    URL,
    記号,
    空白,
    言いよどみ,
    英単語,
    UNK,
    None,
}

impl Default for PoS {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

impl PoS {
    #[inline]
    pub fn from_prim(n: u8) -> Self {
        use num_traits::FromPrimitive;
        <Self as FromPrimitive>::from_u8(n).unwrap_or(Self::None)
    }

    #[inline]
    pub fn to_prim(self) -> u8 {
        self as u8
    }

    pub fn append_to(self, surface: &mut String) {
        surface.push('/');
        surface.push_str(self.into());
    }

    #[inline]
    pub fn iter() -> PosIterator {
        PosIterator::new()
    }

    pub fn repeat<F, T>(f: F) -> impl Iterator<Item = T>
    where
        F: Fn(Self) -> T,
    {
        PosIterator::new().map(f)
    }

    fn next_pos(self) -> Option<Self> {
        use self::PoS::*;

        match self {
            名詞 => Some(動詞),
            動詞 => Some(接尾辞),
            接尾辞 => Some(形容詞),
            形容詞 => Some(代名詞),
            代名詞 => Some(副詞),
            副詞 => Some(形状詞),
            形状詞 => Some(連体詞),
            連体詞 => Some(接頭辞),
            接頭辞 => Some(接続詞),
            接続詞 => Some(感動詞),
            感動詞 => Some(助詞),
            助詞 => Some(補助記号),
            補助記号 => Some(語尾),
            語尾 => Some(助動詞),
            助動詞 => Some(URL),
            URL => Some(記号),
            記号 => Some(空白),
            空白 => Some(言いよどみ),
            言いよどみ => Some(英単語),
            英単語 => Some(UNK),
            UNK => Some(None),
            None => Option::None,
        }
    }
}

pub struct PosIterator {
    current: Option<PoS>,
}

impl Default for PosIterator {
    fn default() -> Self {
        Self {
            current: Some(PoS::名詞),
        }
    }
}

impl PosIterator {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Iterator for PosIterator {
    type Item = PoS;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.current {
            self.current = pos.next_pos();
            Some(pos)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pos_from_str() {
        assert_eq!(Ok(PoS::名詞), "名詞".parse());
    }

    #[test]
    fn pos_to_string() {
        assert_eq!(PoS::名詞.to_string(), "名詞");
    }

    #[test]
    #[cfg(feature = "json")]
    fn pos_ser() {
        let pos = PoS::名詞;
        assert!(serde_json::to_string(&pos).is_ok());
        assert_eq!(serde_json::to_string(&pos).unwrap(), r#""名詞""#);
    }
}
