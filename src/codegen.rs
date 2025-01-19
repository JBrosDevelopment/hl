use crate::parser;

pub fn define_subroutine(name: &String, parameters: &Vec<String>, body: &Vec<parser::Expr>) -> String {
    todo!()
}

pub fn define_function(name: &String, parameters: &Vec<String>, body: &Vec<parser::Expr>) -> String {
    todo!()
}

pub fn call_function(name: &String, arguments: &Vec<parser::Expr>) -> String {
    todo!()
}

pub fn define_variable(name: &String, value: &parser::Expr) -> String {
    todo!()
}

pub fn assign_variable(name: &String, value: &parser::Expr) -> String {
    todo!()
}

pub fn if_else(condition: &parser::Expr, then_body: &Vec<parser::Expr>, else_body: &Vec<parser::Expr>) -> String {
    todo!()
}

pub fn while_loop(condition: &parser::Expr, body: &Vec<parser::Expr>) -> String {
    todo!()
}

pub fn expression(expr: &parser::Expr) -> String {
    todo!()
}

pub fn literal(expr: &String) -> String {
    todo!()
}

pub fn identifier(expr: &String) -> String {
    todo!()
}

pub fn binary_op(l: &parser::Expr, r: &parser::Expr) -> String {
    todo!()
}