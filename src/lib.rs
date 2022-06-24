use html_builder::build_html;
use lexer::{language::Language, Lexer};

mod html_builder;
pub mod lexer;

pub fn highlight(input: &str, language: Language) -> String {
  let mut lexer = Lexer::new(&input);
  lexer.process_input(&language);

  build_html(&input, lexer.get_tokens(), &language)
}
