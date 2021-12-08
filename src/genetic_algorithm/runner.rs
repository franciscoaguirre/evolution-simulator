use super::operations::Individual;

#[derive(Default)]
pub struct Algorithm<T: Individual> {
    pub population: Vec<T>,
    pub offspring_population: Vec<T>,
    new_population: Vec<T>,
}

impl<T: Individual> Algorithm<T> {
    /// Initializes the population for an algorithm runner
    /// The population is initialized using the random implementation of the individual
    ///
    /// # Arguments
    ///
    /// * `population_size` - The size of the population
    pub fn initialize_population(&mut self, population_size: usize) {
        self.population = (0..population_size).map(|_| T::random()).collect();
    }

    /// Select the parents for the next generation
    pub fn selection(&mut self, population_size: usize) {
        self.population
            .sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());

        self.population = self.population[0..population_size].to_vec();
    }

    /// Takes the best parents and creates a new population
    /// The new population is created using the breed implementation of the individual
    pub fn reproduction(&mut self) {
        let mut offspring_population: Vec<T> = Vec::new();

        for chunk in self.population.chunks(2) {
            let (mut first_child, mut second_child) = chunk[0].breed(&chunk[1]);

            first_child = first_child.mutate(1.0);
            second_child = second_child.mutate(1.0);
            first_child.correct();
            second_child.correct();
            offspring_population.push(first_child);
            offspring_population.push(second_child);
        }

        self.offspring_population = offspring_population;
    }

    /// Replaces the population with the new population based on the selection and reproduction
    pub fn replacement(&mut self) {
        self.population.clear();
        self.population.append(&mut self.new_population);
    }

    pub fn finished_evaluating(&mut self, chromosome: T) {
        self.new_population.push(chromosome);
    }

    pub fn all_have_finished_evaluating(&self, population_size: usize) -> bool {
        self.new_population.len() == population_size * 2
    }
}
