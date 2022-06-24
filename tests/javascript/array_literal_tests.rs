#[cfg(test)]
mod array_literal_tests {
  use highlight_rs::lexer::{
    javascript::JavaScriptTokenKind,
    language::Language,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
  };

  #[test]
  fn lexer_parses_array_literal() {
    let input = String::from("let myVar = [123, 123];");
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
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(13, 15),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(18, 20),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_array_index_syntax() {
    let input = String::from("let myVar2 = myVar[0];");
    let expected = vec![
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(0, 2),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(4, 9),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(13, 17),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(19, 19),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
