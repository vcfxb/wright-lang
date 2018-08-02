extern crate wright;
use wright::grammar::*;

#[test]
fn test_hello_world() {
    let prog = r#"
fn main() {println("Hello from Wright");}
    "#;
    println!("{:#?}", program(prog));
}
