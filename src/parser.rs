use crate::{Filter, Lexer, SyntaxKind, SyntaxNode};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let tokens = Lexer::new(input);
        Self {
            builder: GreenNodeBuilder::new(),
            lexer: tokens.peekable(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.builder.start_node(SyntaxKind::Root.into());
        expr(&mut self);

        self.finish_node();

        Parse {
            green_node: self.builder.finish(),
        }
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder.token(Filter::kind_to_raw(kind), text.into());
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder
            .start_node_at(checkpoint, Filter::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        // chew through whitespace
        while self
            .lexer
            .peek()
            .map(|&(kind, _)| kind == SyntaxKind::Whitespace)
            .unwrap_or(false)
        {
            self.bump();
        }

        self.lexer.peek().map(|(kind, _)| *kind)
    }
}

pub(crate) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

enum InfixOp {
    And,
    Or,
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
}

impl InfixOp {
    fn binding_power(&self) -> (u8, u8) {
        (1, 2)
    }
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(SyntaxKind::Ident) => p.bump(),
        Some(SyntaxKind::OpenParen) => {
            p.bump();
            expr_binding_power(p, 0);

            assert_eq!(p.peek(), Some(SyntaxKind::CloseParen));
            p.bump();
        }
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::And) => InfixOp::And,
            Some(SyntaxKind::Or) => InfixOp::Or,
            Some(SyntaxKind::Equals) => InfixOp::Equals,
            Some(SyntaxKind::GreaterThan) => InfixOp::GreaterThan,
            Some(SyntaxKind::LessThan) => InfixOp::LessThan,
            Some(SyntaxKind::GreaterThanOrEqualTo) => InfixOp::GreaterThanOrEqualTo,
            Some(SyntaxKind::LessThanOrEqualTo) => InfixOp::LessThanOrEqualTo,
            _ => return, // we’ll handle errors later.
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            return;
        }

        // Eat the operator’s token.
        p.bump();

        p.start_node_at(checkpoint, SyntaxKind::BinaryExpr);
        expr_binding_power(p, right_binding_power);
        p.finish_node();
    }
}

#[test]
fn test_paren_binding_power() {
    let input = "(hello=INFO)|tokio=DEBUG";
    let parse = Parser::new(input).parse();
    println!("{}", parse.debug_tree());

    let input = "(hello = INFO) | tokio = DEBUG";
    let parse = Parser::new(input).parse();
    println!("{}", parse.debug_tree());
}
