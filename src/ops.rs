use crate::lazy::LazyBuffer;

pub enum DifferentiableFunction {
    // unary
    Zero,
    Neg,
    Sin,
    Relu,
    Log,
    Exp,
    Sqrt,

    // binary
    Less,
    Add,
    Sub,
    Mul,
    Div,
    
    // ternary
    Where,

    // reduce
    Sum,
    Max,

    // movement
    Expand,
    Reshape,
    Permute,
    Pad,
    Shrink,
    Flip,
}

// pub trait Eval {
//     type Inputs<'a>;
//     type GradientOutput;

//     fn forward<'a>(&self, inputs: Self::Inputs<'a>) -> LazyBuffer;
//     fn backward<'a>(&self, inputs: Self::Inputs<'a>, output_grad: LazyBuffer) -> Self::GradientOutput;
// }

impl DifferentiableFunction {
    fn forward(&self) -> LazyBuffer {
        match self {
            DifferentiableFunction::Zero => todo!(),
            DifferentiableFunction::Neg => todo!(),
            DifferentiableFunction::Sin => todo!(),
            DifferentiableFunction::Relu => todo!(),
            DifferentiableFunction::Log => todo!(),
            DifferentiableFunction::Exp => todo!(),
            DifferentiableFunction::Sqrt => todo!(),
            DifferentiableFunction::Less => todo!(),
            DifferentiableFunction::Add => todo!(),
            DifferentiableFunction::Sub => todo!(),
            DifferentiableFunction::Mul => todo!(),
            DifferentiableFunction::Div => todo!(),
            DifferentiableFunction::Where => todo!(),
            DifferentiableFunction::Sum => todo!(),
            DifferentiableFunction::Max => todo!(),
            DifferentiableFunction::Expand => todo!(),
            DifferentiableFunction::Reshape => todo!(),
            DifferentiableFunction::Permute => todo!(),
            DifferentiableFunction::Pad => todo!(),
            DifferentiableFunction::Shrink => todo!(),
            DifferentiableFunction::Flip => todo!(),
        }
    }

    fn backward(&self) -> LazyBuffer {
        match self {
            DifferentiableFunction::Zero => todo!(),
            DifferentiableFunction::Neg => todo!(),
            DifferentiableFunction::Sin => todo!(),
            DifferentiableFunction::Relu => todo!(),
            DifferentiableFunction::Log => todo!(),
            DifferentiableFunction::Exp => todo!(),
            DifferentiableFunction::Sqrt => todo!(),
            DifferentiableFunction::Less => todo!(),
            DifferentiableFunction::Add => todo!(),
            DifferentiableFunction::Sub => todo!(),
            DifferentiableFunction::Mul => todo!(),
            DifferentiableFunction::Div => todo!(),
            DifferentiableFunction::Where => todo!(),
            DifferentiableFunction::Sum => todo!(),
            DifferentiableFunction::Max => todo!(),
            DifferentiableFunction::Expand => todo!(),
            DifferentiableFunction::Reshape => todo!(),
            DifferentiableFunction::Permute => todo!(),
            DifferentiableFunction::Pad => todo!(),
            DifferentiableFunction::Shrink => todo!(),
            DifferentiableFunction::Flip => todo!(),
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