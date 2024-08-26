use std::any::Any;
use std::fmt::Debug;

pub trait StatementNode: Debug {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct ProgramNode {
    pub statements: Vec<Box<dyn StatementNode>>,
}

impl ProgramNode {
    pub fn new() -> Self {
        ProgramNode {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct VariableNode {
    pub name: String,
}

impl VariableNode {
    pub fn new(name: String) -> Self {
        VariableNode { name }
    }
}

impl StatementNode for VariableNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct NumberNode {
    pub value: String,
}

impl NumberNode {
    pub fn new(value: String) -> Self {
        NumberNode { value }
    }
}

impl StatementNode for NumberNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct AssignmentNode {
    pub left: Box<dyn StatementNode>,
    pub right: Box<dyn StatementNode>,
}

impl AssignmentNode {
    pub fn new(left: Box<dyn StatementNode>, right: Box<dyn StatementNode>) -> Self {
        AssignmentNode { left, right }
    }
}

impl StatementNode for AssignmentNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct BinaryOperationNode {
    pub left: Box<dyn StatementNode>,
    pub operator: BinaryOperator,
    pub right: Box<dyn StatementNode>,
}

impl BinaryOperationNode {
    pub fn new(
        left: Box<dyn StatementNode>,
        operator: BinaryOperator,
        right: Box<dyn StatementNode>,
    ) -> Self {
        BinaryOperationNode {
            left,
            operator,
            right,
        }
    }
}

impl StatementNode for BinaryOperationNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ParenthesisNode {
    pub expression: Box<dyn StatementNode>,
}

impl ParenthesisNode {
    pub fn new(expression: Box<dyn StatementNode>) -> Self {
        ParenthesisNode { expression }
    }
}

impl StatementNode for ParenthesisNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct IfNode {
    pub condition: Box<dyn StatementNode>,
    pub then_branch: Box<dyn StatementNode>,
    pub else_branch: Option<Box<dyn StatementNode>>,
}

impl IfNode {
    pub fn new(
        condition: Box<dyn StatementNode>,
        then_branch: Box<dyn StatementNode>,
        else_branch: Option<Box<dyn StatementNode>>,
    ) -> Self {
        IfNode {
            condition,
            then_branch,
            else_branch,
        }
    }
}

impl StatementNode for IfNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WhileNode {
    pub condition: Box<dyn StatementNode>,
    pub body: Box<dyn StatementNode>,
}

impl WhileNode {
    pub fn new(condition: Box<dyn StatementNode>, body: Box<dyn StatementNode>) -> Self {
        WhileNode { condition, body }
    }
}

impl StatementNode for WhileNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
