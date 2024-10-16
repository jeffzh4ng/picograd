use ndarray::{Array, IxDyn};
use crate::targets::{Device, TargetOp};

#[derive(Clone, PartialEq, Debug)]
pub struct LazyBuffer {
    device: Device,
    pub buf: Array<f32, IxDyn>, // todo: type buf and device together
}

impl LazyBuffer {
    pub fn new(buf: Array<f32, IxDyn>) -> Self {
        LazyBuffer { buf, device: Device::Cpu } // tgrs is only backed by ndarray for now
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
        self.device.eval(op) // tgrs' lazybuf is only passthrough for now
    }
}
