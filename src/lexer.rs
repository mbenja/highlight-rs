#[derive(Debug, PartialEq)]
pub struct Token {
  kind: TokenKind,
  span: Span,
}

impl Token {
  pub fn new(kind: TokenKind, span: Span) -> Token {
    Token { kind, span }
  }
}

#[derive(Debug, PartialEq)]
pub struct Span {
  start: usize,
  end: usize,
}

impl Span {
  pub fn new(start: usize, end: usize) -> Span {
    Span { start, end }
  }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  Keyword(Keyword),
}

impl From<Keyword> for TokenKind {
  fn from(keyword: Keyword) -> Self {
    TokenKind::Keyword(keyword)
  }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
  Await,
  Break,
  Case,
  Catch,
  Class,
  Const,
  Continue,
  Debugger,
  Default,
  Delete,
  Do,
  Else,
  Enum,
  Export,
  Extends,
  Finally,
  For,
  Function,
  If,
  Implements,
  Import,
  In,
  InstanceOf,
  Interface,
  Let,
  New,
  Package,
  Public,
  Private,
  Protected,
  Return,
  Static,
  Super,
  Switch,
  This,
  Throw,
  Try,
  TypeOf,
  Var,
  Void,
  While,
  With,
  Yield,
}

pub struct Lexer {
  current_position: usize,
  input: Vec<char>,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(input: String) -> Lexer {
    Lexer {
      current_position: 0,
      input: input.chars().collect(),
      tokens: vec![],
    }
  }

  pub fn get_tokens(&self) -> &Vec<Token> {
    &self.tokens
  }

  pub fn process_input(&mut self) {
    while self.current_position < self.input.len() {
      self.process_token();
    }
  }

  fn process_token(&mut self) {
    let (next_token, span) = self.read_next_token();

    let token = match next_token.as_str() {
      "await" => Token {
        kind: TokenKind::from(Keyword::Await),
        span,
      },
      "break" => Token {
        kind: TokenKind::from(Keyword::Break),
        span,
      },
      "case" => Token {
        kind: TokenKind::from(Keyword::Case),
        span,
      },
      "catch" => Token {
        kind: TokenKind::from(Keyword::Catch),
        span,
      },
      "class" => Token {
        kind: TokenKind::from(Keyword::Class),
        span,
      },
      "const" => Token {
        kind: TokenKind::from(Keyword::Const),
        span,
      },
      "continue" => Token {
        kind: TokenKind::from(Keyword::Continue),
        span,
      },
      "debugger" => Token {
        kind: TokenKind::from(Keyword::Debugger),
        span,
      },
      "default" => Token {
        kind: TokenKind::from(Keyword::Default),
        span,
      },
      "delete" => Token {
        kind: TokenKind::from(Keyword::Delete),
        span,
      },
      "do" => Token {
        kind: TokenKind::from(Keyword::Do),
        span,
      },
      "else" => Token {
        kind: TokenKind::from(Keyword::Else),
        span,
      },
      "enum" => Token {
        kind: TokenKind::from(Keyword::Enum),
        span,
      },
      "export" => Token {
        kind: TokenKind::from(Keyword::Export),
        span,
      },
      "extends" => Token {
        kind: TokenKind::from(Keyword::Extends),
        span,
      },
      "finally" => Token {
        kind: TokenKind::from(Keyword::Finally),
        span,
      },
      "for" => Token {
        kind: TokenKind::from(Keyword::For),
        span,
      },
      "function" => Token {
        kind: TokenKind::from(Keyword::Function),
        span,
      },
      "if" => Token {
        kind: TokenKind::from(Keyword::If),
        span,
      },
      "implements" => Token {
        kind: TokenKind::from(Keyword::Implements),
        span,
      },
      "import" => Token {
        kind: TokenKind::from(Keyword::Import),
        span,
      },
      "in" => Token {
        kind: TokenKind::from(Keyword::In),
        span,
      },
      "instanceof" => Token {
        kind: TokenKind::from(Keyword::InstanceOf),
        span,
      },
      "interface" => Token {
        kind: TokenKind::from(Keyword::Interface),
        span,
      },
      "let" => Token {
        kind: TokenKind::from(Keyword::Let),
        span,
      },
      "new" => Token {
        kind: TokenKind::from(Keyword::New),
        span,
      },
      "package" => Token {
        kind: TokenKind::from(Keyword::Package),
        span,
      },
      "public" => Token {
        kind: TokenKind::from(Keyword::Public),
        span,
      },
      "private" => Token {
        kind: TokenKind::from(Keyword::Private),
        span,
      },
      "protected" => Token {
        kind: TokenKind::from(Keyword::Protected),
        span,
      },
      "return" => Token {
        kind: TokenKind::from(Keyword::Return),
        span,
      },
      "static" => Token {
        kind: TokenKind::from(Keyword::Static),
        span,
      },
      "super" => Token {
        kind: TokenKind::from(Keyword::Super),
        span,
      },
      "switch" => Token {
        kind: TokenKind::from(Keyword::Switch),
        span,
      },
      "this" => Token {
        kind: TokenKind::from(Keyword::This),
        span,
      },
      "throw" => Token {
        kind: TokenKind::from(Keyword::Throw),
        span,
      },
      "try" => Token {
        kind: TokenKind::from(Keyword::Try),
        span,
      },
      "typeof" => Token {
        kind: TokenKind::from(Keyword::TypeOf),
        span,
      },
      "var" => Token {
        kind: TokenKind::from(Keyword::Var),
        span,
      },
      "void" => Token {
        kind: TokenKind::from(Keyword::Void),
        span,
      },
      "while" => Token {
        kind: TokenKind::from(Keyword::While),
        span,
      },
      "with" => Token {
        kind: TokenKind::from(Keyword::With),
        span,
      },
      "yield" => Token {
        kind: TokenKind::from(Keyword::Yield),
        span,
      },
      s => panic!("Unexpected token: {:?}", String::from(s)),
    };

    self.tokens.push(token);
  }

  fn read_next_token(&mut self) -> (String, Span) {
    let mut next_token = String::from("");
    let mut start_index = 0;

    while let Some(first_char) = self.input.get(self.current_position) {
      if first_char.is_whitespace() {
        self.current_position += 1;
        continue;
      }

      next_token.push(*first_char);
      start_index = self.current_position;
      self.current_position += 1;
      break;
    }

    while let Some(next_char) = self.input.get(self.current_position) {
      if next_char.is_alphabetic() || (next_token.len() > 0 && next_char.is_alphanumeric()) {
        next_token.push(*next_char);
        self.current_position += 1;
      } else {
        break;
      }
    }

    (
      next_token,
      Span {
        start: start_index,
        end: self.current_position - 1,
      },
    )
  }
}
