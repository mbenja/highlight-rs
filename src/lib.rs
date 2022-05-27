use lexer::Lexer;

pub mod lexer;

pub fn highlight(input: String) -> String {
  let mut lexer = Lexer::new(input);
  lexer.process_input();

  String::from("")
}
