pub mod parser;
pub mod compiler;
pub mod codegen;

fn main() {
    let path = "test.hl".to_string();

    let code = std::fs::read_to_string(&path).expect("Failed to read file");
    
    let ast = parser::parser::program(&code).expect("Failed to parse file");

    match compiler::Compiler::new(path).compile(true, ast) {
        Ok(()) => println!("Compiled successfully"),
        Err(e) => println!("Failed to compile: {}", e)
    }
}
