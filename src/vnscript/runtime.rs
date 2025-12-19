use crate::vnscript::ast::*;
use std::collections::HashMap;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

pub struct Runtime {
    pub variables: HashMap<String, Value>,
    pub current_index: usize,
    pub labels: HashMap<String, usize>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            current_index: 0,
            labels: HashMap::new(),
        }
    }
    
    pub fn index_labels(&mut self, statements: &[Statement]) {
        for (i, statement) in statements.iter().enumerate() {
            if let Statement::Label(name) = statement {
                self.labels.insert(name.clone(), i);
            }
        }
    }
    
    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    pub fn set_var(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn evaluate_expression(&self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Integer(i) => Ok(Value::Integer(*i)),
            Expression::Float(f) => Ok(Value::Float(*f)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Boolean(b) => Ok(Value::Boolean(*b)),
            Expression::Variable(name) => {
                self.get_var(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Variable '{}' not found", name))
            }
            Expression::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_op(&left_val, op, &right_val)
            }
            Expression::UnaryOp(op, expr) => {
                let val = self.evaluate_expression(expr)?;
                self.apply_unary_op(op, &val)
            }
        }
    }
    
    fn apply_binary_op(&self, left: &Value, op: &BinaryOperator, right: &Value) -> Result<Value> {
        match (left, op, right) {
            (Value::Integer(l), BinaryOperator::Add, Value::Integer(r)) => Ok(Value::Integer(l + r)),
            (Value::Integer(l), BinaryOperator::Subtract, Value::Integer(r)) => Ok(Value::Integer(l - r)),
            (Value::Integer(l), BinaryOperator::Multiply, Value::Integer(r)) => Ok(Value::Integer(l * r)),
            (Value::Integer(l), BinaryOperator::Divide, Value::Integer(r)) => {
                if *r == 0 {
                    return Err(anyhow!("Division by zero"));
                }
                Ok(Value::Integer(l / r))
            }
            (Value::Integer(l), BinaryOperator::Equal, Value::Integer(r)) => Ok(Value::Boolean(l == r)),
            (Value::Integer(l), BinaryOperator::NotEqual, Value::Integer(r)) => Ok(Value::Boolean(l != r)),
            (Value::Integer(l), BinaryOperator::Greater, Value::Integer(r)) => Ok(Value::Boolean(l > r)),
            (Value::Integer(l), BinaryOperator::Less, Value::Integer(r)) => Ok(Value::Boolean(l < r)),
            (Value::Integer(l), BinaryOperator::GreaterEqual, Value::Integer(r)) => Ok(Value::Boolean(l >= r)),
            (Value::Integer(l), BinaryOperator::LessEqual, Value::Integer(r)) => Ok(Value::Boolean(l <= r)),
            (Value::Boolean(l), BinaryOperator::And, Value::Boolean(r)) => Ok(Value::Boolean(*l && *r)),
            (Value::Boolean(l), BinaryOperator::Or, Value::Boolean(r)) => Ok(Value::Boolean(*l || *r)),
            _ => Err(anyhow!("Type mismatch in binary operation")),
        }
    }
    
    fn apply_unary_op(&self, op: &UnaryOperator, val: &Value) -> Result<Value> {
        match (op, val) {
            (UnaryOperator::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            (UnaryOperator::Negate, Value::Integer(i)) => Ok(Value::Integer(-i)),
            (UnaryOperator::Negate, Value::Float(f)) => Ok(Value::Float(-f)),
            _ => Err(anyhow!("Type mismatch in unary operation")),
        }
    }
    
    pub fn jump_to_label(&mut self, label: &str) -> Result<()> {
        if let Some(&index) = self.labels.get(label) {
            self.current_index = index;
            Ok(())
        } else {
            Err(anyhow!("Label '{}' not found", label))
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
