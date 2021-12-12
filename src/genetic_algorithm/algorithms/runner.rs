use crate::genetic_algorithm::operations::Individual;

pub trait Runnable<T: Individual> {
    /// Initializes the population for an algorithm runner
    /// The population is initialized using the random implementation of the individual
    ///
    /// # Arguments
    ///
    /// * `population_size` - The size of the population
    fn initialize_population(&mut self);

    /// Select the parents for the next generation
    fn selection(&mut self);

    /// Takes the best parents and creates a new population
    /// The new population is created using the breed implementation of the individual
    fn reproduction(&mut self);

    /// Replaces the population with the new population based on the selection and reproduction
    fn replacement(&mut self);

    /// Reportes a chromosome to the algorithm with fitness result
    fn finished_evaluating(&mut self, chromosome: T);

    /// Returns true if all the population has been evaluated based on population_size
    fn all_have_finished_evaluating(&self) -> bool;

    /// Saves the generation results into a file called results_generation_{number_of_generation}.ron
    fn save_results(&self, generation_count: usize);

    /// Returns the population inside a vec
    fn get_population_for_sim(&self) -> Vec<T>;

    fn get_should_end(&self) -> bool;
}
