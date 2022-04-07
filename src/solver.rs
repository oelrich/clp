use crate::expressions::{
    AssignedValue, Assignment, ConstraintProgramExpression, Symbol, Variable,
};

/// Assigned value to a constant or variable in a solution.
pub enum Solution {
    Unsatisfiable(Symbol, String),
    Variable(Symbol, AssignedValue),
    Constant(Symbol, AssignedValue),
}
pub fn generate_attempt(free: Vec<Variable>) -> Option<Vec<Assignment>> {
    let mut assigned = Vec::default();
    for x in free {
        if let Some(assignment) = x.assignment() {
            assigned.push(assignment);
        } else {
            return None;
        }
    }
    Some(assigned)
}
pub fn apply(
    program: ConstraintProgramExpression,
    _state: Vec<Assignment>,
) -> ConstraintProgramExpression {
    program
}
pub fn reduce(program: ConstraintProgramExpression) -> ConstraintProgramExpression {
    program
}

pub fn free_variables(program: &ConstraintProgramExpression) -> Vec<Variable> {
    use crate::expressions::FreeVariable;
    program.get_free()
}
pub fn solve(_program: ConstraintProgramExpression) -> Vec<Solution> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::apply;
    use super::free_variables;
    use super::generate_attempt;
    use super::ConstraintProgramExpression;

    #[quickcheck_macros::quickcheck]
    fn a_solution_covers_all_free_variables(p: ConstraintProgramExpression) -> bool {
        let free = free_variables(&p);
        println!("{:?}", free);
        if let Some(attempt) = generate_attempt(free) {
            let update_program = apply(p, attempt);
            let free_after_apply = free_variables(&update_program);
            free_after_apply.is_empty()
        } else {
            true
        }
    }
}
