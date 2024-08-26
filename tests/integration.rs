use put_lang::ast::{AssignmentNode, BinaryOperationNode, VariableNode};
use put_lang::lexer::Lexer;
use put_lang::parser::Parser;
use put_lang::tensor::Tensor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let source = "var x = 42 + 5;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        assert_eq!(program.statements.len(), 1);

        if let Some(assignment) = program.statements[0]
            .as_any()
            .downcast_ref::<AssignmentNode>()
        {
            assert!(assignment
                .left
                .as_any()
                .downcast_ref::<VariableNode>()
                .is_some());
            assert!(assignment
                .right
                .as_any()
                .downcast_ref::<BinaryOperationNode>()
                .is_some());
        } else {
            panic!("Expected AssignmentNode");
        }
    }

    #[test]
    fn test_complex_expression() {
        let source = "var x = (42 + 5) * 2 - 3 / 1.5;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        assert_eq!(program.statements.len(), 1);
        // Add more specific assertions to check the structure of the AST
    }

    #[test]
    fn test_tensor_operations() {
        let t1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let t2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);

        let t_add = &t1 + &t2;
        assert_eq!(t_add.get(&[0, 0]), Some(6.0));

        let t_sub = &t1 - &t2;
        assert_eq!(t_sub.get(&[0, 0]), Some(-4.0));

        let t_mul = &t1 * &t2;
        assert_eq!(t_mul.get(&[0, 0]), Some(5.0));

        let t_matmul = t1.matmul(&t2).unwrap();
        assert_eq!(t_matmul.get(&[0, 0]), Some(19.0));

        let t_transpose = t1.transpose();
        assert_eq!(t_transpose.get(&[0, 1]), Some(3.0));

        assert!((t1.mean() - 2.5).abs() < 1e-6);
        assert!((t1.variance() - 1.25).abs() < 1e-6);
        assert!((t1.std_dev() - 1.118033988749895).abs() < 1e-6);
    }
}
