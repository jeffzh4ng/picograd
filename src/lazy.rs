use ndarray::{Array, IxDyn};
use crate::targets::{cpu_eval, Device, TargetOp};

#[derive(Clone, PartialEq, Debug)]
pub struct LazyBuffer {
    buf: Array<f32, IxDyn>, // todo: type buf and device together
    device: Device,
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

    fn eval(&self, op: TargetOp, inputs: Vec<LazyBuffer>) -> Self {
        // todo: implement eval on Device type?
        match self.device {
            Device::Cpu => cpu_eval(op, inputs),
            Device::Gpu => todo!(),
            Device::Tpu => todo!(),
        }
    }
}