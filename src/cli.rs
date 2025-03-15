use clap::Parser;

/// Command-line arguments for Lotka-Volterra simulation.
#[derive(Parser, Debug)]
#[command(name = "Lotka-Volterra Simulator")]
#[command(about = "Simulate predator-prey dynamics", long_about = None)]
pub struct Cli {
    /// Prey birth rate (alpha)
    #[arg(long)]
    pub alpha: Option<f64>,

    /// Predation rate (beta)
    #[arg(long)]
    pub beta: Option<f64>,

    /// Predator reproduction rate (delta)
    #[arg(long)]
    pub delta: Option<f64>,

    /// Predator death rate (gamma)
    #[arg(long)]
    pub gamma: Option<f64>,

    /// Run in interactive mode
    #[arg(long)]
    pub interactive: bool,

    /// Launch the interactive GUI
    #[arg(long)]
    pub gui: bool,

    /// Enable interactive plot mode
    #[arg(long)]
    pub interactive_plot: bool,
}
