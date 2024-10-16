use crate::{
    generator::{forward, DifferentiableOp},
    interpreter::LazyBuffer,
};
use ndarray::{Array, IxDyn};
use pyo3::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub struct Tensor {
    pub data: LazyBuffer, // tgrs is only backed by ndarray for now
    pub grad: Option<Box<Tensor>>,
    pub requires_grad: bool,
    ctx: bool, // graph
}

#[pymethods]
impl Tensor {
    #[new]
    pub fn new(data: Vec<f32>, shape: Vec<usize>, requires_grad: bool) -> Self {
        Tensor {
            data: LazyBuffer::new(Array::from_shape_vec(IxDyn(&shape), data).unwrap()),
            grad: None,
            requires_grad,
            ctx: true,
        }
    }

    pub fn device(&self) -> String {
        self.data.device()
    }

    #[getter]
    pub fn shape(&self) -> Vec<usize> {
        self.data.shape()
    }

    #[getter]
    pub fn dtype(&self) -> String {
        self.data.dtype()
    }

    fn __repr__(&self) -> String {
        format!("Tensor({:?})", self)
    }

    fn __add__(&self, other: Self) -> Self {
        self.clone() + other
    }

    fn __sub__(&self, other: Self) -> Self {
        self.clone() - other
    }

    fn __mul__(&self, other: Self) -> Self {
        self.clone() * other
    }

    fn __truediv__(&self, other: Self) -> Self {
        self.clone() / other
    }
}

impl Add for Tensor {
    type Output = Tensor;

    fn add(self, other: Self) -> Self::Output {
        Tensor {
            data: forward(DifferentiableOp::Add(self.data, other.data)),
            grad: None,
            requires_grad: self.requires_grad || other.requires_grad,
            ctx: true,
        }
    }
}

impl Sub for Tensor {
    type Output = Tensor;

    fn sub(self, other: Self) -> Self::Output {
        Tensor {
            data: forward(DifferentiableOp::Sub(self.data, other.data)),
            grad: None,
            requires_grad: self.requires_grad || other.requires_grad,
            ctx: true,
        }
    }
}

impl Mul for Tensor {
    type Output = Tensor;

    fn mul(self, other: Self) -> Self::Output {
        Tensor {
            data: forward(DifferentiableOp::Mul(self.data, other.data)),
            grad: None,
            requires_grad: self.requires_grad || other.requires_grad,
            ctx: true,
        }
    }
}

impl Div for Tensor {
    type Output = Tensor;

    fn div(self, other: Self) -> Self::Output {
        Tensor {
            data: forward(DifferentiableOp::Div(self.data, other.data)),
            grad: None,
            requires_grad: self.requires_grad || other.requires_grad,
            ctx: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        let a = Tensor::new(vec![9.0, 9.0, 9.0], vec![3], false);
        let b = Tensor::new(vec![10.0, 10.0, 10.0], vec![3], false);
        let c = a.clone() + b.clone();
        println!("add, {:?}", c);

        let c = a.clone() - b.clone();
        println!("sub, {:?}", c);

        let c = a.clone() * b.clone();
        println!("mul, {:?}", c);

        let c = a.clone() / b.clone();
        println!("div, {:?}", c);
    }
}
