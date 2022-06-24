# highlight-rs

A syntax highlighter written in Rust. 

This project was just for fun and served as a good learning experience for the Rust programming language. `highlight-rs` is not a feature complete syntax highlighter. While the project is configured to support mulitple languages, right now it only supports (most of) JavaScript.

### Example Output
<p align="center">
 <img width="288" alt="highlight-rs-example-output" src="https://user-images.githubusercontent.com/23458996/175671243-746424fc-7963-429f-9728-31057a3b96db.png">
</p>



### Example Usage
```rust
use highlight_rs::{highlight, lexer::language::Language};

fn main() {
    let output = highlight("let myVar = 123;", Language::JavaScript);
}
```

### Benchmarks
Despite not being feature complete, `highlight-rs` is quite fast. The lexical analysis doesn't use regular expressions and instead reads each character of the input string individually, without back-tracking, to tokenize the input. This means that `highlight-rs` performs favorably against other solutions. Being written in Rust also helps!

Here is a comparison against a popular syntax highlighter written in JavaScript, [highlight.js](https://github.com/highlightjs/highlight.js). Each library was given a JavaScript file containing 1000 lines of code, below are the average results of 1000 iterations.

| highlight-rs | highlight.js |
|--------------|--------------|
|1.314719ms    |70.998099ms   |
