use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct OperationResult {
    pub(crate) line_number: i32,
    pub(crate) accumelator: i32,
}

pub struct Operations<'a> {
    operations: HashMap<&'a str, fn(i32, i32, i32) -> OperationResult>,
}

impl Operations<'_> {
    pub(crate) fn new() -> Self {
        Operations {
            operations: [
                ("acc", acc as fn(i32, i32, i32) -> OperationResult),
                ("jmp", jmp as fn(i32, i32, i32) -> OperationResult),
                ("nop", nop as fn(i32, i32, i32) -> OperationResult)
            ].iter().cloned().collect()
        }
    }
}

pub trait Execute {
    fn execute(&self, operation: &str, argument: i32, line_number: i32, accumelator: i32) -> OperationResult;
}

impl Execute for Operations<'_> {
    fn execute(&self, operation: &str, argument: i32, line_number: i32, accumelator: i32) -> OperationResult {
        return self.operations.get(operation).unwrap()(line_number, accumelator, argument);
    }
}

fn acc(line_number: i32, accumelator: i32, argument: i32) -> OperationResult {
    return OperationResult{ line_number: line_number + 1, accumelator: accumelator + argument};
}

fn jmp(line_number: i32, accumelator: i32, argument: i32) -> OperationResult {
    return OperationResult{line_number: line_number + argument, accumelator};
}

fn nop(line_number: i32, accumelator: i32, argument: i32) -> OperationResult {
    return OperationResult{line_number: line_number + 1, accumelator};
}