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
    new_population: Vec<T>,
}

impl<T: Individual> Runnable<T> for Algorithm<T> {
    fn get_population_for_sim(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(
            self.population
                .iter()
                .chain(self.offspring_population.iter()),
        )
    }

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
        let best = self
            .population
            .iter()
            .max_by(|x, y| y.get_fitness().partial_cmp(&x.get_fitness()).unwrap())
            .unwrap();
        let worst = self
            .population
            .iter()
            .min_by(|x, y| y.get_fitness().partial_cmp(&x.get_fitness()).unwrap())
            .unwrap();
        let median = &self.population[self.population.len() / 2];

        let mean = self.population.iter().map(|x| x.get_fitness()).sum::<f32>()
            / self.population.len() as f32;
        let std_dev = self
            .population
            .iter()
            .map(|x| (x.get_fitness() - mean).powi(2))
            .sum::<f32>()
            / self.population.len() as f32;

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
}
