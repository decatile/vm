mod lexer;
mod parser;
mod vm;

fn main() {
    let expr = "+ (* 2 3) (/ 8 2))";
    let tokens = parser::parse(expr).unwrap();
    for token in &tokens {
        println!("{token:?}")
    }
    let ast = lexer::lex(tokens).unwrap();
    println!("{ast:?}")
}
