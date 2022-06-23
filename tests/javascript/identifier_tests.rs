#[cfg(test)]
mod identifier_tests {
  use highlight_rs::lexer::{
    javascript::JavaScriptTokenKind,
    language::Language,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
  };

  #[test]
  fn lexer_parses_variable_identifier() {
    let input = String::from("let myVar;");
    let expected = vec![
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(0, 2),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(4, 8),
      ),
    ];

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_function_identifier() {
    let input = String::from("function foo() { }");
    let expected = vec![
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(0, 7),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(9, 11),
      ),
    ];

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_function_parameter_identifier() {
    let input = String::from("function foo(myParam) { }");
    let expected = vec![
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(0, 7),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(9, 11),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(13, 19),
      ),
    ];

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
