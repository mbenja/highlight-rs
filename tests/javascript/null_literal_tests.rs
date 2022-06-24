#[cfg(test)]
mod null_literal_tests {
  use highlight_rs::lexer::{
    javascript::JavaScriptTokenKind,
    language::Language,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
  };

  #[test]
  fn lexer_parses_null_literal() {
    let input = String::from("let myVar = null;");
    let expected = vec![
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(0, 2),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(4, 8),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Null),
        Span::new(12, 15),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
