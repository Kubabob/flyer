use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::Network,
    pub(crate) satiation: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = nn::Network::random(
            rng,
            &[
                // Input layer
                nn::LayerTopology {
                    neurons: eye.cells(),
                },
                // Hidden layer
                nn::LayerTopology {
                    neurons: 2 * eye.cells(),
                },
                // Output layer (speed, rotation)
                nn::LayerTopology { neurons: 2 },
            ],
        );

        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eye: eye,
            brain: brain,
            satiation: 0,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
