mod ast;
mod lexer;
mod parser;
mod tensor;
mod token;
mod zom_parser;

use ast::{
    AssignmentNode, BinaryOperationNode, NumberNode, ParenthesisNode, StatementNode, VariableNode,
};
use lexer::Lexer;
use parser::Parser;
use std::fs::File;
use tensor::Tensor;
use zom_parser::{parse_zom_file, ProjectConfig};

fn main() {
    // Try to parse project.zom file
    let config = match File::open("project.zom") {
        Ok(file) => match parse_zom_file(file) {
            Ok(cfg) => {
                println!("Successfully parsed project.zom");
                Some(cfg)
            }
            Err(e) => {
                eprintln!("Error parsing project.zom: {}", e);
                None
            }
        },
        Err(_) => {
            println!("project.zom not found, using default configuration");
            None
        }
    };

    // Use the config to set up your environment if it exists
    if let Some(cfg) = config {
        setup_environment(&cfg);
    } else {
        println!("Using default environment setup");
    }

    // Your existing PUT language processing logic
    let source = "var x = (42 + 5) * 2 - 3 / 1.5;";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    println!("Program: {:?}", program);

    println!("\nAST Structure:");
    if program.statements.is_empty() {
        println!("Failed to parse any statements.");
    } else {
        for statement in program.statements {
            print_statement(&statement, 0);
        }
    }

    // Demonstrate tensor operations
    demo_tensor_operations();
}

fn setup_environment(config: &ProjectConfig) {
    println!("Setting up environment based on project.zom:");
    println!(
        "Project name: {}",
        config
            .project_info
            .get("name")
            .unwrap_or(&"Unknown".to_string())
    );
    println!(
        "Project version: {}",
        config
            .project_info
            .get("version")
            .unwrap_or(&"0.0.0".to_string())
    );

    for (basket, version) in &config.dependencies {
        println!("Loading basket: {} (version {})", basket, version);
        // Implement basket loading logic here
    }

    // Apply other configuration settings as needed
}

fn print_statement(statement: &Box<dyn StatementNode>, indent: usize) {
    let indent_str = "  ".repeat(indent);

    if let Some(assignment) = statement.as_any().downcast_ref::<AssignmentNode>() {
        println!("{}AssignmentNode", indent_str);
        print_statement(&assignment.left, indent + 1);
        print_statement(&assignment.right, indent + 1);
    } else if let Some(variable) = statement.as_any().downcast_ref::<VariableNode>() {
        println!("{}VariableNode: {}", indent_str, variable.name);
    } else if let Some(number) = statement.as_any().downcast_ref::<NumberNode>() {
        println!("{}NumberNode: {}", indent_str, number.value);
    } else if let Some(binary_op) = statement.as_any().downcast_ref::<BinaryOperationNode>() {
        println!(
            "{}BinaryOperationNode: {:?}",
            indent_str, binary_op.operator
        );
        print_statement(&binary_op.left, indent + 1);
        print_statement(&binary_op.right, indent + 1);
    } else if let Some(paren) = statement.as_any().downcast_ref::<ParenthesisNode>() {
        println!("{}ParenthesisNode", indent_str);
        print_statement(&paren.expression, indent + 1);
    } else {
        println!("{}Unknown node type", indent_str);
    }
}

fn demo_tensor_operations() {
    println!("\nDemonstrating Tensor Operations:");

    let t1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
    let t2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);

    println!("t1 = {}", t1);
    println!("t2 = {}", t2);

    let t_add = &t1 + &t2;
    println!("t1 + t2 = {}", t_add);

    let t_sub = &t1 - &t2;
    println!("t1 - t2 = {}", t_sub);

    let t_mul = &t1 * &t2;
    println!("t1 * t2 (element-wise) = {}", t_mul);

    let t_matmul = t1.matmul(&t2).unwrap();
    println!("t1 @ t2 (matrix multiplication) = {}", t_matmul);

    let t_transpose = t1.transpose();
    println!("t1 transposed = {}", t_transpose);

    let t_exp = t1.exp();
    println!("exp(t1) = {}", t_exp);

    let t_log = t1.log();
    println!("log(t1) = {}", t_log);

    println!("Mean of t1 = {}", t1.mean());
    println!("Variance of t1 = {}", t1.variance());
    println!("Standard deviation of t1 = {}", t1.std_dev());
}
