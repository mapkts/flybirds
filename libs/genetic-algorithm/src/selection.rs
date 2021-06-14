use crate::*;
use rand::seq::SliceRandom;

pub trait SelectionMethod {
    fn select<'a, I: Individual>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I;
}

#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I: Individual>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I {
        assert!(!population.is_empty());

        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[cfg(test)]
    #[derive(Clone, Debug, PartialEq)]
    pub enum TestIndividual {
        WithChromosome { chromosome: Chromosome },
        WithFitness { fitness: f32 },
    }

    #[cfg(test)]
    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }

    #[cfg(test)]
    impl Individual for TestIndividual {
        fn from_chromosome(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome { chromosome } => chromosome,
                Self::WithFitness { .. } => panic!("not supported for TestIndividual::WithFitness"),
            }
        }

        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => chromosome.iter().sum(),
                Self::WithFitness { fitness } => *fitness,
            }
        }
    }

    #[test]
    fn test_choose_weighted() {
        let method = RouletteWheelSelection::default();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(1.0),
            TestIndividual::new(3.0),
            TestIndividual::new(2.0),
            TestIndividual::new(4.0),
        ];

        let actual_histogram = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(BTreeMap::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as i32).or_default() += 1;

                histogram
            });

        let expected_histogram = BTreeMap::from_iter(vec![(1, 102), (2, 197), (3, 302), (4, 399)]);
        assert_eq!(actual_histogram, expected_histogram);
    }
}
