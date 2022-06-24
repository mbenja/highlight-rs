use crate::lexer::{
  javascript::JavaScriptTokenKind,
  language::Language,
  token::{Token, TokenKind},
};

pub fn build_html(raw_input: &str, tokens: &Vec<Token>, language: &Language) -> String {
  let mut html = format!("<pre>{}</pre>", raw_input);
  let mut insertion_offset: usize = 5;

  for token in tokens.iter() {
    let (token_start, token_end) = token.get_span().get_bounds();

    let token_color = match language {
      Language::JavaScript => resolve_javascript_token_color(token.get_kind()),
    };

    let opening_span_tag = format!("<span style='color:{}'>", token_color);

    html.insert_str(token_start + insertion_offset, &opening_span_tag);
    insertion_offset += opening_span_tag.len();
    html.insert_str(token_end + insertion_offset + 1, "</span>");
    insertion_offset += 7;
  }

  html
}

fn resolve_javascript_token_color(token_kind: &TokenKind) -> &'static str {
  match token_kind {
    TokenKind::JavaScript(token_kind) => match token_kind {
      JavaScriptTokenKind::Boolean => "#6639ba",
      JavaScriptTokenKind::Comment => "#6e7781",
      JavaScriptTokenKind::Identifier => "#d4a72c",
      JavaScriptTokenKind::Keyword => "#033d8b",
      JavaScriptTokenKind::Null => "#fa4549",
      JavaScriptTokenKind::Number => "#a475f9",
      JavaScriptTokenKind::String => "#2da44e",
    },
  }
}
