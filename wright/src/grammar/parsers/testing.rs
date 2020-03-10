use codespan::{FileId, Files};

pub fn setup(src: &'static str) -> (Files<String>, FileId) {
    let mut f: Files<String> = Files::new();
    let id = f.add("test", src.to_string());
    (f, id)
}
