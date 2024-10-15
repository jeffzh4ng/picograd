use ndarray::{Array, IxDyn};
use crate::{ops::DifferentiableFunction, tensor::Tensor};

pub struct LazyBuffer {
    device: String,
    shape: Vec<usize>,
    dtype: String,
    data: Array<f32, IxDyn>,
}

impl LazyBuffer {
    fn eval(&self, op: DifferentiableFunction) -> Tensor {
        match op {
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