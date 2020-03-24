
/// Trait to check if two Syntax trees are equal to each other. Ignores Fragments/Locations in
/// source code.
pub trait ASTEq {
    /// Check if the Abstract syntax trees are equal to each other. This doesn't necessarily mean
    /// that the values are equal. Only that the tree produced by parsing is.
    fn ast_eq(fst: &Self, snd: &Self) -> bool;
}

impl<T> ASTEq for Option<T> where T: ASTEq {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        match (fst, snd) {
            (Some(a), Some(b)) => ASTEq::ast_eq(a,b),
            (None, None) => true,
            _ => false,
        }
    }
}


impl<T> ASTEq for Box<T> where T: ASTEq {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        ASTEq::ast_eq(&*fst, &*snd)
    }
}

impl<T> ASTEq for &T where T: ASTEq {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {ASTEq::ast_eq(*fst, *snd)}
}

impl<T> ASTEq for Vec<T> where T: ASTEq {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.len() == snd.len() &&
        fst.iter()
            .zip(snd.iter())
            .all(|(a,b)| ASTEq::ast_eq(a,b))
    }
}