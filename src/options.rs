#[derive(Debug, StructOpt)]
pub struct Options {
    /// The controller.
    #[structopt(subcommand)]
    pub control: Control,

    /// Silence all log output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increase log verbosity (-v, -vv, -vvv, etc).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,
}

#[derive(Debug, StructOpt)]
pub enum Control {
    /// The bird should be controlled by the player.
    #[structopt(name = "player")]
    Player {},
}
