#![feature(crate_visibility_modifier)]

mod animal;
mod animal_individual;
mod eye;
mod food;
mod world;

pub use animal::*;
pub use animal_individual::*;
pub use eye::*;
pub use food::*;
pub use world::*;

use genetic_algorithm as ga;
use nalgebra as na;
use neural_network as nn;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

/// Minimum speed of a bird.
///
/// Keeping it above zero prevents birds from getting stuck in one place.
const SPEED_MIN: f32 = 0.001;

/// Maximum speed of a bird.
///
/// Keeping it "sane" prevents birds from accelerating up to infinity.
const SPEED_MAX: f32 = 0.005;

/// Speed acceleration.
///
/// Determines how much the brain can affect bird's speed during one step.
const SPEED_ACCELRATION: f32 = 0.2;

/// Rotation acceleration.
const ROTATION_ACCELRATION: f32 = FRAC_PI_2;

/// How much steps have to occur before we push data into the genetic algorithm.
const GENERATION_LENGTH: usize = 2500;

#[derive(Debug)]
pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<
        ga::RouletteWheelSelection,
        ga::UniformCrossover,
        ga::GaussianMutation,
    >,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng, 40, 60);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            self.evolve(rng);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // step 1: prepare birds to be sent into the genetic algorithm.
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // step 2: evolve birds.
        let evolved_population = self.ga.evolve(rng, &current_population);

        // step 3: bring birds back from the genetic algorithm.
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        // step 4: restart foods.
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position +=
                animal.rotation * na::Vector2::new(animal.speed, 0.0);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance =
                    na::distance(&animal.position(), &food.position());

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.foods,
            );

            let response = animal.brain.propagate(vision);

            let speed =
                response[0].clamp(-SPEED_ACCELRATION, SPEED_ACCELRATION);

            let rotation =
                response[1].clamp(-ROTATION_ACCELRATION, ROTATION_ACCELRATION);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            animal.rotation =
                na::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }
}
