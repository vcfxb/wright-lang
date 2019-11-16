use codespan::{
    ByteIndex,
    Span,
    Files,
    FileId
};

pub mod ast;
pub mod parser;
pub mod scanner;

use ast::{
    Program
};

#[derive(Debug, Copy, Clone, Default)]
pub struct Properties {
    span: Span,
}

pub fn parse(src: &str) {
    let res = WrightParser::parse(WrightRule::PROGRAM, src);
    println!("{:#?}", res);
}

pub fn build_asts(files: Files, handles: Vec<FileId>) -> Program {
    unimplemented!()
}