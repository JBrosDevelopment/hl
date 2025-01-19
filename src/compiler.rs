use crate::parser;
use crate::codegen;
use std::fs;
use std::process::Command;


pub struct Compiler {
    pub file: String,
    pub asm_output_file: String,
    pub obj_output_file: String,
    pub exe_output_file: String
}

impl Compiler {
    pub fn new(file: String) -> Self {
        Self {
            asm_output_file: file.split('.').collect::<Vec<&str>>()[..1].join("").to_string() + ".asm",
            obj_output_file: file.split('.').collect::<Vec<&str>>()[..1].join("").to_string() + ".o",
            exe_output_file: file.split('.').collect::<Vec<&str>>()[..1].join("").to_string() + ".exe",
            file
        }
    }
    pub fn compile(&self, debugging: bool, ast: Vec<parser::Expr>) -> Result<(), String> {
        let mut out = "
; -- header --
bits 64
default rel
".to_string();

        out += "
;-- variables --
section .bss
read_number resq 1
";

        out += " 
; -- constants --
section .data
read_format db \"%d\", 0
";

        out += "
; -- entry point --
section .text
global main
extern ExitProcess
extern printf
extern scanf
";
        
        let mut pc = 0;
        while pc < ast.len() {
            let expr = &ast[pc];
            
            match expr {
                parser::Expr::Literal(litteral) => {
                    out += &codegen::literal(litteral);
                }
                parser::Expr::Identifier(name) => {
                    out += &codegen::identifier(name);
                }
                parser::Expr::Assign(name, e) => {
                    out += &codegen::assign_variable(name, e);
                }
                parser::Expr::Let(name, e) => {
                    out += &codegen::define_variable(name, e);
                }
                parser::Expr::Function(name, parameters, body) => {
                    out += &codegen::define_function(name, parameters, body);
                }
                parser::Expr::SubRoutine(name, parameters, body) => {
                    out += &codegen::define_subroutine(name, parameters, body);
                }
                parser::Expr::IfElse(condition, if_body, else_body) => {
                    out += &codegen::if_else(condition, if_body, else_body);
                }
                parser::Expr::WhileLoop(condition, body) => {
                    out += &codegen::while_loop(condition, body);
                }
                parser::Expr::Call(name, args) => {
                    out += &codegen::call_function(name, args);
                }
                parser::Expr::Eq(l, r) |
                parser::Expr::Ne(l, r) |
                parser::Expr::Lt(l, r) |
                parser::Expr::Le(l, r) |
                parser::Expr::Gt(l, r) |
                parser::Expr::Ge(l, r) |
                parser::Expr::Add(l, r) |
                parser::Expr::Sub(l, r) |
                parser::Expr::Mul(l, r) |
                parser::Expr::Div(l, r) => {
                    out += &codegen::binary_op(l, r);
                }
            }

            pc += 1;
        }

        out += "
EXIT_LABEL:
    mov rcx, 0
    call ExitProcess
";

        if debugging {
            fs::write(&self.asm_output_file, out).expect("Failed to write assembly debugging file");
        }

        let nasm = Command::new("nasm")
            .args(["-f", "elf64", &self.asm_output_file])
            .output();

        match nasm {
            Ok(_) => {},
            Err(e) => return Err(e.to_string())
        }

        let gcc = Command::new("gcc")
            .args(["-o", &self.exe_output_file, &self.obj_output_file])
            .output();

        match gcc {
            Ok(_) => {},
            Err(e) => return Err(e.to_string())
        }

        if !debugging {
            fs::remove_file(&self.obj_output_file).expect("Failed to remove object file");
        }

        Ok(())
    }
}