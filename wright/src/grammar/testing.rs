use crate::grammar::ast::AstEq;
use crate::grammar::model::Fragment;
use crate::grammar::tracing::input::OptionallyTraceable;
use codespan::{FileId, Files};
use nom::IResult;

/// A testing context that holds test cases for wright parsers and
/// runs those tests via the exposed methods.
#[derive(Debug)]
pub struct TestingContext {
    files: Files<String>,
    handles: Vec<FileId>,
}

impl TestingContext {
    /// Construct a new testing context.
    pub fn new() -> Self {
        Self {
            files: Files::new(),
            handles: Vec::new(),
        }
    }

    /// Construct a testing context with the given test cases.
    pub fn with(sources: &[&str]) -> Self {
        let mut s = TestingContext::new();
        s.add_sources(sources);
        s
    }

    /// Add a piece of code to this test case.
    pub fn add_source(&mut self, code: &str) {
        let h = self.files.add(self.handles.len().to_string(), code.into());
        self.handles.push(h);
    }

    /// Add multiple sources to this object at once.
    pub fn add_sources(&mut self, sources: &[&str]) {
        sources.iter().for_each(|s| self.add_source(s))
    }

    /// Get the fragment of a given file by index. Panics if index is out of bounds.
    pub fn get_fragment<'a>(&'a self, index: usize) -> Fragment<'a> {
        let mut f = Fragment::new(&self.files, self.handles[index]);
        f.enable_trace();
        f
    }

    /// Run a parser on a source in this object. return the output in the
    /// nom format.
    pub fn run_parser_on<'a, O>(
        &'a self,
        index: usize,
        parser: fn(Fragment<'a>) -> IResult<Fragment<'a>, O>,
    ) -> IResult<Fragment<'a>, O> {
        parser(self.get_fragment(index))
    }

    /// Run a given parser on all the test cases loaded into this TestContext.
    pub fn run_parser_on_all<'a, O>(
        &'a self,
        parser: fn(Fragment<'a>) -> IResult<Fragment<'a>, O>,
    ) -> Vec<IResult<Fragment<'a>, O>> {
        (0..self.handles.len())
            .map(|i| self.get_fragment(i))
            .map(|f| parser(f))
            .collect()
    }

    /// Parse two of the stored test cases and return the result of an
    /// AST equality test. Panic if parsing fails.
    pub fn ast_eq<'a, T: AstEq>(
        &'a self,
        parser: fn(Fragment<'a>) -> IResult<Fragment<'a>, T>,
        index: (usize, usize),
    ) -> bool {
        let n1 = self
            .run_parser_on(index.0, parser)
            .expect("failed to parse testcase 0")
            .1;
        let n2 = self
            .run_parser_on(index.1, parser)
            .expect("failed to parse testcase 1")
            .1;
        AstEq::ast_eq(&n1, &n2)
    }

    /// Test succeeds when the given parser fails for all test cases.
    ///
    /// ## Panics
    /// - if the given parser succeeds on any test case
    pub fn test_all_fail<'a, T>(&'a self, parser: fn(Fragment<'a>) -> IResult<Fragment<'a>, T>) {
        assert!(self
            .run_parser_on_all(parser)
            .iter()
            .enumerate()
            .map(|(ind, r)| {
                if !r.is_err() {
                    let source = self.get_fragment(ind).source();
                    println!(
                        "Parser succeeded on \"{}\" when it should have failed. (test case {})",
                        source, ind
                    );
                    r.as_ref().unwrap().0.get_trace().unwrap().print().unwrap();
                }
                r
            })
            .all(|r| r.is_err()));
    }

    /// Test that the parser succeeds for all stored test cases.
    ///
    /// ## Panics
    /// - if the parser fails for any of the test cases
    pub fn test_all_succeed<'a, N>(&'a self, parser: fn(Fragment<'a>) -> IResult<Fragment<'a>, N>) {
        assert!(self
            .run_parser_on_all(parser)
            .iter()
            .enumerate()
            .map(|(ind, r)| {
                if !r.is_ok() {
                    let source = self.get_fragment(ind).source();
                    println!("Parser failed on \"{}\". (test case {})", source, ind);
                    r.as_ref().unwrap().0.get_trace().unwrap().print().unwrap();
                }
                r
            })
            .all(|r| r.is_ok()));
    }

    /// Run a given test on the output of a given parser when applied to the
    /// specified input. The test case is specified using the index argument.
    ///
    /// This is notably different from [`test_output_node`](method.test_output_node.html)
    /// because the test function also has access to the Fragment representing
    /// the remaining (unparsed) source code.
    ///
    /// ## Panics
    /// - if the index is invalid
    /// - if the parser fails
    /// - if the validation function panics
    pub fn test_output<'a, N>(
        &'a self,
        parser: fn(Fragment<'a>) -> IResult<Fragment<'a>, N>,
        index: usize,
        validation: impl FnOnce((Fragment<'a>, N)),
    ) {
        validation(
            self.run_parser_on(index, parser)
                .map_err(|e| {
                    e.map_input(|f: Fragment| {
                        f.get_trace().unwrap().print().unwrap();
                        f
                    })
                })
                .expect("parser failed"),
        )
    }

    /// Run a given test on the output of a given parser when applied to the
    /// specified input. The test case is specified using the index argument.
    ///
    /// This is notably different from [`test_output`](method.test_output.html)
    /// because the test function only has to deal with the AST node produced
    /// by the parser.
    ///
    /// ## Panics
    /// - if the index is invalid
    /// - if the parser fails
    /// - if the validation function panics
    #[inline]
    pub fn test_output_node<'a, N>(
        &'a self,
        parser: fn(Fragment<'a>) -> IResult<Fragment<'a>, N>,
        index: usize,
        validation: impl FnOnce(N),
    ) {
        self.test_output(parser, index, |o| validation(o.1))
    }
}
