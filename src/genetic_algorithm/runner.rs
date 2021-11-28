use super::operations::Individual;

pub struct AlgorithmRunner<T: Individual> {
    population: Vec<T>,
    fitness_population: Vec<f64>,
    max_generations: usize,
}

impl<T: Individual> AlgorithmRunner<T> {
    pub fn initialize_population(&mut self, population_size: usize) {
        self.population = (0..population_size).map(|_| T::random()).collect();
    }

    pub fn reproduction(&mut self) {
        let mut new_population: Vec<T> = Vec::new();

        self.population = new_population;
    }
}

// // fn start_algorithm() {
// //   let mut population = Population::new(100, 10, 4);
// //   population.initialize();
// //   population.evaluate();
// //   population.crossover();
// //   population.mutate();
// //   population.evaluate();
// //   population.select();
// // }
