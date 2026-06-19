use std::collections::HashMap;
use crate::parser::{ASTNode, ConditionalBranch, ConditionType, ActionType, MathOp};


pub struct Environment {
    // Stores variables mapped to their numerical values
    pub variables: HashMap<String, i64>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    // We pass our environment as a mutable reference through the evaluation tree
    pub fn execute(&self, node: &ASTNode, env: &mut Environment, file_path: &str) {
        match node {
            ASTNode::Program { loop_stmt } => self.execute(loop_stmt, env, file_path),

            ASTNode::Loop { limit, body } => {
                for cycle in 1..=*limit {
                    for stmt in body {
                        self.execute_stmt(stmt, cycle, env, file_path);
                    }
                }
            }
            _ => {}
        }
    }

    fn execute_stmt(&self, node: &ASTNode, cycle: i64, env: &mut Environment, file_path: &str) {
        match node {
            ASTNode::ConditionalBlock { branches } => {
                for branch in branches {
                    let matches = match branch.condition {
                        ConditionType::DivisibleByBoth(x, y) => cycle % x == 0 && cycle % y == 0,
                        ConditionType::DivisibleBy(x) => cycle % x == 0,
                        ConditionType::Otherwise => true,
                    };

                    if matches {
                        match &branch.action {
                            ActionType::PrintString(text) => println!("{}", text),
                            ActionType::PrintRawNumber => {
                                // If we are running inside a While loop, print the variable's current value instead of the global loop index
                                if let Some(val) = env.variables.get("VIVEC") {
                                    println!("{}", val);
                                } else {
                                    println!("{}", cycle);
                                }
                            }
                        }
                        break;
                    }
                }
            }

            ASTNode::VariableAssign { name, value } => {
                env.variables.insert(name.clone(), *value);
            }

            ASTNode::VariableMath { name, op, value } => {
                if let Some(val) = env.variables.get_mut(name) {
                    match op {
                        MathOp::Add => *val += *value,
                        MathOp::Subtract => *val -= *value,
                        MathOp::Multiply => *val *= *value,
                        MathOp::Divide => {
                            if *value == 0 {
                                panic!("Zero-Sum Critical Fault: The machine divided by zero and ceased to exist.");
                            }
                            *val /= *value;
                        }
                    }
                } else {
                    panic!("Mundus Runtime Error: The variable '{}' does not exist in this reality!", name);
                }
            }


            ASTNode::WhileLoop { var_name, body } => {
                while *env.variables.get(var_name).unwrap_or(&0) > 0 {
                    for stmt in body {
                        // Add file_path here to fix the 4-argument requirement:
                        self.execute_stmt(stmt, cycle, env, file_path);
                    }
                }
            }

            _ => {}
        }
    }
}
