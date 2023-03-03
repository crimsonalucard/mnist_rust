use std::collections::VecDeque;
use std::iter::zip;
use rand::Rng;
use matrix::{ColumnVector, Matrix};

fn sigmoid(z: f32) -> f32 {
    1.0 / (1.0 + std::f32::consts::E.powf(-z))
}

fn relu(z: f32) -> f32 {
    if z < 0.0 {
        0.0
    } else {
        z
    }
}

fn softmax(z: &ColumnVector, index: usize) -> f32 {
    z.data[index] / z.average()
}

fn mean_square_error(output_vectors: Vec<ColumnVector>, desired_output_vectors: Vec<ColumnVector>) -> f32 {
    let mut acc: f32 = 0.0;
    // for (output, desired) in zip(&output_vectors, &desired_output_vectors) {
    //     acc += (output - desired).magnitude_squared();
    // }

    //faster than above
    let mut result = ColumnVector::new_with_elements(output_vectors[0].data.len(), 0.0);
    for (output, desired) in zip(&output_vectors, &desired_output_vectors) {
        output._sub(desired, &mut result);
        acc += result.magnitude_squared();
    }

    acc / (2.0 * output_vectors.len() as f32)
}

struct NeuralNetwork {
    pub weights: Vec<Matrix>,
    pub activation_values: VecDeque<ColumnVector>,
    pub biases: Vec<ColumnVector>,
}


impl NeuralNetwork {
    pub fn _forward_pass_one_step<'a>(&mut self, layer_index: usize) {
        let input = self.activation_values.pop_front().unwrap();
        let mut result = self.activation_values.pop_front().unwrap();
        let weights = &self.weights[layer_index];
        let bias = &self.biases[layer_index];
        input._mul_matrix(weights, &mut result);
        result += bias;
        self.activation_values.push_front(result);
        self.activation_values.push_front(input);
    }

    fn calculate_all_activation_values(&mut self, input: ColumnVector) {
        self.activation_values[0] = input;
        for index in 0..self.weights.len() {
            NeuralNetwork::_forward_pass_one_step(self, index);
        }
    }

    pub fn new(&self, layer_sizes: &[usize], default_value: Option<f32>) -> NeuralNetwork {
        if layer_sizes.len() < 2 {
            panic!("Cannot generate neural network with less than 2 layers.");
        } else {
            let mut weights = Vec::with_capacity(layer_sizes.len() - 1);
            let mut biases = Vec::with_capacity(layer_sizes.len() - 1);
            let mut activation_values = Vec::with_capacity(layer_sizes.len());
            activation_values.push(ColumnVector::new_with_elements(layer_sizes[0], 0.0));
            for (index, &size) in layer_sizes[0..layer_sizes.len() - 1].iter().enumerate() {
                match default_value {
                    Some(value) => {
                        weights.push(Matrix::new_with_elements(layer_sizes[index + 1], size, value));
                        biases.push(ColumnVector::new_with_elements(layer_sizes[index + 1], value));
                    }
                    None => {
                        let element_gen = |_| {
                            let mut rng = rand::thread_rng();
                            rng.gen()
                        };
                        weights.push(Matrix::new_with_number_generate(layer_sizes[index + 1], size, &element_gen));
                        biases.push(ColumnVector::new_with_number_generator(layer_sizes[index + 1], &element_gen));
                    }
                };
                activation_values.push(ColumnVector::new_with_elements(layer_sizes[index + 1], 0.0));
            }
            NeuralNetwork::new_from_vecs(weights, Some(biases), Some(activation_values))
        }
    }

    pub fn new_from_vecs(weights: Vec<Matrix>, biases: Option<Vec<ColumnVector>>, activation_values: Option<Vec<ColumnVector>>) -> NeuralNetwork {
        let amount_of_weight_matrices = weights.len();
        NeuralNetwork {
            biases: match biases {
                Some(value) => value,
                None => {
                    let mut acc: Vec<ColumnVector> = Vec::with_capacity(amount_of_weight_matrices);
                    for matrix in &weights {
                        acc.push(ColumnVector::new_with_elements(matrix.data[0].len(), 0.0));
                    }
                    acc
                }
            },
            activation_values: match activation_values {
                Some(values) => VecDeque::from(values),
                None => {
                    let mut acc: VecDeque<ColumnVector> = VecDeque::with_capacity(weights.len());
                    for matrix in &weights {
                        acc.push_back(ColumnVector::new_with_elements(matrix.data[0].len(), 0.0));
                    }
                    acc
                }
            },
            weights,
        }
    }
}


#[cfg(test)]
mod tests {
    use matrix::ColumnVector;
    use crate::NeuralNetwork;
    use super::Matrix;

    #[test]
    fn check_feed_forward() {
        let amount_weight_matrices = 20;
        let matrix_size = 5;
        let mut weights: Vec<Matrix> = Vec::with_capacity(amount_weight_matrices);
        for _ in 0..amount_weight_matrices {
            weights.push(Matrix::identity(matrix_size));
        }
        let mut test_nn = NeuralNetwork::new_from_vecs(weights, None, None);
        let input_vector = ColumnVector::new_with_elements(matrix_size, 1.0);
        let input_vector2 = input_vector.clone();
        test_nn.calculate_all_activation_values(input_vector);
        for value in &test_nn.activation_values {

        }

    }

}
