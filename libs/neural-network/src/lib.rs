use rand::{Rng, RngCore};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("got {got} inputs, but {expected} inputs were expected")]
    MismatchedInputSize { got: usize, expected: usize },
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    // pub fn new(layers: Vec<Layer>) -> Self {
    //     Self { layers }
    // }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| {
                neuron
                    .propagate(&inputs)
                    .expect("Error happened in neuron propagation")
            })
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.random_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
    pub fn propagate(&self, inputs: &[f32]) -> Result<f32> {
        if inputs.len() != self.weights.len() {
            return Err(Error::MismatchedInputSize {
                got: inputs.len(),
                expected: self.weights.len(),
            });
        }

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        Ok(relu(self.bias + output))
    }
}

pub fn relu(value: f32) -> f32 {
    value.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_neuron() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383933, 0.81812596, 0.26284885, 0.5238805].as_ref()
        );
    }

    #[test]
    fn propagate_neuron() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]).unwrap(), 0.0,);

        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]).unwrap(),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        )
    }
}
