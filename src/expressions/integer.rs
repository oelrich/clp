/// The possible values for integer numbers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerNumber {
    NaN,
    Value(i128),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerNumberExpression {
    IntegerNumberVariable(super::Symbol),
    IntegerNumberValue(IntegerNumber),
    Parenthesis(Box<IntegerNumberExpression>),
    Negate(Box<IntegerNumberExpression>),
    Add(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Minus(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Times(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Divide(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
    Modulo(Box<IntegerNumberExpression>, Box<IntegerNumberExpression>),
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

impl super::FreeVariable for IntegerNumberExpression {
    fn get_free(&self) -> Vec<super::Variable> {
        use IntegerNumberExpression::*;
        let mut free: Vec<super::Variable> = Vec::new();
        match self {
            IntegerNumberValue(_) => (),
            IntegerNumberVariable(symbol) => free.push(super::Variable {
                name: symbol.clone(),
                domain: super::Domain::Integer(IntegerNumberDomainExpression::Universe),
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

impl super::FreeVariable for IntegerNumberDomainExpression {
    fn get_free(&self) -> Vec<super::Variable> {
        use IntegerNumberDomainExpression::*;
        let mut free: Vec<super::Variable> = Vec::new();

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
impl super::FreeVariable for Vec<IntegerNumberExpression> {
    fn get_free(&self) -> Vec<super::Variable> {
        let mut free: Vec<super::Variable> = Vec::new();
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
impl super::FreeVariable for BooleanIntegerNumberExpression {
    fn get_free(&self) -> Vec<super::Variable> {
        use BooleanIntegerNumberExpression::*;
        let mut free: Vec<super::Variable> = Vec::new();
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

impl super::Sample for IntegerNumberDomainExpression {
    fn sample(&self) -> Option<super::AssignedValue> {
        use IntegerNumberDomainExpression::*;
        match self {
            Empty => None,
            Universe => Some(super::AssignedValue::Integer(IntegerNumber::Value(0))),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BooleanIntegerNumberExpression, IntegerNumber, IntegerNumberDomainExpression,
        IntegerNumberExpression,
    };
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for IntegerNumber {
        fn arbitrary(g: &mut Gen) -> IntegerNumber {
            match u32::arbitrary(g) % 512 {
                0 => IntegerNumber::NaN,
                _ => IntegerNumber::Value(Arbitrary::arbitrary(g)),
            }
        }
    }

    impl Arbitrary for IntegerNumberExpression {
        fn arbitrary(g: &mut Gen) -> IntegerNumberExpression {
            match u32::arbitrary(g) % 16 {
                0 => IntegerNumberExpression::IntegerNumberValue(Arbitrary::arbitrary(g)),
                1 => IntegerNumberExpression::Parenthesis(Arbitrary::arbitrary(g)),
                2 => IntegerNumberExpression::Negate(Arbitrary::arbitrary(g)),
                3 => IntegerNumberExpression::Add(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
                4 => {
                    IntegerNumberExpression::Minus(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
                }
                5 => {
                    IntegerNumberExpression::Times(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
                }
                6 => IntegerNumberExpression::Divide(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                7 => IntegerNumberExpression::Modulo(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                _ => IntegerNumberExpression::IntegerNumberVariable(Arbitrary::arbitrary(g)),
            }
        }
    }

    impl Arbitrary for IntegerNumberDomainExpression {
        fn arbitrary(g: &mut Gen) -> IntegerNumberDomainExpression {
            match u32::arbitrary(g) % 32 {
                0 => IntegerNumberDomainExpression::Empty,
                1 => IntegerNumberDomainExpression::ClosedRange(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                2 => IntegerNumberDomainExpression::OpenRange(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                3 => IntegerNumberDomainExpression::OpenLeftClosedRightRange(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                4 => IntegerNumberDomainExpression::ClosedLeftOpenRightRange(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                5 => IntegerNumberDomainExpression::ExplicitSet(Arbitrary::arbitrary(g)),
                6 => IntegerNumberDomainExpression::Union(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                7 => IntegerNumberDomainExpression::Intersection(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                8 => IntegerNumberDomainExpression::Difference(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                9 => IntegerNumberDomainExpression::Complement(Arbitrary::arbitrary(g)),
                _ => IntegerNumberDomainExpression::Universe,
            }
        }
    }

    impl Arbitrary for BooleanIntegerNumberExpression {
        fn arbitrary(g: &mut Gen) -> BooleanIntegerNumberExpression {
            match u32::arbitrary(g) % 5 {
                0 => BooleanIntegerNumberExpression::Equals(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                1 => BooleanIntegerNumberExpression::Different(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                2 => BooleanIntegerNumberExpression::Greater(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                3 => BooleanIntegerNumberExpression::Less(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                4 => BooleanIntegerNumberExpression::In(
                    Arbitrary::arbitrary(g),
                    Arbitrary::arbitrary(g),
                ),
                _ => unreachable!(),
            }
        }
    }
}
