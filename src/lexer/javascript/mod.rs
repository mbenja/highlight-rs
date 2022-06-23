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
  Number,
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
    match read_next_token(lexer) {
      Some(token) => lexer.tokens.push(token),
      None => progress_to_whitespace(lexer),
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
  } else if next_char.is_digit(10) {
    return read_numeric_literal(lexer);
  } else if next_char == &'-' {
    let second_char = lexer.input.get(lexer.current_position + 1)?;

    if second_char.is_digit(10) {
      return read_numeric_literal(lexer);
    }
  } else if next_char == &'/' {
    let second_char = lexer.input.get(lexer.current_position + 1)?;

    if second_char == &'*' {
      return Some(read_multi_line_comment(lexer));
    } else {
      return Some(read_single_line_comment(lexer));
    }
  } else if next_char == &'"' || next_char == &'\'' || next_char == &'`' {
    return read_string_literal(lexer);
  } else if next_char == &'[' {
    lexer.current_position += 1;
    return read_next_token(lexer);
  } else {
    lexer.current_position += 1;
  }

  None
}

fn progress_to_whitespace(lexer: &mut Lexer) {
  while let Some(first_char) = lexer.input.get(lexer.current_position) {
    if !first_char.is_whitespace() {
      lexer.current_position += 1;
      continue;
    } else {
      break;
    }
  }
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

fn read_numeric_literal(lexer: &mut Lexer) -> Option<Token> {
  const PERMISSABLE_NON_NUMERIC_CHARS: [char; 10] =
    ['n', 'e', 'b', '-', '+', 'o', 'O', 'x', 'X', '.'];
  let start_position = lexer.current_position;
  let mut prev_char = *lexer.input.get(lexer.current_position)?;
  let mut radix = 10;
  let mut last_separator_position = 0;
  let mut is_float = false;

  while let Some(next_char) = lexer.input.get(lexer.current_position) {
    if next_char.is_whitespace() || next_char == &';' || next_char == &',' || next_char == &']' {
      break;
    }

    if (next_char == &'b' || next_char == &'B')
      && prev_char == '0'
      && lexer.current_position == start_position + 1
    {
      radix = 2;
    } else if (next_char == &'o' || next_char == &'O')
      && prev_char == '0'
      && lexer.current_position == start_position + 1
    {
      radix = 8;
    } else if (next_char == &'x' || next_char == &'X')
      && prev_char == '0'
      && lexer.current_position == start_position + 1
    {
      radix = 16;
    }

    if (next_char == &'-' || next_char == &'+')
      && (prev_char != 'e' || !lexer.input.get(lexer.current_position + 1)?.is_digit(radix))
    {
      return None; // Invalid exponential literal format
    } else if (next_char == &'b' || next_char == &'B') && prev_char != '0' && radix != 16 {
      return None; // Invalid placement of b character for binary literal
    } else if (next_char == &'o' || next_char == &'O') && prev_char != '0' {
      return None; // Invalid placement of o character for octal literal
    } else if (next_char == &'x' || next_char == &'X') && prev_char != '0' {
      return None; // Invalid placement of x character for hexadecimal literal
    } else if next_char == &'_' && prev_char == '_' {
      return None; // Double numeric separator is not allowed
    } else if next_char == &'_' && prev_char == '0' && lexer.current_position == start_position + 1
    {
      return None; // Numeric separator after leading 0 is not allowed
    } else if next_char == &'_' {
      if radix == 10
        && lexer.current_position != start_position + 1
        && lexer.current_position - 4 != last_separator_position
      {
        return None; // Invalid placement of numeric separator in base 10 literal
      } else if radix == 2
        && lexer.current_position != start_position + 6
        && lexer.current_position - 5 != last_separator_position
      {
        return None; // Invalid placement of numeric separator in binary literal
      } else if radix == 8
        && lexer.current_position != start_position + 3
        && lexer.current_position - 2 != last_separator_position
      {
        return None; // Invalid placement of numeric separator in octal literal
      } else if radix == 16
        && lexer.current_position != start_position + 4
        && lexer.current_position - 3 != last_separator_position
      {
        return None; // Invalid placement of numeric separator in hexadecimal literal
      }
    } else if prev_char == 'n' {
      return None; // Invalid placement of n character for BigInt literal
    } else if next_char == &'.' && is_float {
      return None; // Invalid float literal format
    } else if !next_char.is_digit(radix) && !PERMISSABLE_NON_NUMERIC_CHARS.contains(next_char) {
      return None; // Passes above checks but is badly formed numeric literal
    }

    if next_char == &'.' {
      is_float = true;
    }

    if next_char == &'_' {
      last_separator_position = lexer.current_position;
    }

    lexer.current_position += 1;
    prev_char = *next_char;
  }

  if prev_char == '.' {
    return None; // Invalid placement of decimal in float literal
  } else if prev_char == '_' {
    return None; // Ending numeric separator not allowed
  }

  Some(Token::new(
    TokenKind::JavaScript(JavaScriptTokenKind::Number),
    Span::new(start_position, lexer.current_position - 1),
  ))
}
