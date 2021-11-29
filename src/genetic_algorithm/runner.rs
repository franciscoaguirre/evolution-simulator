use super::operations::Individual;

pub struct Algorithm<T: Individual> {
    population: Vec<T>,
    new_population: Vec<T>,
    parent_population: Vec<T>,
    fitness_population: Vec<f64>,
    max_generations: usize,
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
    pub fn selection(&mut self) {
        let mut population = self
            .population
            .iter()
            .zip(self.fitness_population.iter())
            .collect::<Vec<_>>();

        population.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        self.parent_population = population.iter().map(|x| x.0.clone()).collect::<Vec<T>>()
            [0..population.len() / 2]
            .to_vec();
    }

    /// Takes the best parents and creates a new population
    /// The new population is created using the breed implementation of the individual
    pub fn reproduction(&mut self) {
        let mut new_population: Vec<T> = Vec::new();

        for chunk in self.parent_population.chunks(2) {
            let (mut first_child, mut second_child) = chunk[0].breed(&chunk[1]);

            first_child = first_child.mutate(1.0);
            second_child = second_child.mutate(1.0);

            new_population.push(first_child);
            new_population.push(second_child);
        }

        self.new_population = new_population;
    }

    /// Evaluates performances of population and stores them in fitness_population
    pub fn evaluate(&mut self) {
        // self.fitness_population = self
        //     .population
        //     .iter()
        //     .map(|x| x.fitness())
        //     .collect::<Vec<f64>>();
    }

    /// Replaces the population with the new population based on the selection and reproduction
    pub fn replacement(&mut self) {
        // TODO: Replacement should take percentage of parents and percentage of offsprings
        self.population = self.new_population.clone();
    }
}
