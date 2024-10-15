use ndarray::{Array, IxDyn};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub struct Tensor {
    pub data: Array<f32, IxDyn>, // tgrs is only backed by ndarray for now
    pub grad: Option<Box<Tensor>>,
    pub requires_grad: bool,
}

#[pymethods]
impl Tensor {
    #[new]
    fn new(data: Vec<f32>, shape: Vec<usize>, requires_grad: bool) -> Self {
        Tensor {
            data: Array::from_shape_vec(IxDyn(&shape), data).unwrap(),
            grad: None,
            requires_grad,
        }
    }

    // fn __repr__(&self) -> String {
    //     format!("Tensor({:?})", self.data)
    // }

    // fn __str__(&self) -> String {
    //     format!("{:?}", self.data.to_string())
    // }
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