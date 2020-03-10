use crate::grammar::ast::BinaryOp;

/// Operator associativity.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Associativity {Left, Right}

/// Information about an operator.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub struct OperatorInfo {
    pub id: BinaryOp,
    /// tag or string in source code.
    pub str: &'static str,
    /// precedence. Lower means it binds more powerfully.
    pub prec: usize,
    /// Left or right associativity.
    pub assoc: Associativity
}

impl OperatorInfo {
    fn new(id: BinaryOp, str: &'static str, prec: usize, assoc: Associativity) -> Self {
        Self {id, str, prec, assoc}
    }
}

impl BinaryOp {
    /// Get information on this binary operation.
    pub fn get_info(self) -> OperatorInfo {
        use BinaryOp::*;
        use Associativity::*;
        match self {
            OrOr => OperatorInfo::new(OrOr, "||", 0, Left),
            AndAnd => OperatorInfo::new(AndAnd, "&&", 1, Left),
            Or => OperatorInfo::new(Or, "|", 2, Left),
            Xor => OperatorInfo::new(Xor, "^", 3, Left),
            And => OperatorInfo::new(And, "&", 4, Left),
            EqEq => OperatorInfo::new(EqEq, "==", 5, Left),
            NotEq => OperatorInfo::new(NotEq, "!=", 5, Left),
            Le => OperatorInfo::new(Le, "<=", 5, Left),
            Ge => OperatorInfo::new(Ge, ">=", 5, Left),
            Lt => OperatorInfo::new(Lt, "<", 5, Left),
            Gt => OperatorInfo::new(Gt, ">", 5, Left),
            DotDot => OperatorInfo::new(DotDot, "..", 6, Left),
            Add => OperatorInfo::new(Add, "+", 7, Left),
            Sub => OperatorInfo::new(Sub, "-", 7, Left),
            Mul => OperatorInfo::new(Mul, "*", 8, Left),
            Mod => OperatorInfo::new(Mod, "%", 8, Left),
            Div => OperatorInfo::new(Div, "/", 8, Left),
        }
    }
}