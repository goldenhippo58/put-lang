use crate::ast::*;

pub struct TypeChecker;

impl TypeChecker {
    pub fn check_program(program: &ProgramNode) -> Result<(), String> {
        for statement in &program.statements {
            Self::check_statement(statement.as_ref())?;
        }
        Ok(())
    }

    fn check_statement(statement: &dyn StatementNode) -> Result<(), String> {
        match statement.as_any().downcast_ref::<VariableNode>() {
            Some(var) => {
                // Check variable data types or more logic can be added
                Ok(())
            }
            None => match statement.as_any().downcast_ref::<AssignmentNode>() {
                Some(assign) => {
                    // Ensure the types of the left and right side of assignment match
                    Self::check_statement(assign.left.as_ref())?;
                    Self::check_statement(assign.right.as_ref())?;
                    Ok(())
                }
                None => match statement.as_any().downcast_ref::<BinaryOperationNode>() {
                    Some(bin_op) => {
                        // Check types of operands and ensure they are compatible with the operation
                        Self::check_statement(bin_op.left.as_ref())?;
                        Self::check_statement(bin_op.right.as_ref())?;
                        Ok(())
                    }
                    None => Err("Unknown statement type encountered".to_string()),
                },
            },
        }
    }
}
