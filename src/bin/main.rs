use lotka_volterra::{plot_lotka_volterra, LotkaVolterraParams};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for input
    let alpha = prompt_for_float("Enter prey birth rate (alpha): ")?;
    let beta = prompt_for_float("Enter predation rate (beta): ")?;
    let delta = prompt_for_float("Enter predator reproduction rate (delta): ")?;
    let gamma = prompt_for_float("Enter predator death rate (gamma): ")?;

    // Create the LotkaVolterraParams struct
    let params = LotkaVolterraParams {
        alpha,
        beta,
        delta,
        gamma,
    };

    // Initial conditions: [prey, predator]
    let y0 = [40.0, 9.0];

    // Time parameters
    let t0 = 0.0;
    let t_end = 200.0;
    let step = 0.1;

    // Plot and save the results
    plot_lotka_volterra(params, y0, t0, t_end, step, "lotka_volterra.png")?;

    println!("Plot saved as lotka_volterra.png");
    Ok(())
}

/// Helper function to prompt the user for a floating-point number.
fn prompt_for_float(prompt: &str) -> Result<f64, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?; // Ensure the prompt is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Parse the input into an f64
    let value: f64 = input.trim().parse()?;
    Ok(value)
}
