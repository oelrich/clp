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
