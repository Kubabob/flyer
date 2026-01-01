pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // TODO selection
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
    fn select<'a, I>(&self, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

#[cfg(test)]
mod tests {
    use super::*;
}
