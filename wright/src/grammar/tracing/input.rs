use crate::grammar::tracing::TraceInfo;

// FIXME: link
/// Parser input represents any type that can be passed as an input to a
/// wright parser. The one used internally for formally parsing wright source code
/// is the [Fragment type](), however this trait is also implemented for other nom
/// parser input types, namely `&str` and `&[u8]`.
pub trait OptionallyTraceable: Clone {
    /// If tracing is available, record a start function label to the
    /// trace history.
    fn trace_start(&mut self, tag: &'static str);

    /// If tracing is available, record a function end label to
    /// the trace history.
    fn trace_end(&mut self, tag: &'static str, success: bool);


    /// Get a clone of this input's trace if available. Note that this may
    /// be entirely unavailable for some types (such as `&str`).
    fn get_trace(&self) -> Option<TraceInfo>;

    // FIXME link
    /// Similar to [`trace_start`](#method.trace_start) except it clones
    /// the input and returns it, instead of modifying it.
    fn trace_start_clone(&self, tag: &'static str) -> Self {
        let mut clone = self.clone();
        clone.trace_start(tag);
        clone
    }

    // FIXME: check link
    /// Similar to [`trace_end`](#trace_end) except it clones
    /// the input rather than mutating it.
    fn trace_end_clone(&self, tag: &'static str, success: bool) -> Self {
        let mut clone = self.clone();
        clone.trace_end(tag, success);
        clone
    }
}

/// Strings (`&str`) do not trace input. As such all of these functions are no
/// operation on strings.
impl<'a> OptionallyTraceable for &'a str {
    fn trace_start(&mut self, _: &'static str) {}

    fn trace_end(&mut self, _: &'static str, _: bool) {}

    /// Always returns `None`.
    fn get_trace(&self) -> Option<TraceInfo> { None }
}

/// Byte arrays (`&[u8]`) do not trace input. As such all of these
/// functions are no operation.
impl<'a> OptionallyTraceable for &'a[u8] {
    fn trace_start(&mut self, _: &'static str) {}

    fn trace_end(&mut self, _: &'static str, _: bool) {}

    /// Always returns `None`.
    fn get_trace(&self) -> Option<TraceInfo> { None }
}