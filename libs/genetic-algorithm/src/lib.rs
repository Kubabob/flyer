use rand::seq::{IndexedRandom, SliceRandom};
use rand::{Rng, RngCore};

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // TODO selection
                let parent_a = self.selection_method.select(rng, population);
                let parent_b = self.selection_method.select(rng, population);

                // TODO crossover
                // TODO mutation
                todo!();
            })
            .collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Got an empty population")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[derive(Debug, Clone)]
    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
    }

    mod selection_methods {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use super::*;

        #[test]
        fn roulette_wheel_selection() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let population = vec![
                TestIndividual::new(2.0),
                TestIndividual::new(1.0),
                TestIndividual::new(4.0),
                TestIndividual::new(3.0),
            ];

            let mut actual_histogram = BTreeMap::new();
            for _ in 0..50 {
                let fitness = RouletteWheelSelection
                    .select(&mut rng, &population)
                    .fitness() as i32;

                *actual_histogram.entry(fitness).or_insert(0) += 1;
            }

            let expected_histogram = BTreeMap::from_iter([(1, 4), (2, 10), (3, 14), (4, 22)]);

            assert_eq!(actual_histogram, expected_histogram);
        }
    }
}
