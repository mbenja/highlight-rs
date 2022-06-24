use self::{javascript::lex_javascript, language::Language, token::Token};

pub mod javascript;
pub mod language;
pub mod span;
pub mod token;

pub struct Lexer {
  current_position: usize,
  input: Vec<char>,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(input: &str) -> Self {
    Lexer {
      current_position: 0,
      input: input.chars().collect(),
      tokens: vec![],
    }
  }

  pub fn get_tokens(&self) -> &Vec<Token> {
    &self.tokens
  }

  pub fn process_input(&mut self, language: Language) {
    match language {
      Language::JavaScript => lex_javascript(self),
    }
  }
}
