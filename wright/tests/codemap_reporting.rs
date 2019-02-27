use wright::codemap::codemap_report;
use codemap_report::Severity;
#[test]
fn test_ord() {assert!(Severity::CompilerBug > Severity::Error); }