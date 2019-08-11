//! # CLP : A Constraint Logic Programming library
//! Automatically solve problems without thinking!
//! Amaze your friends! Confidently deal with the thing
//! that actually needs doing while dumping the core
//! work to some random guy with a keyboard.
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod expressions {
  //! # Expressions
  //! Basic syntax for describing constraint programs in
  //! the CLP library.
  //! ## General description
  //! A CLP program is constructed as a type tree from the expression enums described in this file.
  //! To be interesting a program should have atleast one free variable and no self contradictions.

  /// The name of a symbol (variable or constant of some type).
  #[derive(Debug, Clone)]
  pub struct Symbol {
    name: String,
  }

  impl Symbol {
    pub fn new(s: String) -> Symbol {
      Symbol { name: s }
    }
  }

  #[derive(Debug, Clone)]
  pub enum DomainValue {
    Boolean(BooleanValueDomainExpression),
    Integer(IntegerNumberDomainExpression),
  }

  #[derive(Debug, Clone)]
  pub enum BooleanValueDomainExpression {
    Universe,
    Empty,
    Single(BooleanValue),
  }

  /// The set of values currently supported in CLP.
  #[derive(Debug, Clone)]
  pub enum AssignedValue {
    Boolean(BooleanValue),
    Integer(IntegerNumber),
  }

  #[derive(Debug, Clone)]
  pub struct Variable {
    name: Symbol,
    domain: DomainValue,
  }

  /// The logic base type values.
  #[derive(Debug, Clone)]
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
          domain: DomainValue::Boolean(Universe),
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
  #[derive(Debug, Clone)]
  pub enum IntegerNumber {
    NaN,
    Value(i128),
  }

  #[derive(Debug, Clone)]
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
        IntegerNumberVariable(symb) => free.push(Variable {
          name: symb.clone(),
          domain: DomainValue::Integer(IntegerNumberDomainExpression::Universe),
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

  #[derive(Debug, Clone)]
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
}

pub mod solver {
  use crate::expressions::{AssignedValue, ConstraintProgramExpression, Symbol, Variable};

  /// Assigned value to a constant or variable in a solution.
  pub enum Solution {
    Unsatisfiable(Symbol, String),
    Variable(Symbol, AssignedValue),
    Constant(Symbol, AssignedValue),
  }
  pub fn generate_attempt(free: Vec<Variable>) -> Vec<Variable> {
    free
  }
  pub fn apply(
    program: ConstraintProgramExpression,
    _state: Vec<Variable>,
  ) -> ConstraintProgramExpression {
    program
  }
  pub fn reduce(program: ConstraintProgramExpression) -> ConstraintProgramExpression {
    program
  }

  pub fn free_variables(program: &ConstraintProgramExpression) -> Vec<Variable> {
    println!("{:?}", program);
    use crate::expressions::FreeVariable;
    program.get_free()
  }
  pub fn solve(_program: ConstraintProgramExpression) -> Vec<Solution> {
    Vec::new()
  }
}

#[cfg(test)]
mod tests;
