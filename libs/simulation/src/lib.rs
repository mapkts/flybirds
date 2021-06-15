use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::*;

#[derive(Debug)]
pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            // For simplicity, we just hardcode the `num_animals` and `num_foods` for now.
            world: World::random(rng, 40, 60),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position(), &food.position());

                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore, num_animals: u32, num_foods: u32) -> Self {
        let animals = (0..num_animals).map(|_| Animal::random(rng)).collect();
        let foods = (0..num_foods).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
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

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    // Range of field of view.
    const FOV_RANGE: f32 = 0.25;

    // Angle of field of view.
    const FOV_ANGLE: f32 = PI + FRAC_PI_4;

    // Photoreceptors in a single eye.
    const CELLS: usize = 9;

    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0. && fov_angle > 0. && cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    // pub fn process_vision() -> Vec<f32> {
    //     for food in foods {
    //         let vec = food.position - position;
    //         let dist = vec.norm();
    //         if dist >= self.fov_range {
    //             continue;
    //         }
    //     }
    // }

    pub fn cells(&self) -> usize {
        self.cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(Self::FOV_RANGE, Self::FOV_ANGLE, Self::CELLS)
    }
}
