//! `kytea-tokenizer` is a wrapper of KyTea, the japanese morphological analyzer.

pub(crate) mod kytea;
pub use kytea::run_cmd;

mod pos;
pub use pos::PoS;
pub use pos::PosIterator;

mod tokenizer;
pub use tokenizer::Surface;
pub use tokenizer::WordIterator;

use tokenizer::{Word, Words};

pub fn strip(out: impl AsRef<str>) -> String {
    let mut stripped = String::new();

    for line in out.as_ref().lines() {
        for word in Words::from(line) {
            word.pushed_to(&mut stripped);
            stripped.push(' ');
        }
        stripped.push('\n');
    }

    stripped
}

pub fn get_surface_and_pos(s: &str) -> (Surface<'_>, Option<PoS>) {
    Word::from(s).surface_and_pos()
}
