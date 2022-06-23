#[cfg(test)]
mod javascript_lexer_tests {
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
    let input = fs::read_to_string("tests/js_keywords.txt").expect("Error reading input file");
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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_identifiers() {
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
  fn lexer_parses_single_line_comment() {
    let input = String::from("// This is a comment");
    let expected = vec![Token::new(
      TokenKind::JavaScript(JavaScriptTokenKind::Comment),
      Span::new(0, 19),
    )];

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_multi_line_comment() {
    let input =
      fs::read_to_string("tests/js_multi_line_comment.txt").expect("Error reading input file");
    let expected = vec![Token::new(
      TokenKind::JavaScript(JavaScriptTokenKind::Comment),
      Span::new(0, 37),
    )];

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_string_literal_in_double_quotes() {
    let input = String::from("\"String in double quotes\"");
    let expected = vec![Token::new(
      TokenKind::JavaScript(JavaScriptTokenKind::String),
      Span::new(0, 24),
    )];

    let mut lexer = Lexer::new(input);
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

    let mut lexer = Lexer::new(input);
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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_boolean_literal() {
    let input = String::from("let myVar = false;");
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
        TokenKind::JavaScript(JavaScriptTokenKind::Boolean),
        Span::new(12, 16),
      ),
    ];

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }

  #[test]
  fn lexer_parses_array() {
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

    let mut lexer = Lexer::new(input);
    lexer.process_input(Language::JavaScript);

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
