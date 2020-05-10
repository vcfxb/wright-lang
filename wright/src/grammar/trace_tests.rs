use crate::grammar::tracing::TraceInfo;

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
}
