use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Options")]
pub struct Opt {
    /// Activate headless mode
    // short and long flags (-h, --headless)
    #[structopt(short, long)]
    pub headless: bool,

    /// Use speciesism
    // short and long flags (-s, --speciesism)
    #[structopt(short, long)]
    pub speciesism: bool,

    /// Set number of max generations
    #[structopt(long, default_value = "100")]
    pub max_generations: usize,

    /// Set number of generations with no improvement for end condition
    #[structopt(long, default_value = "10")]
    pub max_no_improvement: usize,

    /// Set chance of mutation for genes
    /// with default value of 0.1
    #[structopt(short = "mc", long = "mutation", default_value = "0.1")]
    pub mutation_chance: f32,

    /// Set chance for crossover
    /// with default value of 0.5
    #[structopt(short = "cc", long = "crossover", default_value = "0.5")]
    pub crossover_chance: f32,

    /// Set number of population
    /// with default value of 100
    #[structopt(long, default_value = "100")]
    pub population_size: usize,

    /// Whether or not the playground should be run instead of the ga
    #[structopt(long)]
    pub playground: bool,
}
