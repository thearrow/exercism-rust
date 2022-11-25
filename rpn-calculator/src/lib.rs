use CalculatorInput::*;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn try_op(stack: &mut Vec<CalculatorInput>, op: fn(i32, i32) -> i32) -> bool {
    if let Some(r) = stack.pop() {
        if let Some(l) = stack.pop() {
            if let (Value(rv), Value(lv)) = (r, l) {
                stack.push(Value(op(lv, rv)));
                return true;
            }
        }
    }
    false
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<CalculatorInput> = vec![];

    for n in inputs {
        if !match n {
            Value(i) => {
                stack.push(Value(*i));
                true
            }
            Add => try_op(&mut stack, |l, r| l + r),
            Subtract => try_op(&mut stack, |l, r| l - r),
            Multiply => try_op(&mut stack, |l, r| l * r),
            Divide => try_op(&mut stack, |l, r| l / r),
        } {
            return None;
        }
    }

    if let Some(Value(n)) = stack.pop() {
        if stack.is_empty() {
            Some(n)
        } else {
            None
        }
    } else {
        None
    }
}
