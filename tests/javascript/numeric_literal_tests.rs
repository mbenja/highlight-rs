#[cfg(test)]
mod numeric_literal_tests {
  use highlight_rs::lexer::{
    javascript::JavaScriptTokenKind,
    language::Language,
    span::Span,
    token::{Token, TokenKind},
    Lexer,
  };

  #[test]
  fn lexer_parses_unsigned_number_base_10() {
    let input = String::from("let myVar = 123;");
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
        Span::new(12, 14),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_exponential_numbers() {
    let input = String::from("let myVar = 0e-5; let myVar = 0e+5; let myVar = 5e1;");
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
        Span::new(12, 15),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(18, 20),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(22, 26),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(30, 33),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(36, 38),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(40, 44),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(48, 50),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_binary_number() {
    let input = String::from("let myVar = 0b10;");
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
        Span::new(12, 15),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_binary_number() {
    let input = String::from("let myVar = 0b3;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_octal_number() {
    let input = String::from("let myVar = 0o755; let myVar = 0O755;");
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
        Span::new(12, 16),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(19, 21),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(23, 27),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(31, 35),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_octal_number() {
    let input = String::from("let myVar = 0o9;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_hexadecimal_number() {
    let input = String::from("let myVar = 0xA; let myVar = 0XA;");
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
        Span::new(12, 14),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(17, 19),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(21, 25),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(29, 31),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_hexadecimal_number() {
    let input = String::from("let myVar = 0xz;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_big_int_number() {
    let input =
      String::from("let myVar = 123n; let myVar = 0o7n; let myVar = 0xAn; let myVar = 0b1n;");
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
        Span::new(12, 15),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(18, 20),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(22, 26),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(30, 33),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(36, 38),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(40, 44),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(48, 51),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        Span::new(54, 56),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        Span::new(58, 62),
      ),
      Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Number),
        Span::new(66, 69),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_big_int_number() {
    let input = String::from("let myVar = 1n5;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_float_number() {
    let input = String::from("let myVar = 1.5;");
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
        Span::new(12, 14),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_float_number() {
    let input = String::from("let myVar = 1.5.5;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_numeric_separator_in_base_10_number() {
    let input = String::from("let myVar = 1_000_000;");
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
        Span::new(12, 20),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_numeric_separator_in_float_number() {
    let input = String::from("let myVar = 1_050.95;");
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
        Span::new(12, 19),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_numeric_separator_in_base_10_number() {
    let input = String::from("let myVar = 1_000_1_0");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_numeric_separator_in_binary_number() {
    let input = String::from("let myVar = 0b1010_0001_1000;");
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
        Span::new(12, 27),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_numeric_separator_in_binary_number() {
    let input = String::from("let myVar = 0b1010_00_01_1000;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_numeric_separator_in_octal_number() {
    let input = String::from("let myVar = 0o2_2_5_6;");
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
        Span::new(12, 20),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_numeric_separator_in_octal_number() {
    let input = String::from("let myVar = 0o2_2_55_6;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_numeric_separator_in_hexadecimal_number() {
    let input = String::from("let myVar = 0xA0_B0_C0;");
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
        Span::new(12, 21),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_invalid_numeric_separator_in_hexadecimal_number() {
    let input = String::from("let myVar = 0xA0_B00_C0;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_numeric_separator_in_big_int_number() {
    let input = String::from("let myVar = 1_000_000n;");
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
        Span::new(12, 21),
      ),
    ];

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_double_numeric_separator() {
    let input = String::from("let myVar = 1_000__000;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_ending_numeric_separator() {
    let input = String::from("let myVar = 1_000_000_;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_skips_numeric_separator_after_leading_zero() {
    let input = String::from("let myVar = 0_1;");
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

    let mut lexer = Lexer::new(&input);
    lexer.process_input(&Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
