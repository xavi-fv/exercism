#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Value(value) => {
                stack.push(*value);
            }
            _ => {
                if stack.len() < 2 { return None; }
                let v2 = stack.pop().expect("There must be a value here");
                let v1 = stack.pop().expect("There must be a value here");
                let result: i32 = match input {
                    CalculatorInput::Add => v1 + v2,
                    CalculatorInput::Subtract => v1 - v2,
                    CalculatorInput::Multiply => v1 * v2,
                    CalculatorInput::Divide => v1 / v2,
                    _ => unreachable!()
                };
                stack.push(result);
            }
        }
    }
    if stack.len() > 1 { None } else { stack.pop() }
}
