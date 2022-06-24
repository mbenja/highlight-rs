#[cfg(test)]
mod string_literal_tests {
  use highlight_rs::lexer::{
    javascript::JavaScriptTokenKind,
    language::Language,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
  };

  #[test]
  fn lexer_parses_string_literal_in_double_quotes() {
    let input = String::from("\"String in double quotes\"");
    let expected = vec![Token::new(
      TokenKind::JavaScript(JavaScriptTokenKind::String),
      Span::new(0, 24),
    )];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_string_literal_in_single_quotes() {
    let input = String::from("'String in single quotes'");
    let expected = vec![Token::new(
      TokenKind::JavaScript(JavaScriptTokenKind::String),
      Span::new(0, 24),
    )];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_string_literal_in_back_ticks() {
    let input = String::from("`String in back ticks`");
    let expected = vec![Token::new(
      TokenKind::JavaScript(JavaScriptTokenKind::String),
      Span::new(0, 21),
    )];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
