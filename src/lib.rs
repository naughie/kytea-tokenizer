//! `kytea-tokenizer` is a wrapper of KyTea, the japanese morphological analyzer.

pub const WORD_DELIM: u8 = b'\t';
#[cfg(feature = "cmd")]
const DELIM_STR: &str = "\t";
pub const ESCAPE: u8 = b'\\';
pub const TAG_DELIM: u8 = b'/';

#[cfg(feature = "cmd")]
mod cmd;
#[cfg(feature = "cmd")]
pub use cmd::kytea_command as cmd;
#[cfg(feature = "cmd")]
pub use cmd::run_cmd;

#[cfg(feature = "ffi")]
pub mod ffi;

mod pos;
pub use pos::PoS;
pub use pos::PosIterator;

mod parser;
pub use parser::DefaultTags;
pub use parser::Surface;
pub use parser::Tags;
pub use parser::WordIterator;

pub mod tokenizer;
