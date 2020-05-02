use crate::grammar::ast::BinaryOp;

/// Operator associativity.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Associativity {
    Left,
    Right,
}

/// Information about an operator.
#[derive(Copy, Clone, Debug)]
pub struct OperatorInfo {
    /// The binary operator.
    pub id: BinaryOp,
    /// precedence. Lower means it binds more powerfully.
    pub prec: usize,
    /// Left or right associativity.
    pub assoc: Associativity,
    /// Matching token in source code.
    pub token: &'static str,
}

impl OperatorInfo {
    /// Construct a new operator description.
    pub const fn new(id: BinaryOp, prec: usize, assoc: Associativity, token: &'static str) -> Self {
        Self {
            id,
            prec,
            assoc,
            token,
        }
    }
}

impl BinaryOp {
    /// Get information on this binary operation.
    pub fn get_info(self) -> OperatorInfo {
        use Associativity::*;
        use BinaryOp::*;
        let f1 = move || match self {
            OrOr => (0, Left, "||"),
            AndAnd => (1, Left, "&&"),
            Or => (2, Left, "|"),
            Xor => (3, Left, "^"),
            And => (4, Left, "&"),
            EqEq => (5, Left, "=="),
            NotEq => (5, Left, "!="),
            Le => (5, Left, "<="),
            Ge => (5, Left, ">="),
            Lt => (5, Left, "<"),
            Gt => (5, Left, ">"),
            DotDot => (6, Left, ".."),
            Add => (7, Left, "+"),
            Sub => (7, Left, "-"),
            Mul => (8, Left, "*"),
            Mod => (8, Left, "%"),
            Div => (8, Left, "/"),
        };
        let (prec, assoc, tok) = f1();
        OperatorInfo::new(self, prec, assoc, tok)
    }
}
