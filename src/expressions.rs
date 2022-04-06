//! # Expressions
//! Basic syntax for describing constraint programs in
//! the CLP library.
//! ## General description
//! A CLP program is constructed as a type tree from the expression enums described in this file.
//! To be interesting a program should have at least one free variable and no self contradictions.

/// The name of a symbol (variable or constant of some type).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    name: String,
}

impl Symbol {
    pub fn new(s: String) -> Symbol {
        Symbol { name: s }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Domain {
    Boolean(BooleanValueDomainExpression),
    Integer(IntegerNumberDomainExpression),
}
pub trait Sample {
    fn sample(&self) -> Option<AssignedValue>;
}

impl Sample for Domain {
    fn sample(&self) -> Option<AssignedValue> {
        match self {
            Domain::Boolean(dom) => dom.sample(),
            Domain::Integer(dom) => dom.sample(),
        }
    }
}
impl Sample for BooleanValueDomainExpression {
    fn sample(&self) -> Option<AssignedValue> {
        use BooleanValueDomainExpression::*;
        match self {
            Empty => None,
            Single(val) => Some(AssignedValue::Boolean(val.clone())),
            Universe => Some(AssignedValue::Boolean(BooleanValue::False)),
        }
    }
}
impl Sample for IntegerNumberDomainExpression {
    fn sample(&self) -> Option<AssignedValue> {
        use IntegerNumberDomainExpression::*;
        match self {
            Empty => None,
            Universe => Some(AssignedValue::Integer(IntegerNumber::Value(0))),
            _ => unimplemented!(),
        }
    }
}
trait Reduce {
    fn reduce(&self, value: AssignedValue) -> Vec<Box<Self>>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanValueDomainExpression {
    Universe,
    Empty,
    Single(BooleanValue),
}

/// The set of values currently supported in CLP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignedValue {
    Boolean(BooleanValue),
    Integer(IntegerNumber),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: Symbol,
    domain: Domain,
}

impl Variable {
    pub fn assignment(&self) -> Option<Assignment> {
        match self.domain.sample() {
            None => None,
            Some(value) => Some(Assignment {
                name: self.name.clone(),
                value,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    name: Symbol,
    value: AssignedValue,
}

/// The logic base type values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanValue {
    False,
    True,
}

pub trait FreeVariable {
    fn get_free(&self) -> Vec<Variable>;
}

/// # The boolean relations and named definitions available in CLP.
/// Boolean values and operations are the base for the CLP program
/// and all constraints are from some type to the BooleanExpression
/// type. All constraints are also considered to be in an implicit
/// conjugation.
#[derive(Debug, Clone)]
pub enum BooleanExpression {
    And(Box<BooleanExpression>, Box<BooleanExpression>),
    Or(Box<BooleanExpression>, Box<BooleanExpression>),
    Implies(Box<BooleanExpression>, Box<BooleanExpression>),
    Equals(Box<BooleanExpression>, Box<BooleanExpression>),
    Parenthesis(Box<BooleanExpression>),
    Not(Box<BooleanExpression>),
    BooleanVariable(Symbol),
    BooleanValue(BooleanValue),
}

impl FreeVariable for BooleanExpression {
    fn get_free(&self) -> Vec<Variable> {
        use BooleanExpression::*;
        use BooleanValueDomainExpression::Universe;
        let mut free_variables: Vec<Variable> = Vec::new();
        match self {
            BooleanVariable(symbol) => free_variables.push(Variable {
                name: symbol.clone(),
                domain: Domain::Boolean(Universe),
            }),
            Not(expr) => free_variables.extend(expr.get_free()),
            Parenthesis(expr) => free_variables.extend(expr.get_free()),
            And(expr_a, expr_b) => {
                free_variables.extend(expr_a.get_free());
                free_variables.extend(expr_b.get_free());
            }
            Or(expr_a, expr_b) => {
                free_variables.extend(expr_a.get_free());
                free_variables.extend(expr_b.get_free());
            }
            Implies(expr_a, expr_b) => {
                free_variables.extend(expr_a.get_free());
                free_variables.extend(expr_b.get_free());
            }
            Equals(expr_a, expr_b) => {
                free_variables.extend(expr_a.get_free());
                free_variables.extend(expr_b.get_free());
            }
            BooleanValue(_) => (),
        }
        free_variables
    }
}

/// The possible values for integer numbers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerNumber {
    NaN,
    Value(i128),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerNumberExpression {
    IntegerNumberVariable(Symbol),
    IntegerNumberValue(IntegerNumber),
    Parenthesis(Box<IntegerNumberExpression>),
    Negate(Box<IntegerNumberExpression>),
    Add(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Minus(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Times(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Divide(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Modulo(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
}

impl FreeVariable for IntegerNumberExpression {
    fn get_free(&self) -> Vec<Variable> {
        use IntegerNumberExpression::*;
        let mut free: Vec<Variable> = Vec::new();
        match self {
            IntegerNumberValue(_) => (),
            IntegerNumberVariable(symbol) => free.push(Variable {
                name: symbol.clone(),
                domain: Domain::Integer(IntegerNumberDomainExpression::Universe),
            }),
            Parenthesis(expr) => free.extend(expr.get_free()),
            Negate(expr) => free.extend(expr.get_free()),
            Add(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Minus(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Times(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Divide(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Modulo(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
        }

        free
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerNumberDomainExpression {
    Universe,
    Empty,
    ClosedRange(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    OpenRange(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    OpenLeftClosedRightRange(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    ClosedLeftOpenRightRange(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    ExplicitSet(Vec<IntegerNumberExpression>),
    Union(
        Box<IntegerNumberDomainExpression>,
        Box<IntegerNumberDomainExpression>,
    ),
    Intersection(
        Box<IntegerNumberDomainExpression>,
        Box<IntegerNumberDomainExpression>,
    ),
    Difference(
        Box<IntegerNumberDomainExpression>,
        Box<IntegerNumberDomainExpression>,
    ),
    Complement(Box<IntegerNumberDomainExpression>),
}

impl FreeVariable for IntegerNumberDomainExpression {
    fn get_free(&self) -> Vec<Variable> {
        use IntegerNumberDomainExpression::*;
        let mut free: Vec<Variable> = Vec::new();

        match self {
            Universe => (),
            Empty => (),
            ClosedRange(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            OpenRange(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            OpenLeftClosedRightRange(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            ClosedLeftOpenRightRange(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            ExplicitSet(expr) => free.extend(expr.get_free()),
            Union(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Intersection(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Difference(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Complement(expr) => free.extend(expr.get_free()),
        }

        free
    }
}
impl FreeVariable for Vec<IntegerNumberExpression> {
    fn get_free(&self) -> Vec<Variable> {
        let mut free: Vec<Variable> = Vec::new();
        for elt in self {
            free.extend(elt.get_free());
        }

        free
    }
}
#[derive(Debug, Clone)]
pub enum BooleanIntegerNumberExpression {
    Equals(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Different(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Greater(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Less(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    In(
        Box<IntegerNumberExpression>,
        Box<IntegerNumberDomainExpression>,
    ),
}
impl FreeVariable for BooleanIntegerNumberExpression {
    fn get_free(&self) -> Vec<Variable> {
        use BooleanIntegerNumberExpression::*;
        let mut free: Vec<Variable> = Vec::new();
        match self {
            Equals(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Different(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Greater(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            Less(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
            In(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.get_free());
            }
        }
        free
    }
}

#[derive(Debug, Clone)]
pub enum ConstraintLogicExpression {
    Boolean(Box<BooleanExpression>),
    OfIntegerNumber(Box<BooleanIntegerNumberExpression>),
}
impl FreeVariable for ConstraintLogicExpression {
    fn get_free(&self) -> Vec<Variable> {
        use ConstraintLogicExpression::*;
        let mut free: Vec<Variable> = Vec::new();
        match self {
            Boolean(expr) => free.extend(expr.get_free()),
            OfIntegerNumber(expr) => free.extend(expr.get_free()),
        }
        free
    }
}
#[derive(Debug, Clone)]
pub enum SatisfactionExpression {
    Satisfy(Box<ConstraintLogicExpression>),
    Minimise(Box<ConstraintLogicExpression>),
    Maximise(Box<ConstraintLogicExpression>),
}
impl FreeVariable for SatisfactionExpression {
    fn get_free(&self) -> Vec<Variable> {
        use SatisfactionExpression::*;
        let mut free: Vec<Variable> = Vec::new();
        match self {
            Satisfy(expr) => free.extend(expr.get_free()),
            Minimise(expr) => free.extend(expr.get_free()),
            Maximise(expr) => free.extend(expr.get_free()),
        }
        free
    }
}
#[derive(Debug, Clone)]
pub enum ConstraintProgramExpression {
    Solve(Box<SatisfactionExpression>),
    SolveAnd(
        Box<SatisfactionExpression>,
        Box<ConstraintProgramExpression>,
    ),
    ConstrainAnd(
        Box<ConstraintLogicExpression>,
        Box<ConstraintProgramExpression>,
    ),
}
impl FreeVariable for &ConstraintProgramExpression {
    fn get_free(&self) -> Vec<Variable> {
        use ConstraintProgramExpression::*;
        let mut free: Vec<Variable> = Vec::new();
        match self {
            Solve(expr) => free.extend(expr.get_free()),
            SolveAnd(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.as_ref().get_free());
            }
            ConstrainAnd(expr_a, expr_b) => {
                free.extend(expr_a.get_free());
                free.extend(expr_b.as_ref().get_free());
            }
        }
        free
    }
}
