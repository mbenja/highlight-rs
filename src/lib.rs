use lexer::{language::Language, Lexer};

pub mod lexer;

pub fn highlight(input: String) -> String {
  let mut lexer = Lexer::new(input);
  lexer.process_input(Language::JavaScript);

  String::from("")
}
