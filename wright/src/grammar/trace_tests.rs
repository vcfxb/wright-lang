use crate::grammar::tracing::TraceInfo;

#[test]
fn test_rfind() {
    let mut tracer = TraceInfo::new();
    tracer.start("a");
    tracer.end("a", true);
    assert_eq!(tracer.rfind(1), Some(0));
}

#[test]
fn test_basic() {
    let mut tracer = TraceInfo::new();
    tracer.start("a");
    tracer.start("b");
    tracer.end("b", false);
    tracer.start("c");
    tracer.end("c", true);
    tracer.end("a", true);
    tracer.print().unwrap();
    panic!();
}