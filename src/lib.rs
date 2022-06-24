use html_builder::build_html;
use lexer::{language::Language, Lexer};

mod html_builder;
pub mod lexer;

pub fn highlight(input: String) -> String {
  let mut lexer = Lexer::new(&input);
  lexer.process_input(Language::JavaScript);

  build_html(&input, lexer.get_tokens(), Language::JavaScript)
}
