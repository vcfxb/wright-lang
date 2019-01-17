extern crate wright;
use wright::grammar::*;

#[test]
fn test_hello_world() {
    let prog = r#"
use prelude
func main() {
    println("Hello from Wright");
}
    "#;
    println!("{:#?}", parse_program(prog));
}
