use lotka_volterra::{plot_lotka_volterra, LotkaVolterraParams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define parameters
    let params = LotkaVolterraParams {
        alpha: 0.1,
        beta: 0.02,
        delta: 0.01,
        gamma: 0.1,
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
