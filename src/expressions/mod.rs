//! # Expressions
//! Basic syntax for describing constraint programs in
//! the CLP library.
//! ## General description
//! A CLP program is constructed as a type tree from the expression enums described in this file.
//! To be interesting a program should have at least one free variable and no self contradictions.

pub mod boolean;
pub mod integer;

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
    Boolean(boolean::BooleanValueDomainExpression),
    Integer(integer::IntegerNumberDomainExpression),
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
trait Reduce {
    fn reduce(&self, value: AssignedValue) -> Vec<Box<Self>>;
}

/// The set of values currently supported in CLP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignedValue {
    Boolean(boolean::BooleanValue),
    Integer(integer::IntegerNumber),
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

pub trait FreeVariable {
    fn get_free(&self) -> Vec<Variable>;
}

#[derive(Debug, Clone)]
pub enum ConstraintLogicExpression {
    Boolean(Box<boolean::BooleanExpression>),
    OfIntegerNumber(Box<integer::BooleanIntegerNumberExpression>),
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

#[cfg(test)]
mod tests {

    use super::{
        ConstraintLogicExpression, ConstraintProgramExpression, SatisfactionExpression, Symbol,
    };
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Symbol {
        fn arbitrary(g: &mut Gen) -> Symbol {
            fn some_name(g: &mut Gen) -> String {
                use rand::seq::SliceRandom;
                let names = vec![
                    "corn",
                    "cob",
                    "cat",
                    "bunny",
                    "edge",
                    "lead",
                    "joke",
                    "elite",
                    "report",
                    "employee",
                    "tech",
                    "sun",
                    "candy",
                    "rain",
                    "clear",
                    "rest",
                    "organised",
                    "trauma",
                    "head",
                    "hand",
                    "foot",
                    "point",
                    "love",
                ];
                match u32::arbitrary(g) % 3 {
                    0 => format!("a_{}", u32::arbitrary(g) % 10),
                    1 => format!("b_{}", u32::arbitrary(g) % 10),
                    _ => format!(
                        "{}_{}",
                        names.choose(&mut rand::thread_rng()).unwrap(),
                        names.choose(&mut rand::thread_rng()).unwrap()
                    ),
                }
            }

            Symbol::new(some_name(g))
        }
    }

    impl Arbitrary for ConstraintLogicExpression {
        fn arbitrary(g: &mut Gen) -> ConstraintLogicExpression {
            match u32::arbitrary(g) % 2 {
                0 => ConstraintLogicExpression::Boolean(Arbitrary::arbitrary(g)),
                1 => ConstraintLogicExpression::OfIntegerNumber(Arbitrary::arbitrary(g)),
                _ => unreachable!(),
            }
        }
    }

    impl Arbitrary for SatisfactionExpression {
        fn arbitrary(g: &mut Gen) -> SatisfactionExpression {
            match u32::arbitrary(g) % 3 {
                0 => SatisfactionExpression::Satisfy(Arbitrary::arbitrary(g)),
                1 => SatisfactionExpression::Maximise(Arbitrary::arbitrary(g)),
                2 => SatisfactionExpression::Minimise(Arbitrary::arbitrary(g)),
                _ => unreachable!(),
            }
        }
    }
    impl Arbitrary for ConstraintProgramExpression {
        fn arbitrary(g: &mut Gen) -> ConstraintProgramExpression {
            match u32::arbitrary(g) % 5 {
                0 => ConstraintProgramExpression::Solve(Arbitrary::arbitrary(g)),
                1 => ConstraintProgramExpression::SolveAnd(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                _ => ConstraintProgramExpression::ConstrainAnd(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
            }
        }
    }
}
