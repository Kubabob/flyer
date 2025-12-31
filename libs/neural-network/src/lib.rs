use rand::Rng;
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

    pub fn random(layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
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

    pub fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size))
            .collect();

        Self { neurons }
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn random(input_size: usize) -> Self {
        let mut rng = rand::rng();
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
