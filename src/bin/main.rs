use lotka_volterra::{plot_lotka_volterra, LotkaVolterraParams, prompt_for_float};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let alpha = prompt_for_float("Enter prey birth rate (alpha): ")?;
    let beta = prompt_for_float("Enter predation rate (beta): ")?;
    let delta = prompt_for_float("Enter predator reproduction rate (delta): ")?;
    let gamma = prompt_for_float("Enter predator death rate (gamma): ")?;

    let params = LotkaVolterraParams {
        alpha,
        beta,
        delta,
        gamma,
    };

    let y0 = [40.0, 9.0];
    let t0 = 0.0;
    let t_end = 200.0;
    let step = 0.1;

    plot_lotka_volterra(params, y0, t0, t_end, step, "lotka_volterra.png")?;

    println!("Plot saved as lotka_volterra.png");
    Ok(())
}
