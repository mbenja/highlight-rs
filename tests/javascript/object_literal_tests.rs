#[cfg(test)]
mod object_literal_tests {
  use highlight_rs::lexer::{
    javascript::JavaScriptTokenKind,
    language::Language,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
  };

  #[test]
  fn lexer_parses_object_literal() {
    let input = String::from("let myVar = { a: 1, b: 'a' };");
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
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(14, 14),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(17, 17),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(20, 20),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::String),
        Span::new(23, 25),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
