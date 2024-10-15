use crate::lazy::LazyBuffer;

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

// pub trait Eval {
//     type Inputs<'a>;
//     type GradientOutput;

//     fn forward<'a>(&self, inputs: Self::Inputs<'a>) -> LazyBuffer;
//     fn backward<'a>(&self, inputs: Self::Inputs<'a>, output_grad: LazyBuffer) -> Self::GradientOutput;
// }

impl DifferentiableOp {
    fn forward(&self) -> LazyBuffer {
        match self {
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

    fn backward(&self) -> LazyBuffer {
        match self {
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
}


// impl Function for Log {
//     type Inputs<'a> = &'a Tensor;
//     type Gradients = Tensor;
    
//     fn forward(input: Self::Inputs<'_>) -> Tensor {
//         // Implementation here
//         todo!()
//     }

//     fn backward(input: Self::Inputs<'_>, output_grad: Tensor) -> Self::Gradients {
//         // Implementation here
//         todo!()
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;
// }