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
}
