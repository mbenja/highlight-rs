use super::{javascript::JavaScriptTokenKind, span::Span};

#[derive(Debug, PartialEq)]
pub struct Token {
  kind: TokenKind,
  span: Span,
}

impl Token {
  pub fn new(kind: TokenKind, span: Span) -> Token {
    Token { kind, span }
  }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  JavaScript(JavaScriptTokenKind),
}
