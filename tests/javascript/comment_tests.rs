#[cfg(test)]
mod comment_tests {
  use std::fs;

  use highlight_rs::lexer::{
    javascript::JavaScriptTokenKind,
    language::Language,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
  };

  #[test]
  fn lexer_parses_single_line_comment() {
    let input = String::from("// This is a comment");
    let expected = vec![Token::new(
      TokenKind::JavaScript(JavaScriptTokenKind::Comment),
      Span::new(0, 19),
    )];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_multi_line_comment() {
    let input = fs::read_to_string("tests/javascript/js_multi_line_comment.txt")
      .expect("Error reading input file");
    let expected = vec![Token::new(
      TokenKind::JavaScript(JavaScriptTokenKind::Comment),
      Span::new(0, 37),
    )];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
