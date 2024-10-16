use std::fmt;

use crate::lazy::LazyBuffer;

#[derive(Clone, PartialEq, Debug)]
pub enum TargetOp {
    // unary
    Noop(LazyBuffer),
    Exp2(LazyBuffer),
    Log2(LazyBuffer),
    Cast(LazyBuffer),
    Sin(LazyBuffer),
    Sqrt(LazyBuffer),
    Recip(LazyBuffer),
    Neg(LazyBuffer),

    // binary
    Add(LazyBuffer, LazyBuffer),
    Sub(LazyBuffer, LazyBuffer),
    Mul(LazyBuffer, LazyBuffer),
    Div(LazyBuffer, LazyBuffer),
    Mx(LazyBuffer, LazyBuffer),
    Mod(LazyBuffer, LazyBuffer),
    Lt(LazyBuffer, LazyBuffer),

    // reduce
    Sum(LazyBuffer),
    MaxReduce(LazyBuffer),

    // ternary
    MulAcc(LazyBuffer, LazyBuffer, LazyBuffer),
    Where(LazyBuffer, LazyBuffer, LazyBuffer),

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

impl Device {
    pub fn eval(&self, op: TargetOp) -> LazyBuffer {
        match self {
            Device::Cpu => match op {
                TargetOp::Noop(x) => todo!(),
                TargetOp::Exp2(x) => LazyBuffer::new(x.buf.mapv(|x| x.exp2())),
                TargetOp::Log2(x) => LazyBuffer::new(x.buf.mapv(|x| x.log2())),
                TargetOp::Cast(x) => todo!(),
                TargetOp::Sin(x) => LazyBuffer::new(x.buf.mapv(|x| x.sin())),
                TargetOp::Sqrt(x) => LazyBuffer::new(x.buf.mapv(|x| x.sqrt())),
                TargetOp::Recip(x) => todo!(),
                TargetOp::Neg(x) => LazyBuffer::new(-x.buf),
                TargetOp::Add(x, y) => LazyBuffer::new(&x.buf+&y.buf),
                TargetOp::Sub(x, y) => LazyBuffer::new(&x.buf-&y.buf),
                TargetOp::Mul(x, y) => LazyBuffer::new(&x.buf*&y.buf),
                TargetOp::Div(x, y) => LazyBuffer::new(&x.buf/&y.buf),
                TargetOp::Mx(x, y) => todo!(),
                TargetOp::Mod(x, y) => todo!(),
                TargetOp::Lt(x, y) => todo!(),
                TargetOp::Sum(x) => todo!(),
                TargetOp::MaxReduce(x) => todo!(),
                TargetOp::MulAcc(x, y, z) => todo!(),
                TargetOp::Where(lazy_buffer, y, z) => todo!(),
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
            },
            Device::Gpu => todo!(),
            Device::Tpu => todo!(),
        }
    }
}