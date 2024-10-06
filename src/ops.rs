use crate::tensor::Tensor;

pub trait Op {
    fn forward(inputs: &[&Tensor]) -> Tensor;
    fn backward(inputs: &[&Tensor], output_grad: Tensor) -> Vec<Tensor>;
}

pub struct Add;
pub struct Mul;

impl Op for Add {
    fn forward(inputs: &[&Tensor]) -> Tensor {
        assert!(
            inputs.len() == 2,
            "Add operation requires exactly two inputs"
        );
        Tensor::from_array(inputs[0].data.clone() + inputs[1].data.clone(), false)
    }

    fn backward(inputs: &[&Tensor], output_grad: Tensor) -> Vec<Tensor> {
        assert!(
            inputs.len() == 2,
            "Add operation requires exactly two inputs"
        );
        vec![output_grad.clone(), output_grad]
    }
}

impl Op for Mul {
    fn forward(inputs: &[&Tensor]) -> Tensor {
        assert!(
            inputs.len() == 2,
            "Mul operation requires exactly two inputs"
        );
        Tensor::from_array(inputs[0].data.clone() * inputs[1].data.clone(), false)
    }

    fn backward(inputs: &[&Tensor], output_grad: Tensor) -> Vec<Tensor> {
        assert!(
            inputs.len() == 2,
            "Mul operation requires exactly two inputs"
        );
        vec![
            Tensor::from_array(inputs[1].data.clone() * output_grad.data.clone(), false),
            Tensor::from_array(inputs[0].data.clone() * output_grad.data.clone(), false),
        ]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
