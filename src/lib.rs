//! `kytea-tokenizer` is a wrapper of KyTea, the japanese morphological analyzer.

pub(crate) mod kytea;
pub use kytea::kytea_command as cmd;
pub use kytea::run_cmd;

mod pos;
pub use pos::PoS;
pub use pos::PosIterator;

mod tokenizer;
pub use tokenizer::DefaultTags;
pub use tokenizer::Surface;
pub use tokenizer::Tags;
pub use tokenizer::WordIterator;
