use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::models::LotkaVolterraParameters;
use crate::solver::solve_lotka_volterra;
use std::error::Error;
use crate::error::SimulationError;

/// Main application struct for the Lotka-Volterra GUI.
pub struct LotkaVolterraApp {
    params: LotkaVolterraParameters,
    prey_points: Vec<[f64; 2]>,       // Prey population over time
    predator_points: Vec<[f64; 2]>,   // Predator population over time
    phase_points: Vec<[f64; 2]>,      // Predator vs. Prey (phase plot)
}

impl LotkaVolterraApp {
    /// Create a new instance of the app with the given parameters.
    pub fn new(params: LotkaVolterraParameters) -> Self {
        let mut app = Self {
            params,
            prey_points: Vec::new(),
            predator_points: Vec::new(),
            phase_points: Vec::new(),
        };
        app.solve_system(); // Solve the system and initialize the plot data
        app
    }

    /// Solve the Lotka-Volterra system and update the plot data.
    fn solve_system(&mut self) {
        // Initial conditions: [prey, predator]
        let y0 = [self.params.initial_prey, self.params.initial_predator];
        let t0 = 0.0;
        let t_end = 200.0;
        let step = 0.1;

        if let Ok((times, prey, predators)) = solve_lotka_volterra(self.params, y0, t0, t_end, step) {
            // Store the results for plotting
            self.prey_points = times.iter().zip(prey.iter()).map(|(&x, &y)| [x, y]).collect();
            self.predator_points = times.iter().zip(predators.iter()).map(|(&x, &y)| [x, y]).collect();
            self.phase_points = prey.iter().zip(predators.iter()).map(|(&x, &y)| [x, y]).collect();
        }
    }
}

impl eframe::App for LotkaVolterraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Center the main title
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Lotka-Volterra Predator-Prey Model");
            });

            // Add space after the main title
            ui.add_space(10.0);

            // Use columns to display the two plots side by side with equal size
            ui.columns(2, |columns| {
                // First column: Population over time
                columns[0].vertical(|ui| {
                    // Center the title for the first plot
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label("Prey and Predator Populations Over Time");
                    });

                    // Add space after the graph title
                    ui.add_space(5.0);

                    let prey_line = Line::new(PlotPoints::from_iter(self.prey_points.iter().map(|&[x, y]| [x, y])));
                    let predator_line = Line::new(PlotPoints::from_iter(self.predator_points.iter().map(|&[x, y]| [x, y])));
                    Plot::new("populations_over_time")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            plot_ui.line(prey_line.name("Prey"));
                            plot_ui.line(predator_line.name("Predators"));
                        });
                });

                // Second column: Phase plot
                columns[1].vertical(|ui| {
                    // Center the title for the second plot
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label("Predator vs. Prey (Phase Plot)");
                    });

                    // Add space after the graph title
                    ui.add_space(5.0);

                    let phase_line = Line::new(PlotPoints::from_iter(self.phase_points.iter().map(|&[x, y]| [x, y])));
                    Plot::new("phase_plot")
                        .view_aspect(1.0) // Square aspect ratio for phase plots
                        .show(ui, |plot_ui| {
                            plot_ui.line(phase_line.name("Phase Plot"));
                        });
                });
            });

            // Add space before the sliders and button
            ui.add_space(10.0);

            // Sliders for initial populations
            ui.horizontal(|ui| {
                ui.label("Initial Prey Population:");
                ui.add(egui::Slider::new(&mut self.params.initial_prey, 0.0..=100.0));
            });
            ui.horizontal(|ui| {
                ui.label("Initial Predator Population:");
                ui.add(egui::Slider::new(&mut self.params.initial_predator, 0.0..=50.0));
            });

            // Sliders for model parameters
            ui.horizontal(|ui| {
                ui.label("Alpha (prey birth rate):");
                ui.add(egui::Slider::new(&mut self.params.alpha, 0.0..=2.0));
            });
            ui.horizontal(|ui| {
                ui.label("Beta (predation rate):");
                ui.add(egui::Slider::new(&mut self.params.beta, 0.0..=0.5));
            });
            ui.horizontal(|ui| {
                ui.label("Delta (predator reproduction rate):");
                ui.add(egui::Slider::new(&mut self.params.delta, 0.0..=0.5));
            });
            ui.horizontal(|ui| {
                ui.label("Gamma (predator death rate):");
                ui.add(egui::Slider::new(&mut self.params.gamma, 0.0..=2.0));
            });

            // Solve button
            if ui.button("Plot").clicked() {
                self.solve_system();
            }
        });
    }
}

/// Launch the interactive GUI.
pub fn launch_gui(params: LotkaVolterraParameters) -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lotka-Volterra Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(LotkaVolterraApp::new(params)))),
    )
    .map_err(|e| SimulationError::GuiError(e.to_string()).into())
}
