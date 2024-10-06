use crate::ops::{self, Op};
use ndarray::{Array, IxDyn};
use pyo3::prelude::*;
use std::borrow::Borrow;
use std::sync::Arc;

#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub struct Tensor {
    pub data: Array<f32, IxDyn>, // torch.Tensor slice/reshape returns a view, not a new tensor
    pub requires_grad: bool,
    pub grad: Option<Arc<Tensor>>,
}

#[pymethods]
impl Tensor {
    #[new]
    fn new(data: Vec<f32>, shape: Vec<usize>, requires_grad: bool) -> Self {
        Tensor {
            data: Array::from_shape_vec(IxDyn(&shape), data).unwrap(),
            requires_grad,
            grad: None,
        }
    }

    fn __repr__(&self) -> String {
        format!("Tensor({:?})", self.data)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.data.to_string())
    }

    fn __add__(&self, other: Self) -> Self {
        ops::Add::forward(&[self, &other])
    }

    fn __mul__(&self, other: Self) -> Self {
        ops::Mul::forward(&[self, &other])
    }
}

impl Tensor {
    pub fn from_array(array: Array<f32, IxDyn>, requires_grad: bool) -> Self {
        Tensor {
            data: array,
            requires_grad,
            grad: None,
        }
    }
}

impl<T: Borrow<Tensor>> std::ops::Add<T> for Tensor {
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        ops::Add::forward(&[&self, other.borrow()])
    }
}

impl<T: Borrow<Tensor>> std::ops::Mul<T> for Tensor {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        ops::Mul::forward(&[&self, other.borrow()])
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
