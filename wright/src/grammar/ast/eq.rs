/// Trait to check if two Syntax trees are equal to each other. Ignores Fragments/Locations in
/// source code.
pub trait AstEq {
    /// Check if the Abstract syntax trees are equal to each other. This doesn't necessarily mean
    /// that the values are equal. Only that the tree produced by parsing is.
    fn ast_eq(fst: &Self, snd: &Self) -> bool;
}

impl<T> AstEq for Option<T>
where
    T: AstEq,
{
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        match (fst, snd) {
            (Some(a), Some(b)) => AstEq::ast_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T> AstEq for Box<T>
where
    T: AstEq,
{
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&**fst, &**snd)
        // weird double dereference here, but it works.
        // *fst is Box<T>
        // **fst is T
        // &**fst is &T
        // ast_eq requires &T
    }
}

impl<T> AstEq for &T
where
    T: AstEq,
{
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(*fst, *snd)
    }
}

impl<T> AstEq for Vec<T>
where
    T: AstEq,
{
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.len() == snd.len() && fst.iter().zip(snd.iter()).all(|(a, b)| AstEq::ast_eq(a, b))
    }
}

// impl used by conditional blocks (which use tuples).
// probably should set up a macro for this or something.
impl<T, U> AstEq for (T, U)
where
    T: AstEq,
    U: AstEq,
{
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.0, &snd.0) && AstEq::ast_eq(&fst.1, &snd.1)
    }
}
