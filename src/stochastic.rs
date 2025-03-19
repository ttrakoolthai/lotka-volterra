use eframe::egui;
use egui_plot::{Corner, Legend, Line, Plot, PlotPoints};
use rand::Rng;

/// Structure to hold Lotka-Volterra parameters.
#[derive(Clone)]
pub struct LotkaVolterraParams {
    pub alpha: f64,            // Prey birth rate
    pub beta: f64,             // Predation rate
    pub gamma: f64,            // Predator birth rate
    pub delta: f64,            // Predator death rate
    pub dt: f64,               // Time step
    pub n: usize,              // Number of steps
    pub initial_prey: f64,     // Initial prey population
    pub initial_predator: f64, // Initial predator population
}

/// Simulates the stochastic Lotka-Volterra system and returns phase plot data.
pub fn solve_stochastic_lotka_volterra(params: &LotkaVolterraParams) -> Vec<[f64; 2]> {
    let mut rng = rand::rng();
    let mut prey = params.initial_prey;
    let mut predators = params.initial_predator;
    let mut phase_points = Vec::new();

    phase_points.push([prey, predators]);

    for _ in 0..params.n {
        let x: f64 = rng.random();

        if x <= params.alpha * prey * params.dt {
            // Prey reproduces
            prey += 1.0;
        } else if x <= params.alpha * prey * params.dt + params.delta * predators * params.dt {
            // Predators die
            predators -= 1.0;
        } else if x
            <= params.alpha * prey * params.dt
                + params.delta * predators * params.dt
                + params.beta * prey * predators * params.dt
        {
            // Prey die
            prey -= 1.0;
        } else if x
            <= params.alpha * prey * params.dt
                + params.delta * predators * params.dt
                + params.beta * prey * predators * params.dt
                + params.gamma * prey * predators * params.dt
        {
            // Predators reproduce
            predators += 1.0;
        }

        phase_points.push([prey, predators]);
    }

    phase_points
}

/// Stores stochastic phase plot data
struct StochasticLotkaVolterraApp {
    params: LotkaVolterraParams,
    phase_points: Vec<[f64; 2]>,
}

impl StochasticLotkaVolterraApp {
    fn new() -> Self {
        let params = LotkaVolterraParams {
            alpha: 0.01,
            beta: 0.00001,
            gamma: 0.00001,
            delta: 0.01,
            dt: 0.001,
            n: 1000000,
            initial_prey: 2000.0,
            initial_predator: 2000.0,
        };

        let phase_points = solve_stochastic_lotka_volterra(&params);

        Self {
            params,
            phase_points,
        }
    }

    fn update_simulation(&mut self) {
        self.phase_points = solve_stochastic_lotka_volterra(&self.params);
    }
}

impl eframe::App for StochasticLotkaVolterraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Stochastic Lotka-Volterra Phase Plot");
            });
            ui.add_space(10.0);

            // Plot the stochastic phase plot
            if !self.phase_points.is_empty() {
                let stochastic_line = Line::new(PlotPoints::from_iter(
                    self.phase_points.iter().map(|&[x, y]| [x, y]),
                ))
                .name("Stochastic Phase Plot");

                Plot::new("stochastic_phase_plot")
                    .view_aspect(2.0)
                    .legend(Legend::default().position(Corner::LeftTop))
                    .x_axis_label("Prey Population")
                    .y_axis_label("Predator Population")
                    .show(ui, |plot_ui| {
                        plot_ui.line(stochastic_line);
                    });
            } else {
                ui.label("No data available for stochastic phase plot.");
            }

            // Add space before parameter section to push it to the bottom
            ui.add_space(20.0);

            // Collapsible parameter section
            egui::CollapsingHeader::new("Adjust Parameters")
                .default_open(true)
                .show(ui, |ui| {
                    ui.columns(2, |columns| {
                        let mut updated = false;

                        // Left column: Initial Conditions + Time Step + Steps
                        columns[0].vertical(|ui| {
                            ui.label("Initial Conditions & Time Settings:");
                            updated |= ui
                                .add(
                                    egui::Slider::new(&mut self.params.initial_prey, 0.0..=2000.0)
                                        .text("Initial Prey"),
                                )
                                .changed();
                            updated |= ui
                                .add(
                                    egui::Slider::new(
                                        &mut self.params.initial_predator,
                                        0.0..=2000.0,
                                    )
                                    .text("Initial Predator"),
                                )
                                .changed();
                            updated |= ui
                                .add(
                                    egui::Slider::new(&mut self.params.dt, 0.0..=0.001)
                                        .text("dt (Time Step)"),
                                )
                                .changed();
                            updated |= ui
                                .add(
                                    egui::Slider::new(&mut self.params.n, 0..=1000000)
                                        .text("n (Steps)"),
                                )
                                .changed();
                        });

                        // Right column: Model Parameters
                        columns[1].vertical(|ui| {
                            ui.label("Model Parameters:");
                            updated |= ui
                                .add(
                                    egui::Slider::new(&mut self.params.alpha, 0.0..=0.01)
                                        .text("Alpha (Prey Birth Rate)"),
                                )
                                .changed();
                            updated |= ui
                                .add(
                                    egui::Slider::new(&mut self.params.beta, 0.0..=0.0001)
                                        .text("Beta (Predation Rate)"),
                                )
                                .changed();
                            updated |= ui
                                .add(
                                    egui::Slider::new(&mut self.params.gamma, 0.0..=0.0001)
                                        .text("Gamma (Predator Birth Rate)"),
                                )
                                .changed();
                            updated |= ui
                                .add(
                                    egui::Slider::new(&mut self.params.delta, 0.0..=0.01)
                                        .text("Delta (Predator Death Rate)"),
                                )
                                .changed();
                        });

                        if updated {
                            // Re-run the simulation when sliders change
                            self.update_simulation();
                        }
                    });
                });
        });
    }

    /// Handle window close event
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("\nSimulation terminating...");
    }
}

pub fn launch_stochastic_gui() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lotka-Volterra Stochastic Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(StochasticLotkaVolterraApp::new()))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stochastic_solver_generates_data() {
        let params = LotkaVolterraParams {
            alpha: 0.01,
            beta: 0.00001,
            gamma: 0.00001,
            delta: 0.01,
            dt: 0.001,
            n: 1000,
            initial_prey: 2000.0,
            initial_predator: 2000.0,
        };

        let result = solve_stochastic_lotka_volterra(&params);
        assert!(
            !result.is_empty(),
            "Stochastic solver should generate non-empty data."
        );
    }

    #[test]
    fn test_stochastic_solver_initial_conditions() {
        let params = LotkaVolterraParams {
            alpha: 0.1,
            beta: 0.02,
            gamma: 0.02,
            delta: 0.1,
            dt: 0.01,
            n: 100,
            initial_prey: 100.0,
            initial_predator: 50.0,
        };

        let result = solve_stochastic_lotka_volterra(&params);
        assert_eq!(
            result[0],
            [100.0, 50.0],
            "Initial values should match the input parameters."
        );
    }
}
