use std::{
    fs::File,
    io::{BufWriter, Write},
};

use bevy::log::info;
use ron::ser::{to_string_pretty, PrettyConfig};

use crate::genetic_algorithm::{
    operations::Individual,
    write_stat::{write_stat, InstanceStats},
};

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
    testing: bool,
    max_test_count: usize,
    instance_number: usize,
    testing_count: usize,
    instance_stats: InstanceStats,
}

impl<T: Individual + Default> Algorithm<T> {
    pub fn new(
        population_size: usize,
        max_generations: usize,
        max_no_improvement: usize,
        mutation_chance: f32,
        crossover_chance: f32,
        testing: bool,
        max_test_count: usize,
        instance_number: usize,
    ) -> Self {
        Algorithm {
            population_size,
            max_generations,
            max_no_improvement,
            mutation_chance,
            crossover_chance,
            testing,
            max_test_count,
            instance_number,
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

    fn is_testing(&self) -> bool {
        self.testing
    }

    fn should_finish_testing(&self) -> bool {
        self.testing_count >= self.max_test_count
    }

    fn initialize_population(&mut self) {
        self.instance_stats = InstanceStats::default();
        self.current_generation = 0;
        self.current_unbeat_best = (std::f32::MIN, 0);
        self.testing_count += 1;

        self.offspring_population.clear();
        self.new_population.clear();

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

            if self.testing {
                self.instance_stats.best_fitness = chromosome.get_fitness();
                self.instance_stats.best_fitness_sum += chromosome.get_fitness();
                self.instance_stats.generation_count = self.current_generation;
            }
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

        if self.testing {
            write_stat(
                format!(
                    "experiments/population_{}_mutation_{}_crossover_{}/instance_{}/execution_{}/generation_{}.ron",
                    self.new_population.len(),
                    self.mutation_chance,
                    self.crossover_chance,
                    self.instance_number,
                    self.testing_count,
                    generation_count
                ),
                best.get_fitness(),
                median.get_fitness(),
                worst.get_fitness(),
                mean,
                std_dev,
            );

            self.instance_stats.write(format!(
                "experiments/population_{}_mutation_{}_crossover_{}/instance_{}/execution_{}/stats.ron",
                self.new_population.len(),
                self.mutation_chance,
                self.crossover_chance,
                self.instance_number,
                self.testing_count,
            ));
        }
    }

    fn get_should_end(&self) -> bool {
        self.max_generations + 1 == self.current_generation
            || self.current_generation - self.current_unbeat_best.1 > self.max_no_improvement
    }
}
