/// The logic base type values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanValue {
    False,
    True,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanValueDomainExpression {
    Universe,
    Empty,
    Single(BooleanValue),
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
    BooleanVariable(super::Symbol),
    BooleanValue(BooleanValue),
}

impl super::FreeVariable for BooleanExpression {
    fn get_free(&self) -> Vec<super::Variable> {
        use super::Variable;
        use BooleanExpression::*;
        use BooleanValueDomainExpression::Universe;
        let mut free_variables: Vec<Variable> = Vec::new();
        match self {
            BooleanVariable(symbol) => free_variables.push(Variable {
                name: symbol.clone(),
                domain: super::Domain::Boolean(Universe),
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

impl super::Sample for BooleanValueDomainExpression {
    fn sample(&self) -> Option<super::AssignedValue> {
        use BooleanValueDomainExpression::*;
        match self {
            Empty => None,
            Single(val) => Some(super::AssignedValue::Boolean(val.clone())),
            Universe => Some(super::AssignedValue::Boolean(BooleanValue::False)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BooleanExpression, BooleanValue};
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for BooleanValue {
        fn arbitrary(g: &mut Gen) -> BooleanValue {
            if bool::arbitrary(g) {
                BooleanValue::False
            } else {
                BooleanValue::True
            }
        }
    }

    impl Arbitrary for BooleanExpression {
        fn arbitrary(g: &mut Gen) -> BooleanExpression {
            match u32::arbitrary(g) % 16 {
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
}
