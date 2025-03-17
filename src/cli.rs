use clap::Parser;

/// Command-line arguments for Lotka-Volterra simulation.
#[derive(Parser, Debug)]
#[command(name = "Lotka-Volterra Simulator")]
#[command(about = "Simulate predator-prey dynamics", long_about = None)]
pub struct Cli {
    /// Prey birth rate (alpha)
    #[arg(short = 'a', long = "alpha")]
    pub alpha: Option<f64>,

    /// Predation rate (beta)
    #[arg(short = 'b', long = "beta")]
    pub beta: Option<f64>,

    /// Predator reproduction rate (delta)
    #[arg(short = 'd', long = "delta")]
    pub delta: Option<f64>,

    /// Predator death rate (gamma)
    #[arg(short = 'c', long = "gamma")]
    pub gamma: Option<f64>,

    /// Initial prey population
    #[arg(short = 'P', long = "initial-prey")]
    pub initial_prey: Option<f64>,

    /// Initial predator population
    #[arg(short = 'Q', long = "initial-predator")]
    pub initial_predator: Option<f64>,

    /// Run in interactive mode
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,

    /// Launch the interactive GUI
    #[arg(short = 'g', long = "gui")]
    pub gui: bool,

    /// Enable interactive plot mode
    #[arg(short = 'p', long = "interactive-plot")]
    pub interactive_plot: bool,
}
