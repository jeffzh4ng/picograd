use ndarray::{Array, IxDyn};
use std::fmt;

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
    Add(LazyBuffer),
    Sub(LazyBuffer),
    Mul(LazyBuffer),
    Div(LazyBuffer),
    Mx(LazyBuffer),
    Mod(LazyBuffer),
    Lt(LazyBuffer),

    // reduce
    Sum,
    MaxReduce,

    // ternary
    MulAcc(LazyBuffer, LazyBuffer),
    Where(LazyBuffer, LazyBuffer),

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
pub struct LazyBuffer {
    device: Device,
    buf: Array<f32, IxDyn>, // tgrs is only backed by ndarray for now. todo: buf type for other backends?
}

impl LazyBuffer {
    pub fn new(buf: Array<f32, IxDyn>) -> Self {
        LazyBuffer { device: Device::Cpu, buf } // tgrs is only backed by ndarray for now
    }

    pub fn device(&self) -> String {
        self.device.to_string()
    }

    pub fn shape(&self) -> Vec<usize> {
        self.buf.shape().to_vec()
    }

    pub fn dtype(&self) -> String {
        "f32".to_string() // tgrs is only using f32 for now
    }

    pub fn eval(&self, op: TargetOp) -> Self {
      match self.device {
        Device::Cpu => match op {
            TargetOp::Noop => todo!(),
            TargetOp::Exp2 => LazyBuffer::new(self.buf.mapv(|x| x.exp2())),
            TargetOp::Log2 => LazyBuffer::new(self.buf.mapv(|x| x.log2())),
            TargetOp::Cast => todo!(),
            TargetOp::Sin => LazyBuffer::new(self.buf.mapv(|x| x.sin())),
            TargetOp::Sqrt => LazyBuffer::new(self.buf.mapv(|x| x.sqrt())),
            TargetOp::Recip => todo!(),
            TargetOp::Neg => LazyBuffer::new(-&self.buf),
            TargetOp::Add(y) => LazyBuffer::new(&self.buf + &y.buf),
            TargetOp::Sub(y) => LazyBuffer::new(&self.buf - &y.buf),
            TargetOp::Mul(y) => LazyBuffer::new(&self.buf * &y.buf),
            TargetOp::Div(y) => LazyBuffer::new(&self.buf / &y.buf),
            TargetOp::Mx(y) => todo!(),
            TargetOp::Mod(y) => todo!(),
            TargetOp::Lt(y) => todo!(),
            TargetOp::Sum => todo!(),
            TargetOp::MaxReduce => todo!(),
            TargetOp::MulAcc(x, y) => todo!(),
            TargetOp::Where(x, y) => todo!(),
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

#[derive(Clone, PartialEq, Debug)]
pub enum Device { // in ml land this is called "Device"
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
