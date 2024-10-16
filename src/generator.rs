use crate::{interpreter::TargetOp, interpreter::LazyBuffer};

pub enum DifferentiableOp {
    // unary
    Zero,
    Neg(LazyBuffer),
    Sin(LazyBuffer),
    Relu(LazyBuffer),
    Log(LazyBuffer),
    Exp(LazyBuffer),
    Sqrt(LazyBuffer),

    // binary
    Less(LazyBuffer, LazyBuffer),
    Add(LazyBuffer, LazyBuffer),
    Sub(LazyBuffer, LazyBuffer),
    Mul(LazyBuffer, LazyBuffer),
    Div(LazyBuffer, LazyBuffer),
    
    // ternary
    Where(LazyBuffer, LazyBuffer, LazyBuffer),

    // reduce
    Sum(LazyBuffer),
    Max(LazyBuffer),

    // movement
    Expand(LazyBuffer),
    Reshape(LazyBuffer),
    Permute(LazyBuffer),
    Pad(LazyBuffer),
    Shrink(LazyBuffer),
    Flip(LazyBuffer),
}

fn forward(op: DifferentiableOp) -> LazyBuffer {
    match op {
        DifferentiableOp::Zero => todo!(),
        DifferentiableOp::Neg(x) => x.eval(TargetOp::Neg),
        DifferentiableOp::Sin(x) => x.eval(TargetOp::Sin),
        DifferentiableOp::Relu(x) => todo!(),
        DifferentiableOp::Log(x) => todo!(),
        DifferentiableOp::Exp(x) => todo!(),
        DifferentiableOp::Sqrt(x) => x.eval(TargetOp::Sqrt),
        DifferentiableOp::Less(x, y) => todo!(),
        DifferentiableOp::Add(x, y) => x.eval(TargetOp::Add(y)),
        DifferentiableOp::Sub(x, y) => x.eval(TargetOp::Sub(y)),
        DifferentiableOp::Mul(x, y) => x.eval(TargetOp::Mul(y)),
        DifferentiableOp::Div(x, y) => x.eval(TargetOp::Div(y)),
        DifferentiableOp::Where(x, y, z) => todo!(),
        DifferentiableOp::Sum(x) => todo!(),
        DifferentiableOp::Max(x) => todo!(),
        DifferentiableOp::Expand(x) => todo!(),
        DifferentiableOp::Reshape(x) => todo!(),
        DifferentiableOp::Permute(x) => todo!(),
        DifferentiableOp::Pad(x) => todo!(),
        DifferentiableOp::Shrink(x) => todo!(),
        DifferentiableOp::Flip(x) => todo!(),
    }
}

fn backward(op: DifferentiableOp) -> LazyBuffer {
    match op {
            DifferentiableOp::Zero => todo!(),
            DifferentiableOp::Neg(x) => todo!(),
            DifferentiableOp::Sin(x) => todo!(),
            DifferentiableOp::Relu(x) => todo!(),
            DifferentiableOp::Log(x) => todo!(),
            DifferentiableOp::Exp(x) => todo!(),
            DifferentiableOp::Sqrt(x) => todo!(),
            DifferentiableOp::Less(x, y) => todo!(),
            DifferentiableOp::Add(x, y) => todo!(),
            DifferentiableOp::Sub(x, y) => todo!(),
            DifferentiableOp::Mul(x, y) => todo!(),
            DifferentiableOp::Div(x, y) => todo!(),
            DifferentiableOp::Where(x, y, z) => todo!(),
            DifferentiableOp::Sum(x) => todo!(),
            DifferentiableOp::Max(x) => todo!(),
            DifferentiableOp::Expand(x) => todo!(),
            DifferentiableOp::Reshape(x) => todo!(),
            DifferentiableOp::Permute(x) => todo!(),
            DifferentiableOp::Pad(x) => todo!(),
            DifferentiableOp::Shrink(x) => todo!(),
            DifferentiableOp::Flip(x) => todo!(),
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
// }