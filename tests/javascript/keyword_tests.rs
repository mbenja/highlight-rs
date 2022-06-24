#[cfg(test)]
mod keyword_tests {
  use std::fs;

  use highlight_rs::lexer::{
    javascript::JavaScriptTokenKind,
    language::Language,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
  };

  #[test]
  fn lexer_parses_all_keywords() {
    let input =
      fs::read_to_string("tests/javascript/js_keywords.txt").expect("Error reading input file");
    let expected = vec![
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(0, 4),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(6, 10),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(12, 15),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(17, 21),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(23, 27),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(29, 33),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(35, 42),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(44, 51),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(53, 59),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(61, 66),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(68, 69),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(71, 74),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(76, 79),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(81, 86),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(88, 94),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(96, 102),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(104, 106),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(108, 115),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(117, 118),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(120, 129),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(131, 136),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(138, 139),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(141, 150),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(152, 160),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(162, 164),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(166, 168),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(170, 176),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(178, 183),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(185, 191),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(193, 201),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(203, 208),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(210, 215),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(217, 221),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(223, 228),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(230, 233),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(235, 239),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(241, 243),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(245, 250),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(252, 254),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(256, 259),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(261, 265),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(267, 270),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(272, 276),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
