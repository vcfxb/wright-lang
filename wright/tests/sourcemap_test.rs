use wright;
use wright::codemap::*;

fn make_sample(v: bool) -> CodeMap {
    let mut s = CodeMap::new(v);
    s.add_virtual(
        "virt1",
        r#"
import virt2
import prelude
fn main: integer = {} -> (
    0;
    1;
    2;
    3->hp1 ;
    hp1<-3 ;
    3 -> hp1 == hp1 <- 3 ;
    true
)
"#
    );
    s.add_virtual(
        "virt2",
        r#"
fn helper: integer = integer -> self
fn hp1: integer = integer -> self+1
        "#
    );
    s.add_virtual("empty", r#""#);

    s
}

#[test]
fn test_make() {
    let c = make_sample(true);
    println!("{:?}", c.get_source(1u32)
        .unwrap()
        .upgrade()
        .unwrap()
    );
}

#[test]
fn test_loc() {
    let c = make_sample(false);
    assert!(c.get_source(0u32).is_none());
    c.get_source(1u32).unwrap();
    //print!("{:?}", c.get_file(1u32).unwrap().upgrade().unwrap().name);
    // todo
}

#[test]
fn test_line_num() {
    // todo
}