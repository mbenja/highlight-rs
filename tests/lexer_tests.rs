#[cfg(test)]
mod lexer_tests {
  use std::fs;

  use highlight_rs::lexer::{Keyword, Lexer, Span, Token, TokenKind};

  #[test]
  fn lexer_parses_all_keywords() {
    let input = fs::read_to_string("tests/all_keywords.txt").expect("Error reading input file");
    let expected = vec![
      Token::new(TokenKind::Keyword(Keyword::Await), Span::new(0, 4)),
      Token::new(TokenKind::Keyword(Keyword::Break), Span::new(6, 10)),
      Token::new(TokenKind::Keyword(Keyword::Case), Span::new(12, 15)),
      Token::new(TokenKind::Keyword(Keyword::Catch), Span::new(17, 21)),
      Token::new(TokenKind::Keyword(Keyword::Class), Span::new(23, 27)),
      Token::new(TokenKind::Keyword(Keyword::Const), Span::new(29, 33)),
      Token::new(TokenKind::Keyword(Keyword::Continue), Span::new(35, 42)),
      Token::new(TokenKind::Keyword(Keyword::Debugger), Span::new(44, 51)),
      Token::new(TokenKind::Keyword(Keyword::Default), Span::new(53, 59)),
      Token::new(TokenKind::Keyword(Keyword::Delete), Span::new(61, 66)),
      Token::new(TokenKind::Keyword(Keyword::Do), Span::new(68, 69)),
      Token::new(TokenKind::Keyword(Keyword::Else), Span::new(71, 74)),
      Token::new(TokenKind::Keyword(Keyword::Enum), Span::new(76, 79)),
      Token::new(TokenKind::Keyword(Keyword::Export), Span::new(81, 86)),
      Token::new(TokenKind::Keyword(Keyword::Extends), Span::new(88, 94)),
      Token::new(TokenKind::Keyword(Keyword::Finally), Span::new(96, 102)),
      Token::new(TokenKind::Keyword(Keyword::For), Span::new(104, 106)),
      Token::new(TokenKind::Keyword(Keyword::Function), Span::new(108, 115)),
      Token::new(TokenKind::Keyword(Keyword::If), Span::new(117, 118)),
      Token::new(TokenKind::Keyword(Keyword::Implements), Span::new(120, 129)),
      Token::new(TokenKind::Keyword(Keyword::Import), Span::new(131, 136)),
      Token::new(TokenKind::Keyword(Keyword::In), Span::new(138, 139)),
      Token::new(TokenKind::Keyword(Keyword::InstanceOf), Span::new(141, 150)),
      Token::new(TokenKind::Keyword(Keyword::Interface), Span::new(152, 160)),
      Token::new(TokenKind::Keyword(Keyword::Let), Span::new(162, 164)),
      Token::new(TokenKind::Keyword(Keyword::New), Span::new(166, 168)),
      Token::new(TokenKind::Keyword(Keyword::Package), Span::new(170, 176)),
      Token::new(TokenKind::Keyword(Keyword::Public), Span::new(178, 183)),
      Token::new(TokenKind::Keyword(Keyword::Private), Span::new(185, 191)),
      Token::new(TokenKind::Keyword(Keyword::Protected), Span::new(193, 201)),
      Token::new(TokenKind::Keyword(Keyword::Return), Span::new(203, 208)),
      Token::new(TokenKind::Keyword(Keyword::Static), Span::new(210, 215)),
      Token::new(TokenKind::Keyword(Keyword::Super), Span::new(217, 221)),
      Token::new(TokenKind::Keyword(Keyword::Switch), Span::new(223, 228)),
      Token::new(TokenKind::Keyword(Keyword::This), Span::new(230, 233)),
      Token::new(TokenKind::Keyword(Keyword::Throw), Span::new(235, 239)),
      Token::new(TokenKind::Keyword(Keyword::Try), Span::new(241, 243)),
      Token::new(TokenKind::Keyword(Keyword::TypeOf), Span::new(245, 250)),
      Token::new(TokenKind::Keyword(Keyword::Var), Span::new(252, 254)),
      Token::new(TokenKind::Keyword(Keyword::Void), Span::new(256, 259)),
      Token::new(TokenKind::Keyword(Keyword::While), Span::new(261, 265)),
      Token::new(TokenKind::Keyword(Keyword::With), Span::new(267, 270)),
      Token::new(TokenKind::Keyword(Keyword::Yield), Span::new(272, 276)),
    ];

    let mut lexer = Lexer::new(input);
    lexer.process_input();

    assert_eq!(*lexer.get_tokens(), expected);
  }
}
