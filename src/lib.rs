/// language
///
/// ```bash
/// target = foo | (target = bar & level >= info) | level >= error
/// span.key = "foo"
/// event.key = "bar"
/// ```
///
/// ```compile_fail
/// let filter = Filter::new()
///    | filter::target("foo")
///    | (filter::target("bar") & LevelFilter::INFO)
///    | LevelFilter::ERROR
/// ```
///
/// is equivalent to:
///
/// ```compile_fail
/// let filter = "target = foo | (target = bar & level >= info) | level >= error"
///     .parse::<Filter>()
///     .unwrap();
/// ```
///
/// is equivalent to
///
/// ```compile_fail
/// let filter = Filter::new()
///     | filter::target("foo")
///     | "(target = bar & level >= info)".parse::<Filter>().unwrap()
///     | LevelFilter::ERROR;
/// ```
///
///
/// credit: https://arzg.github.io/lang/10/
mod lexer;
mod parser;

use lexer::{Lexer, SyntaxKind};
pub use parser::{Parse, Parser};

/// Some boilerplate is needed, as rowan settled on using its own
/// `struct SyntaxKind(u16)` internally, instead of accepting the
/// user's `enum SyntaxKind` as a type parameter.
///
/// First, to easily pass the enum variants into rowan via `.into()`:
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Filter;

impl rowan::Language for Filter {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 < 14);
        unsafe { std::mem::transmute(raw) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

pub(crate) type SyntaxNode = rowan::SyntaxNode<Filter>;
