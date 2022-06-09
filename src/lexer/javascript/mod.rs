use super::{
  span::Span,
  token::{Token, TokenKind},
  Lexer,
};

#[derive(Debug, PartialEq)]
pub enum JavaScriptTokenKind {
  Boolean,
  Comment,
  Identifier,
  Keyword,
  Null,
  String,
}

const KEYWORDS: [&str; 43] = [
  "await",
  "break",
  "case",
  "catch",
  "class",
  "const",
  "continue",
  "debugger",
  "default",
  "delete",
  "do",
  "else",
  "enum",
  "export",
  "extends",
  "finally",
  "for",
  "function",
  "if",
  "implements",
  "import",
  "in",
  "instanceof",
  "interface",
  "let",
  "new",
  "package",
  "public",
  "private",
  "protected",
  "return",
  "static",
  "super",
  "switch",
  "this",
  "throw",
  "try",
  "typeof",
  "var",
  "void",
  "while",
  "with",
  "yield",
];

pub fn lex_javascript(lexer: &mut Lexer) {
  while lexer.current_position < lexer.input.len() {
    if let Some(token) = read_next_token(lexer) {
      lexer.tokens.push(token);
    }
  }
}

fn read_next_token(lexer: &mut Lexer) -> Option<Token> {
  progress_to_non_whitespace(lexer);

  let next_char = lexer.input.get(lexer.current_position)?;

  if next_char.is_alphabetic() {
    let mut next_token = String::from("");
    let start_position = lexer.current_position;

    while let Some(next_char) = lexer.input.get(lexer.current_position) {
      if next_char.is_whitespace() || next_char == &';' {
        break;
      } else {
        next_token.push(*next_char);
        lexer.current_position += 1;
      }
    }

    let span = Span::new(start_position, lexer.current_position - 1);

    if next_token == "true" || next_token == "false" {
      return Some(Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Boolean),
        span,
      ));
    } else if next_token == "null" {
      return Some(Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Null),
        span,
      ));
    } else if KEYWORDS.contains(&next_token.as_str()) {
      return Some(Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Keyword),
        span,
      ));
    } else {
      return Some(Token::new(
        TokenKind::JavaScript(JavaScriptTokenKind::Identifier),
        span,
      ));
    }
  } else if next_char == &'/' {
    let second_char = *lexer.input.get(lexer.current_position + 1)?;

    if second_char == '*' {
      return Some(read_multi_line_comment(lexer));
    } else {
      return Some(read_single_line_comment(lexer));
    }
  } else if next_char == &'"' || next_char == &'\'' || next_char == &'`' {
    return read_string_literal(lexer);
  } else {
    lexer.current_position += 1;
  }

  None
}

fn progress_to_non_whitespace(lexer: &mut Lexer) {
  while let Some(first_char) = lexer.input.get(lexer.current_position) {
    if first_char.is_whitespace() {
      lexer.current_position += 1;
      continue;
    } else {
      break;
    }
  }
}

fn read_single_line_comment(lexer: &mut Lexer) -> Token {
  let start_position = lexer.current_position;

  while let Some(next_char) = lexer.input.get(lexer.current_position) {
    if next_char == &'\n' {
      break;
    } else {
      lexer.current_position += 1;
    }
  }

  Token::new(
    TokenKind::JavaScript(JavaScriptTokenKind::Comment),
    Span::new(start_position, lexer.current_position - 1),
  )
}

fn read_multi_line_comment(lexer: &mut Lexer) -> Token {
  let start_position = lexer.current_position;
  let mut prev_char = '/';

  while let Some(next_char) = lexer.input.get(lexer.current_position) {
    lexer.current_position += 1;

    if next_char == &'/' && prev_char == '*' {
      break;
    } else {
      prev_char = *next_char;
    }
  }

  Token::new(
    TokenKind::JavaScript(JavaScriptTokenKind::Comment),
    Span::new(start_position, lexer.current_position - 1),
  )
}

fn read_string_literal(lexer: &mut Lexer) -> Option<Token> {
  let start_position = lexer.current_position;
  let wrapping_character = lexer.input.get(lexer.current_position)?;
  lexer.current_position += 1;

  while let Some(next_char) = lexer.input.get(lexer.current_position) {
    lexer.current_position += 1;

    if next_char == wrapping_character {
      break;
    }
  }

  Some(Token::new(
    TokenKind::JavaScript(JavaScriptTokenKind::String),
    Span::new(start_position, lexer.current_position - 1),
  ))
}
