use super::*;

#[derive(Debug)]
pub struct Animal {
    crate position: na::Point2<f32>,
    crate rotation: na::Rotation2<f32>,
    crate eye: Eye,
    crate brain: nn::Network,
    crate speed: f32,
    crate satiation: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = nn::Network::random(
            rng,
            &[
                // the input layer
                nn::LayerTopology { neurons: eye.cells() },
                // the hidden layer
                nn::LayerTopology { neurons: 2 * eye.cells() },
                // the output layer
                nn::LayerTopology { neurons: 2 },
            ],
        );

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}
