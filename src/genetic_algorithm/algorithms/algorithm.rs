use std::{
    fs::File,
    io::{BufWriter, Write},
};

use ron::ser::{to_string_pretty, PrettyConfig};

use crate::genetic_algorithm::operations::Individual;

use super::runner::Runnable;

#[derive(Default)]
pub struct Algorithm<T: Individual> {
    pub population: Vec<T>,
    pub offspring_population: Vec<T>,
    max_generations: usize,
    max_no_improvement: usize,
    current_unbeat_best: (f32, usize),
    new_population: Vec<T>,
    current_generation: usize,

    mutation_chance: f32,
    crossover_chance: f32,
    population_size: usize,
}

impl<T: Individual + Default> Algorithm<T> {
    pub fn new(
        population_size: usize,
        max_generations: usize,
        max_no_improvement: usize,
        mutation_chance: f32,
        crossover_chance: f32,
    ) -> Self {
        Algorithm {
            population_size,
            max_generations,
            max_no_improvement,
            mutation_chance,
            crossover_chance,
            ..Default::default()
        }
    }
}

impl<T: Individual> Runnable<T> for Algorithm<T> {
    fn get_population_for_sim(&self) -> Vec<T> {
        self.population
            .iter()
            .chain(self.offspring_population.iter())
            .cloned()
            .collect()
    }

    fn initialize_population(&mut self) {
        self.current_generation = 0;
        self.current_unbeat_best = (std::f32::MIN, 0);
        self.population = (0..self.population_size).map(|_| T::random()).collect();
    }

    fn selection(&mut self) {
        self.population
            .sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());

        self.population = self.population[0..self.population_size].to_vec();
    }

    fn reproduction(&mut self) {
        self.current_generation += 1;
        let mut offspring_population: Vec<T> = Vec::new();

        for chunk in self.population.chunks(2) {
            let (mut first_child, mut second_child) =
                chunk[0].breed(&chunk[1], self.crossover_chance);

            first_child = first_child.mutate(self.mutation_chance);
            second_child = second_child.mutate(self.mutation_chance);
            first_child.correct();
            second_child.correct();
            offspring_population.push(first_child);
            offspring_population.push(second_child);
        }

        self.offspring_population = offspring_population;
    }

    fn replacement(&mut self) {
        self.population.clear();
        self.population.append(&mut self.new_population);

        self.new_population.clear();
    }

    fn finished_evaluating(&mut self, chromosome: T) {
        if chromosome.get_fitness() > self.current_unbeat_best.0 {
            self.current_unbeat_best = (chromosome.get_fitness(), self.current_generation);
        }

        self.new_population.push(chromosome);
    }

    fn all_have_finished_evaluating(&self) -> bool {
        self.new_population.len() == self.population_size * 2
    }

    fn save_results(&self, generation_count: usize) {
        let pretty_config = PrettyConfig::default();
        let best = self
            .new_population
            .iter()
            .max_by(|x, y| x.get_fitness().partial_cmp(&y.get_fitness()).unwrap())
            .unwrap();
        let worst = self
            .new_population
            .iter()
            .min_by(|x, y| x.get_fitness().partial_cmp(&y.get_fitness()).unwrap())
            .unwrap();
        let median = &self.new_population[self.new_population.len() / 2];

        let mean = self
            .new_population
            .iter()
            .map(|x| x.get_fitness())
            .sum::<f32>()
            / self.new_population.len() as f32;
        let std_dev = self
            .new_population
            .iter()
            .map(|x| (x.get_fitness() - mean).powi(2))
            .sum::<f32>()
            / self.new_population.len() as f32;

        let mean_string = format!("{:.2}", mean);
        let std_dev_string = format!("{:.2}", std_dev);

        println!(
            "Generation {}: Best: {}, Worst: {}, Median: {}, Mean: {}, StdDev: {}",
            generation_count,
            best.get_fitness(),
            worst.get_fitness(),
            median.get_fitness(),
            mean_string,
            std_dev_string
        );

        let buffer = File::create(format!("results_generation_{}.ron", generation_count)).unwrap();
        let mut stream = BufWriter::new(buffer);
        stream.write(b"Best: ").unwrap();
        stream
            .write(
                to_string_pretty(best, pretty_config.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        stream.write(b"\n").unwrap();
        stream.write(b"Worst: ").unwrap();
        stream
            .write(
                to_string_pretty(worst, pretty_config.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        stream.write(b"\n").unwrap();
        stream.write(b"Mean: ").unwrap();
        stream
            .write(
                to_string_pretty(median, pretty_config.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        stream.write(b"\n").unwrap();
        stream.write(b"Mean: ").unwrap();
        stream.write(mean_string.as_bytes()).unwrap();
        stream.write(b"\n").unwrap();
        stream.write(b"Std. Dev.: ").unwrap();
        stream.write(std_dev_string.as_bytes()).unwrap();
        stream.write(b"\n").unwrap();
        stream.flush().unwrap();
    }

    fn get_should_end(&self) -> bool {
        self.max_generations + 1 == self.current_generation
            || self.current_generation - self.current_unbeat_best.1 > self.max_no_improvement
    }
}
