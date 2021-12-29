use logos::Logos;

#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,

    #[token("&")]
    And,
    #[token("|")]
    Or,

    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token(">=")]
    GreaterThanOrEqualTo,
    #[token("<=")]
    LessThanOrEqualTo,

    #[token("=")]
    Equals,

    #[regex("[a-zA-Z]+")]
    Ident,
    #[token(" ")]
    Whitespace,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
    BinaryExpr,
    Root,
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some((kind, text))
    }
}
