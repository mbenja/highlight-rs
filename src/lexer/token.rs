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

  pub fn get_kind(&self) -> &TokenKind {
    &self.kind
  }

  pub fn get_span(&self) -> &Span {
    &self.span
  }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  JavaScript(JavaScriptTokenKind),
}
