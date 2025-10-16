#[allow(warnings)]
mod bindings;

use bindings::exports::docs::calculator::calculate::{Guest, Op};

// Bring the imported add function into scope
use bindings::docs::adder::add::add;

// Bring the imported subtract function into scope
use bindings::docs::subtractor::subtract::subtract;

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x, y),
            Op::Subtract => subtract(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);