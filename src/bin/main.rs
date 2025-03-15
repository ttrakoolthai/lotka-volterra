use clap::Parser;
use lotka_volterra::{
    cli::Cli,
    interactive::interactive_mode,
    models::*,
    plot::*,
    solver::*,
    gui::launch_gui, // Import the launch_gui function
    error::SimulationError,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Determine parameters: use CLI args if provided, otherwise enter interactive mode
    let params = if cli.interactive
        || cli.alpha.is_none()
        || cli.beta.is_none()
        || cli.delta.is_none()
        || cli.gamma.is_none()
    {
        interactive_mode()?
    } else {
        let params = LotkaVolterraParameters {
            alpha: cli.alpha.unwrap(),
            beta: cli.beta.unwrap(),
            delta: cli.delta.unwrap(),
            gamma: cli.gamma.unwrap(),
        };
        params
    };

    // Run the simulation based on the selected mode
    if cli.gui {
        println!("🚀 Launching interactive GUI...");
        launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;
    } else if cli.interactive_plot {
        println!("📊 Launching Interactive Plot...");
        launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;
    } else {
        println!("✅ Running simulation...");

        // Initial conditions: [prey, predator]
        let y0 = [40.0, 9.0];
        let t0 = 0.0;
        let t_end = 200.0;
        let step = 0.1;

        // Run the simulation
        let (times, prey, predators) = solve_lotka_volterra(params, y0, t0, t_end, step)
            .map_err(|e| SimulationError::PlotError(e.to_string()))?;

        // Plot results
        plot_lotka_volterra(&times, &prey, &predators, "lotka_volterra.png")
            .map_err(|e| SimulationError::PlotError(e.to_string()))?;

        println!("📊 Plot saved as lotka_volterra.png");
    }

    Ok(())
}
