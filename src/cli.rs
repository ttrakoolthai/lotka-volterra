use clap::Parser;

/// Command-line arguments for the parameters of the Lotka-Volterra differential
/// equation. The command-line arguments are used as values in the simulation.
#[derive(Parser, Debug)]
#[command(name = "Lotka-Volterra Simulator")]
#[command(about = "Simulate predator-prey dynamics", long_about = None)]
pub struct Cli {
    /// Prey birth rate (alpha)
    #[arg(short = 'a', long = "alpha")]
    pub alpha: Option<f64>,

    /// Prety death rate (beta)
    #[arg(short = 'b', long = "beta")]
    pub beta: Option<f64>,

    /// Predator birt rate (delta)
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

    /// Start time
    #[arg(short = 't', long = "start")]
    pub t_start: Option<f64>,

    /// End time
    #[arg(short = 'T', long = "end")]
    pub t_end: Option<f64>,

    /// Launch the interactive menu
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,

    /// Launch the interactive GUI
    #[arg(short = 'g', long = "gui")]
    pub gui: bool,

    /// Enable interactive plot mode
    #[arg(short = 'p', long = "interactive-plot")]
    pub interactive_plot: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parsing_defaults() {
        let args = vec!["lotka_volterra"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.gui, false, "GUI mode should be false by default.");
        assert!(cli.alpha.is_none(), "Alpha should be None by default.");
    }

    #[test]
    fn test_cli_parsing_with_arguments() {
        let args = vec!["lotka_volterra", "--alpha", "0.1", "--beta", "0.02"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.alpha.unwrap(), 0.1, "Alpha should be parsed correctly.");
        assert_eq!(cli.beta.unwrap(), 0.02, "Beta should be parsed correctly.");
    }
}
