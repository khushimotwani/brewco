// src/type_checker.rs

use crate::ast::{Statement, Expr, BinaryOperator};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Object(HashMap<String, Type>),
    Array(Box<Type>),
    Function {
        param_types: Vec<Type>,
        return_type: Box<Type>,
    },
    Any,    // For when we can't determine the type, or for dynamic features
    Null,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number => write!(f, "Number"),
            Type::String => write!(f, "String"),
            Type::Boolean => write!(f, "Boolean"),
            Type::Object(_) => write!(f, "Object"),
            Type::Array(t) => write!(f, "Array<{}>", t),
            Type::Function { .. } => write!(f, "Function"),
            Type::Any => write!(f, "Any"),
            Type::Null => write!(f, "Null"),
        }
    }
}

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Type>>,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            scopes: vec![HashMap::new()],
            errors: Vec::new(),
        }
    }

    pub fn check(&mut self, statements: &[Statement]) -> Result<(), Vec<String>> {
        for statement in statements {
            self.check_statement(statement);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn add_error(&mut self, message: String) {
        self.errors.push(message);
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_var(&mut self, name: &str, var_type: Type) {
        self.scopes.last_mut().unwrap().insert(name.to_string(), var_type);
    }

    fn get_var_type(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(t) = scope.get(name) {
                return Some(t.clone());
            }
        }
        None
    }

    fn string_to_type(&self, type_str: &str) -> Type {
        match type_str {
            "Number" => Type::Number,
            "String" => Type::String,
            "Boolean" => Type::Boolean,
            // Add more complex types like Array<String> later
            _ => Type::Any, // For unknown types for now
        }
    }

    fn check_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VarDecl { name, type_ann, value } => {
                let value_type = self.infer_expr_type(value);

                if let Some(ann_str) = type_ann {
                    let declared_type = self.string_to_type(ann_str);
                    if value_type != declared_type {
                        self.add_error(format!(
                            "Type mismatch for '{}': expected {}, but got {}.",
                            name, declared_type, value_type
                        ));
                    }
                    self.define_var(name, declared_type);
                } else {
                    // No annotation, infer and store
                    self.define_var(name, value_type);
                }
            }
            Statement::ExprStmt(expr) => {
                self.infer_expr_type(expr); // Evaluate for side-effects and errors
            }
            // We will add other statement types here
            _ => (),
        }
    }

    fn infer_expr_type(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Number(_) => Type::Number,
            Expr::String(_) => Type::String,
            Expr::Boolean(_) => Type::Boolean,
            Expr::Identifier(name) => {
                if let Some(t) = self.get_var_type(name) {
                    t
                } else {
                    self.add_error(format!("Variable '{}' not found.", name));
                    Type::Any // Return Any to prevent cascade errors
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_type = self.infer_expr_type(left);
                let right_type = self.infer_expr_type(right);

                match op {
                    // Handle numeric and string operations
                    BinaryOperator::Add => {
                        if (left_type == Type::Number || left_type == Type::String) &&
                           (right_type == Type::Number || right_type == Type::String) {
                            // If either is a string, the result is a string
                            if left_type == Type::String || right_type == Type::String {
                                Type::String
                            } else {
                                Type::Number
                            }
                        } else {
                            self.add_error(format!(
                                "The 'add' operation only supports numbers or strings, but got {} and {}.",
                                left_type, right_type
                            ));
                            Type::Any
                        }
                    }
                    BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => {
                        if left_type != Type::Number || right_type != Type::Number {
                            self.add_error(format!(
                                "Arithmetic operation requires two numbers, but got {} and {}.",
                                left_type, right_type
                            ));
                            return Type::Any;
                        }
                        Type::Number
                    }
                    // Comparison operators
                    BinaryOperator::Equal | BinaryOperator::NotEqual => {
                        if (left_type == Type::Number && right_type == Type::Number) ||
                           (left_type == Type::String && right_type == Type::String) ||
                           (left_type == Type::Boolean && right_type == Type::Boolean) {
                            Type::Boolean
                        } else {
                            self.add_error(format!(
                                "Cannot compare {} and {}. They must be of the same type.",
                                left_type, right_type
                            ));
                            Type::Any
                        }
                    }
                    BinaryOperator::Greater | BinaryOperator::Less | BinaryOperator::GreaterEqual | BinaryOperator::LessEqual => {
                        if left_type == Type::Number && right_type == Type::Number {
                            Type::Boolean
                        } else {
                            self.add_error(format!(
                                "Can only compare numbers, but got {} and {}.",
                                left_type, right_type
                            ));
                            Type::Any
                        }
                    }
                    // Logical operators
                    BinaryOperator::And | BinaryOperator::Or => {
                        if left_type == Type::Boolean && right_type == Type::Boolean {
                            Type::Boolean
                        } else {
                            self.add_error(format!(
                                "Logical operators require two booleans, but got {} and {}.",
                                left_type, right_type
                            ));
                            Type::Any
                        }
                    }
                    // Other operators will be handled later
                    _ => Type::Any,
                }
            }
            // More expressions to be handled later
            _ => Type::Any,
        }
    }
} 