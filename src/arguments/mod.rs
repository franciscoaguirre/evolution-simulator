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
    // with default value of 100
    #[structopt(short, long, default_value = "100")]
    pub max_generations: usize,

    /// Set number of generations with no improvement for end condition
    // with default value of 100
    #[structopt(short, long, default_value = "10")]
    pub max_no_improvement: usize,

    /// Set chance of mutation for genes
    /// with default value of 0.1
    #[structopt(short = "c", long = "chance", default_value = "0.1")]
    pub mutation_chance: f32,
}
