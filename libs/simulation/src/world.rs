use super::*;

#[derive(Debug)]
pub struct World {
    crate animals: Vec<Animal>,
    crate foods: Vec<Food>,
}

impl World {
    pub fn random(
        rng: &mut dyn RngCore,
        num_animals: u32,
        num_foods: u32,
    ) -> Self {
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
