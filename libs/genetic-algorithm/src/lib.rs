mod chromosome;
mod crossover;
mod mutation;
mod selection;

use chromosome::*;
use crossover::*;
use mutation::*;
use selection::*;

use rand::prelude::*;

pub struct GeneticAlgorithm<S, C, M> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}

impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    M: MutationMethod,
{
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn evolve<'a, I: Individual>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> Vec<I> {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // selection
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                // crossover
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                // mutation
                self.mutation_method.mutate(rng, &mut child);

                // convert `Chromosome` back into `Individual`.
                I::from_chromosome(child)
            })
            .collect()
    }
}

pub trait Individual {
    fn from_chromosome(chromosome: Chromosome) -> Self;
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}
