use rand::{Rng, RngCore};
use std::iter::once;
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

impl LayerTopology {
    pub fn new(neurons: usize) -> Self {
        Self { neurons }
    }
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
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

    mod neuron {
        use super::*;

        #[test]
        fn random_generation() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            assert_relative_eq!(neuron.bias, -0.6255188);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383933, 0.81812596, 0.26284885, 0.5238805].as_ref()
            );
        }

        #[test]
        fn propagate() {
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

    mod layer {
        use approx::assert_relative_eq;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::{Layer, Neuron};

        impl approx::AbsDiffEq for Neuron {
            type Epsilon = f32;

            fn default_epsilon() -> f32 {
                f32::EPSILON
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: f32) -> bool {
                self.bias.abs_diff_eq(&other.bias, epsilon)
                    && self.weights.len() == other.weights.len()
                    && self
                        .weights
                        .iter()
                        .zip(other.weights.iter())
                        .all(|(a, b)| a.abs_diff_eq(b, epsilon))
            }
        }

        impl approx::RelativeEq for Neuron {
            fn default_max_relative() -> f32 {
                f32::default_max_relative()
            }

            fn relative_eq(&self, other: &Self, epsilon: f32, max_relative: f32) -> bool {
                self.bias.relative_eq(&other.bias, epsilon, max_relative)
                    && self.weights.len() == other.weights.len()
                    && self
                        .weights
                        .iter()
                        .zip(other.weights.iter())
                        .all(|(a, b)| a.relative_eq(b, epsilon, max_relative))
            }
        }

        #[test]
        fn random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 3, 2);

            let mut rng2 = ChaCha8Rng::from_seed(Default::default());
            let expected_neurons = vec![Neuron::random(&mut rng2, 3), Neuron::random(&mut rng2, 3)];

            assert_eq!(layer.neurons.len(), 2);
            assert_relative_eq!(layer.neurons.as_slice(), expected_neurons.as_slice());
        }

        #[test]
        fn propagate() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 3, 2);

            assert_relative_eq!(
                layer.propagate(vec![-10.0, -10.0, 5.0]).as_slice(),
                vec![0.0, 1.3577781].as_slice()
            );
        }
    }

    mod network {
        use approx::assert_relative_eq;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::{Layer, LayerTopology, Network, Neuron};

        impl approx::AbsDiffEq for Layer {
            type Epsilon = f32;

            fn default_epsilon() -> f32 {
                f32::EPSILON
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: f32) -> bool {
                self.neurons.len() == other.neurons.len()
                    && self
                        .neurons
                        .iter()
                        .zip(other.neurons.iter())
                        .all(|(a, b)| a.abs_diff_eq(b, epsilon))
            }
        }

        impl approx::RelativeEq for Layer {
            fn default_max_relative() -> f32 {
                f32::default_max_relative()
            }

            fn relative_eq(&self, other: &Self, epsilon: f32, max_relative: f32) -> bool {
                self.neurons.len() == other.neurons.len()
                    && self
                        .neurons
                        .iter()
                        .zip(other.neurons.iter())
                        .all(|(a, b)| a.relative_eq(b, epsilon, max_relative))
            }
        }

        #[test]
        fn random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let network = Network::random(
                &mut rng,
                &[
                    LayerTopology { neurons: 3 },
                    LayerTopology { neurons: 4 },
                    LayerTopology { neurons: 5 },
                ],
            );

            let mut rng2 = ChaCha8Rng::from_seed(Default::default());

            assert_relative_eq!(
                network.layers.as_slice(),
                [
                    Layer::random(&mut rng2, 3, 4),
                    Layer::random(&mut rng2, 4, 5)
                ]
                .as_ref()
            );
        }

        #[test]
        fn propagate() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let network = Network::random(
                &mut rng,
                &[
                    LayerTopology { neurons: 3 },
                    LayerTopology { neurons: 4 },
                    LayerTopology { neurons: 5 },
                ],
            );

            assert_relative_eq!(
                network.propagate(vec![-10.0, -10.0, 5.0]).as_slice(),
                vec![0.0, 0.0, 2.2639842, 1.7549752, 2.1092079].as_slice()
            );
        }

        #[test]
        fn weights() {
            let network = Network {
                layers: vec![
                    Layer {
                        neurons: vec![Neuron {
                            bias: 0.1,
                            weights: vec![0.2, 0.3, 0.4],
                        }],
                    },
                    Layer {
                        neurons: vec![Neuron {
                            bias: 0.5,
                            weights: vec![0.6, 0.7, 0.8],
                        }],
                    },
                ],
            };

            let actual: Vec<_> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }

        #[test]
        fn from_weights() {
            let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().collect();

            assert_relative_eq!(actual.as_slice(), weights.as_slice());
        }
    }
}
