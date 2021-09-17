mod ast0;
mod ast1;
mod codegen;
mod parse;

use codegen::compile;
use parse::parse;

pub fn run(source: &str) {
    let (remaining, ast) = parse(source).unwrap();
    if remaining.is_empty() {
        let ast: ast1::Ast = ast.into();
        dbg!(&ast);
        println!("{}", compile(ast));
    } else {
        eprintln!(
            "unexpected input:\n{}\nast:\n{:?}",
            remaining, ast
        );
    }
}
