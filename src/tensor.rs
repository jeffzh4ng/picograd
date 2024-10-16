use pyo3::prelude::*;
use crate::interpreter::LazyBuffer;

#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub struct Tensor {
    pub lazy_buf: LazyBuffer, // tgrs is only backed by ndarray for now
    pub grad: Option<Box<Tensor>>,
    pub requires_grad: bool,
}

#[pymethods]
impl Tensor {
    #[new]
    pub fn new(data: Vec<f32>, shape: Vec<usize>, requires_grad: bool) -> Self {
        Tensor {
            lazy_buf: todo!(),
            grad: None,
            requires_grad,
        }
    }

    pub fn device(&self) -> String {
        self.lazy_buf.device()
    }

    #[getter]
    pub fn shape(&self) -> Vec<usize> {
        self.lazy_buf.shape()
    }

    #[getter]
    pub fn dtype(&self) -> String {
        self.lazy_buf.dtype()
    }

    fn __repr__(&self) -> String {
        format!("Tensor({:?})", self)
    }
}

// impl Tensor {
//     pub fn from_array(array: Array<f32, IxDyn>, requires_grad: bool) -> Self {
//         Tensor {
//             data: array,
//             requires_grad,
//             grad: None,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
// }