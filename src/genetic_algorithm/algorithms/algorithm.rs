use std::{fs::File, io::Write};

use ron::ser::{to_string_pretty, PrettyConfig};

use crate::genetic_algorithm::operations::Individual;

use super::runner::Runnable;

#[derive(Default)]
pub struct Algorithm<T: Individual> {
    pub population: Vec<T>,
    pub offspring_population: Vec<T>,
    new_population: Vec<T>,
}

impl<T: Individual> Runnable<T> for Algorithm<T> {
    fn initialize_population(&mut self, population_size: usize) {
        self.population = (0..population_size).map(|_| T::random()).collect();
    }

    fn selection(&mut self, population_size: usize) {
        self.population
            .sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());

        self.population = self.population[0..population_size].to_vec();
    }

    fn reproduction(&mut self) {
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

    fn replacement(&mut self) {
        self.population.clear();
        self.population.append(&mut self.new_population);

        self.new_population.clear();
    }

    fn finished_evaluating(&mut self, chromosome: T) {
        self.new_population.push(chromosome);
    }

    fn all_have_finished_evaluating(&self, population_size: usize) -> bool {
        self.new_population.len() == population_size * 2
    }

    fn save_results(&self, generation_count: usize) {
        let pretty_config = PrettyConfig::default();
        let best = to_string_pretty(
            self.population
                .iter()
                .max_by(|x, y| x.get_fitness().partial_cmp(&y.get_fitness()).unwrap())
                .unwrap(),
            pretty_config.clone(),
        )
        .unwrap();
        let worst = to_string_pretty(
            self.population
                .iter()
                .min_by(|x, y| x.get_fitness().partial_cmp(&y.get_fitness()).unwrap())
                .unwrap(),
            pretty_config.clone(),
        )
        .unwrap();
        let mean = to_string_pretty(
            &self.population[self.population.len() / 2],
            pretty_config.clone(),
        )
        .unwrap();

        let mut buffer =
            File::create(format!("results_generation_{}.ron", generation_count)).unwrap();
        buffer.write(b"Best: ").unwrap();
        buffer.write(best.as_bytes()).unwrap();
        buffer.write(b"\n").unwrap();
        buffer.write(b"Worst: ").unwrap();
        buffer.write(worst.as_bytes()).unwrap();
        buffer.write(b"\n").unwrap();
        buffer.write(b"Mean: ").unwrap();
        buffer.write(mean.as_bytes()).unwrap();
    }
}
