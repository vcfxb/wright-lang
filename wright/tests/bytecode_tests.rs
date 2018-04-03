extern crate wright;
use wright::target::jvm::bytecode::OpCode;

#[test]
fn test_bytecode_nop() {
    let nop = OpCode::nop;
    let byte = nop.to_byte();
    assert_eq!(OpCode::from_byte(byte), nop);
}

#[test]
fn test_all_bytecodes() {
    for i in 0..::std::u8::MAX {
        let op = OpCode::from_byte(i);
        assert_eq!(op.to_byte(), i);
    }
}