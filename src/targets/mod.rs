use std::fmt;

use crate::lazy::LazyBuffer;

#[derive(Clone, PartialEq, Debug)]
pub enum TargetOp {
    // unary
    Noop,
    Exp2,
    Log2,
    Cast,
    Sin,
    Sqrt,
    Recip,
    Neg,

    // binary
    Add,
    Sub,
    Mul,
    Div,
    MaxBin, // ?
    Mod,
    Lt,

    // reduce
    Sum,
    MaxReduce, // ?

    // ternary
    MulAcc,
    Where,

    // movement
    Reshape,
    Permute,
    Expand,
    Pad,
    Shrink,
    Stride,

    // load
    Empty,
    Rand,
    Const,
    From,
    Contiguous,
    Custom,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Device {
    Cpu,
    Gpu,
    Tpu,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Device::Cpu => write!(f, "CPU"),
            Device::Gpu => write!(f, "GPU"),
            Device::Tpu => write!(f, "TPU"),
        }
    }
}

pub fn cpu_eval(op: TargetOp, inputs: Vec<LazyBuffer>) -> LazyBuffer {
    match op {
        TargetOp::Noop => todo!(),
        TargetOp::Exp2 => todo!(),
        TargetOp::Log2 => todo!(),
        TargetOp::Cast => todo!(),
        TargetOp::Sin => todo!(),
        TargetOp::Sqrt => todo!(),
        TargetOp::Recip => todo!(),
        TargetOp::Neg => todo!(),
        TargetOp::Add => todo!(),
        TargetOp::Sub => todo!(),
        TargetOp::Mul => todo!(),
        TargetOp::Div => todo!(),
        TargetOp::MaxBin => todo!(),
        TargetOp::Mod => todo!(),
        TargetOp::Lt => todo!(),
        TargetOp::Sum => todo!(),
        TargetOp::MaxReduce => todo!(),
        TargetOp::MulAcc => todo!(),
        TargetOp::Where => todo!(),
        TargetOp::Reshape => todo!(),
        TargetOp::Permute => todo!(),
        TargetOp::Expand => todo!(),
        TargetOp::Pad => todo!(),
        TargetOp::Shrink => todo!(),
        TargetOp::Stride => todo!(),
        TargetOp::Empty => todo!(),
        TargetOp::Rand => todo!(),
        TargetOp::Const => todo!(),
        TargetOp::From => todo!(),
        TargetOp::Contiguous => todo!(),
        TargetOp::Custom => todo!(),
    }
}