use crate::expressions::BooleanExpression;
use crate::expressions::BooleanIntegerNumberExpression;
use crate::expressions::BooleanValue;
use crate::expressions::ConstraintLogicExpression;
use crate::expressions::ConstraintProgramExpression;
use crate::expressions::IntegerNumber;
use crate::expressions::IntegerNumberDomainExpression;
use crate::expressions::IntegerNumberExpression;
use crate::expressions::SatisfactionExpression;
use crate::expressions::Symbol;
use crate::solver::apply;
use crate::solver::free_variables;
use crate::solver::generate_attempt;

#[cfg(test)]
extern crate quickcheck;

use quickcheck::Arbitrary;

impl Arbitrary for Symbol {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Symbol {
    fn some_name<G: quickcheck::Gen>(g: &mut G) -> String {
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
      match g.next_u32() % 3 {
        0 => format!("a_{}", g.next_u32() % 10),
        1 => format!("b_{}", g.next_u32() % 10),
        _ => format!(
          "{}_{}",
          names.choose(&mut rand::thread_rng()).unwrap(),
          names.choose(&mut rand::thread_rng()).unwrap()
        ),
      }
    }

    Symbol::new(some_name(g).to_owned())
  }
}

impl Arbitrary for BooleanValue {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> BooleanValue {
    if g.next_u32() % 2 == 0 {
      BooleanValue::False
    } else {
      BooleanValue::True
    }
  }
}

impl Arbitrary for BooleanExpression {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> BooleanExpression {
    match g.next_u32() % 16 {
      0 => BooleanExpression::And(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      1 => BooleanExpression::Or(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      2 => BooleanExpression::Implies(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      3 => BooleanExpression::Equals(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      4 => BooleanExpression::Parenthesis(Arbitrary::arbitrary(g)),
      5 => BooleanExpression::Not(Arbitrary::arbitrary(g)),
      6 => BooleanExpression::BooleanValue(Arbitrary::arbitrary(g)),
      _ => BooleanExpression::BooleanVariable(Arbitrary::arbitrary(g)),
    }
  }
}

impl Arbitrary for IntegerNumber {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> IntegerNumber {
    match g.next_u32() % 512 {
      0 => IntegerNumber::NaN,
      _ => IntegerNumber::Value(Arbitrary::arbitrary(g)),
    }
  }
}

impl Arbitrary for IntegerNumberExpression {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> IntegerNumberExpression {
    match g.next_u32() % 16 {
      0 => IntegerNumberExpression::IntegerNumberValue(Arbitrary::arbitrary(g)),
      1 => IntegerNumberExpression::Parenthesis(Arbitrary::arbitrary(g)),
      2 => IntegerNumberExpression::Negate(Arbitrary::arbitrary(g)),
      3 => IntegerNumberExpression::Add(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      4 => IntegerNumberExpression::Minus(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      5 => IntegerNumberExpression::Times(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      6 => IntegerNumberExpression::Divide(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      7 => IntegerNumberExpression::Modulo(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      _ => IntegerNumberExpression::IntegerNumberVariable(Arbitrary::arbitrary(g)),
    }
  }
}

impl Arbitrary for IntegerNumberDomainExpression {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> IntegerNumberDomainExpression {
    match g.next_u32() % 32 {
      0 => IntegerNumberDomainExpression::Empty,
      1 => {
        IntegerNumberDomainExpression::ClosedRange(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
      }
      2 => {
        IntegerNumberDomainExpression::OpenRange(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
      }
      3 => IntegerNumberDomainExpression::OpenLeftClosedRightRange(
        Arbitrary::arbitrary(g),
        Arbitrary::arbitrary(g),
      ),
      4 => IntegerNumberDomainExpression::ClosedLeftOpenRightRange(
        Arbitrary::arbitrary(g),
        Arbitrary::arbitrary(g),
      ),
      5 => IntegerNumberDomainExpression::ExplicitSet(Arbitrary::arbitrary(g)),
      6 => IntegerNumberDomainExpression::Union(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      7 => IntegerNumberDomainExpression::Intersection(
        Arbitrary::arbitrary(g),
        Arbitrary::arbitrary(g),
      ),
      8 => {
        IntegerNumberDomainExpression::Difference(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
      }
      9 => IntegerNumberDomainExpression::Complement(Arbitrary::arbitrary(g)),
      _ => IntegerNumberDomainExpression::Universe,
    }
  }
}

impl Arbitrary for BooleanIntegerNumberExpression {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> BooleanIntegerNumberExpression {
    match g.next_u32() % 5 {
      0 => BooleanIntegerNumberExpression::Equals(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      1 => {
        BooleanIntegerNumberExpression::Different(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
      }
      2 => {
        BooleanIntegerNumberExpression::Greater(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
      }
      3 => BooleanIntegerNumberExpression::Less(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      4 => BooleanIntegerNumberExpression::In(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      _ => unreachable!(),
    }
  }
}

impl Arbitrary for ConstraintLogicExpression {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> ConstraintLogicExpression {
    match g.next_u32() % 2 {
      0 => ConstraintLogicExpression::Boolean(Arbitrary::arbitrary(g)),
      1 => ConstraintLogicExpression::OfIntegerNumber(Arbitrary::arbitrary(g)),
      _ => unreachable!(),
    }
  }
}

impl Arbitrary for SatisfactionExpression {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> SatisfactionExpression {
    match g.next_u32() % 3 {
      0 => SatisfactionExpression::Satisfy(Arbitrary::arbitrary(g)),
      1 => SatisfactionExpression::Maximise(Arbitrary::arbitrary(g)),
      2 => SatisfactionExpression::Minimise(Arbitrary::arbitrary(g)),
      _ => unreachable!(),
    }
  }
}
impl Arbitrary for ConstraintProgramExpression {
  fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> ConstraintProgramExpression {
    let branch: u32 = g.next_u32();
    match branch % 5 {
      0 => ConstraintProgramExpression::Solve(Arbitrary::arbitrary(g)),
      1 => ConstraintProgramExpression::SolveAnd(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
      _ => {
        ConstraintProgramExpression::ConstrainAnd(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
      }
    }
  }
}

#[quickcheck]
fn a_solution_covers_all_free_variables(p: ConstraintProgramExpression) -> bool {
  let free = free_variables(&p);
  let attempt = generate_attempt(free);
  let update_program = apply(p, attempt);
  let free_after_apply = free_variables(&update_program);

  free_after_apply.is_empty()
}
