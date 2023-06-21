mod lexer;
mod ast;
mod repl;
mod token;
fn main() {
    let rep = repl::repl::Start();
}
