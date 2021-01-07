use std::collections::HashMap;
use super::ID;
use super::SymbolTable;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub ops: Vec<Op>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Value(ID),
    Unary(Unary),
    Binary(Binary),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unary {
    Negate,
}

impl Unary {
    fn evaluate(&self, value: ID) -> Option<ID> {
        match (self, value) {
            (Unary::Negate, ID::Integer(i)) => Some(ID::Integer(-1i64 * i)),
            (Unary::Negate, ID::Bool(b)) => Some(ID::Bool(!b)),
             _ => {
                 println!("unexpected value type on the stack");
                 return None;
             }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Binary {
    LessThan,
    GreaterThan,
    Add,
    And,
}

impl Binary {
    fn evaluate(&self, left: ID, right: ID) -> Option<ID> {
        match (self, left, right) {
            (Binary::LessThan, ID::Integer(i), ID::Integer(j)) => Some(ID::Bool(i < j)),
            (Binary::GreaterThan, ID::Integer(i), ID::Integer(j)) => Some(ID::Bool(i > j)),
            (Binary::Add, ID::Integer(i), ID::Integer(j)) => Some(ID::Integer(i + j)),
            (Binary::And, ID::Bool(i), ID::Bool(j)) => Some(ID::Bool(i & j)),
            _ => {
                println!("unexpected value type on the stack");
                return None;
            }
        }
    }
}

impl Expression {
    pub fn evaluate(&self, values: &HashMap<u32, ID>) -> Option<ID> {
        let mut stack: Vec<ID> = Vec::new();

        for op in self.ops.iter() {
            println!("op: {:?}\t| stack: {:?}", op, stack);
            match op {
                Op::Value(ID::Variable(i)) => match values.get(&i) {
                    Some(id) => stack.push(id.clone()),
                    None => {
                        println!("unknown variable {}", i);
                        return None;
                    }
                }
                Op::Value(id) => stack.push(id.clone()),
                Op::Unary(unary) => match stack.pop() {
                    None => {
                        println!("expected a value on the stack");
                        return None;
                    }
                    Some(id) => match unary.evaluate(id) {
                        Some(res) => stack.push(res),
                        None => return None,
                    }
                },
                Op::Binary(binary) => match (stack.pop(), stack.pop()) {
                    (Some(right_id), Some(left_id)) => match binary.evaluate(left_id, right_id) {
                        Some(res) => stack.push(res),
                        None => return None,
                    },
                    _ => {
                        println!("expected two values on the stack");
                        return None;
                    }
                }
            }
        }

        if stack.len() == 1 {
            Some(stack.remove(0))
        } else {
            None
        }
    }

    pub fn print(&self, symbols: &SymbolTable) -> String {
        let mut stack: Vec<String> = Vec::new();
        let s = "<invalid expression>".to_string();

        for op in self.ops.iter() {
            println!("op: {:?}\t| stack: {:?}", op, stack);
            match op {
                Op::Value(i) => stack.push(symbols.print_id(&i)),
                Op::Unary(unary) => match unary {
                    Unary::Negate => match stack.pop() {
                        None => return s,
                        Some(s) => stack.push(format!("-{}", s)),
                    },
                },
                Op::Binary(binary) => match (stack.pop(), stack.pop()) {
                    (Some(right), Some(left)) => match binary {
                        Binary::LessThan => stack.push(format!("{} < {}", left, right)),
                        Binary::GreaterThan => stack.push(format!("{} > {}", left, right)),
                        Binary::Add => stack.push(format!("{} + {}", left, right)),
                        Binary::And => stack.push(format!("{} && {}", left, right)),
                    },
                    _ => return s,
                }
            }
        }

        if stack.len() == 1 {
            stack.remove(0)
        } else {
            s
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::SymbolTable;

    #[test]
    fn negate() {
        let symbols = SymbolTable {
            symbols: vec![
                "test1".to_string(),
                "test2".to_string(),
                "var1".to_string(),
            ],
        };

        let ops = vec![
            Op::Value(ID::Integer(1)),
            Op::Unary(Unary::Negate),
            Op::Value(ID::Variable(2)),
            Op::Binary(Binary::LessThan),
        ];

        let values: HashMap<u32, ID> = [(2, ID::Integer(0))]
            .iter().cloned().collect();

        println!("ops: {:?}", ops);

        let e = Expression { ops };
        println!("print: {}", e.print(&symbols));

        let res = e.evaluate(&values);
        assert_eq!(res, Some(ID::Bool(true)));
        panic!();
    }


    #[test]
    fn printer() {
        let symbols = SymbolTable {
            symbols: vec![
                "test1".to_string(),
                "test2".to_string(),
                "var1".to_string(),
            ],
        };

        let ops1 = vec![
            Op::Value(ID::Integer(1)),
            Op::Unary(Unary::Negate),
            Op::Value(ID::Variable(2)),
            Op::Binary(Binary::LessThan),
        ];

        let ops2 = vec![
            Op::Value(ID::Integer(1)),
            Op::Value(ID::Integer(2)),
            Op::Value(ID::Integer(3)),
            Op::Binary(Binary::Add),
            Op::Binary(Binary::LessThan),
        ];

        let ops3 = vec![
            Op::Value(ID::Integer(1)),
            Op::Value(ID::Integer(2)),
            Op::Binary(Binary::Add),
            Op::Value(ID::Integer(3)),
            Op::Binary(Binary::LessThan),
        ];

        println!("ops1: {:?}", ops1);
        println!("ops2: {:?}", ops2);
        println!("ops3: {:?}", ops3);
        let e1 = Expression { ops: ops1 };
        let e2 = Expression { ops: ops2 };
        let e3 = Expression { ops: ops3 };

        assert_eq!(e1.print(&symbols), "-1 < $var1");

        assert_eq!(e2.print(&symbols), "1 < 2 + 3");

        assert_eq!(e3.print(&symbols), "1 + 2 < 3");
        //panic!();
    }

}
