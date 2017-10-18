extern crate regex;

use self::regex::Regex;

pub struct Parser {
    rules: Vec<ParseRule>
}

struct ParseRule {
    name: String,
    rule: Regex,
}

impl ParseRule {
    fn new(arg_name: String, arg_rule: Regex) -> ParseRule {
        ParseRule { name: arg_name, rule: arg_rule}
    }
}

#[derive(Debug, Clone)]
pub struct Type(String);

#[derive(Debug, Clone)]
pub enum BinaryOpTypes {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    // modify
    Increment,
    Decrement,
    // compare
    IsEq,
    IsNotEq,
    IsGE,
    IsLE,
    Greater,
    Less,
    // boolean
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
pub enum UnaryOpTypes {
    Negation,
    // boolean
    Not,
}


#[derive(Debug, Clone)]
pub enum Expr {
    Literal(String),
    Id(String),
    BinaryOp(BinaryOpTypes, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOpTypes, Box<Expr>),
    FunctionCall(Box<Expr>, Box<Vec<Expr>>),
}

#[derive(Debug, Clone)]
pub enum Stmnt{
    VarDec(Expr, Type),
    Assign(Expr, Expr),
    // todo
    //FnDec
    //ClassDec
}

impl Expr {

}

impl Parser {
    pub fn new() -> Parser {
        let id = r"([[[:lower:]]_]*)";
        let class = r"([[:upper:]][[:alpha:]]+)";

        // testing
        println!("{:?} == true", Regex::new(id).unwrap().is_match("Hi"));
        return Parser{rules: vec![]};
    }
}