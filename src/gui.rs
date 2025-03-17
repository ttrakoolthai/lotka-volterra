use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Legend, Corner};
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
    error_message: Option<String>,    // Stores error messages for display
    t_start: f64, // 🔹 New: Start time
    t_end: f64, // 🔹 New: Total simulation time
}

impl LotkaVolterraApp {
    /// Create a new instance of the app with the given parameters.
    pub fn new(params: LotkaVolterraParameters) -> Self {
        let mut app = Self {
            params,
            prey_points: Vec::new(),
            predator_points: Vec::new(),
            phase_points: Vec::new(),
            error_message: None,
            t_start: 0.0, // Default simulation duration
            t_end: 200.0, // Default simulation duration
        };
        app.solve_system(); // Solve the system and initialize the plot data
        app
    }

    /// Solve the Lotka-Volterra system and update the plot data.
    fn solve_system(&mut self) {
        let y0 = [self.params.initial_prey, self.params.initial_predator];
        let t0 = 0.0;
        let step = 0.1;

        match solve_lotka_volterra(self.params, y0, t0, self.t_end, step) {
            Ok((times, prey, predators)) => {
                self.prey_points = times.iter().zip(prey.iter()).map(|(&x, &y)| [x, y]).collect();
                self.predator_points = times.iter().zip(predators.iter()).map(|(&x, &y)| [x, y]).collect();
                self.phase_points = prey.iter().zip(predators.iter()).map(|(&x, &y)| [x, y]).collect();
                self.error_message = None; // Clear previous errors if successful
            }
            Err(e) => {
                self.error_message = Some(format!("Error solving equations: {}", e));
                eprintln!("Error solving Lotka-Volterra equations: {:?}", e);
            }
        }
    }
}

impl eframe::App for LotkaVolterraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Centered main title (keeps large font)
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Lotka-Volterra Predator-Prey Model");
            });
            ui.add_space(10.0);

            // Display error message in the GUI if there's a problem
            if let Some(ref msg) = self.error_message {
                ui.colored_label(egui::Color32::RED, msg);
                ui.add_space(10.0);
            }

            // Use columns to display the two plots side by side
            ui.columns(2, |columns| {
                // Phase plot (Predator vs Prey)
                columns[0].vertical(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Predator vs. Prey (Phase Plot)").size(14.0)); // Smaller font
                    });
                    ui.add_space(5.0);

                    let phase_line = Line::new(PlotPoints::from_iter(self.phase_points.iter().map(|&[x, y]| [x, y])))
                        .name("Phase Plot");

                    Plot::new("phase_plot")
                        .view_aspect(1.0) // Square aspect ratio
                        .legend(Legend::default().position(Corner::LeftTop)) // Add legend
                        .show(ui, |plot_ui| {
                            plot_ui.line(phase_line);
                        });

                });

                // Population over time plot
                columns[1].vertical(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Prey and Predator Populations Over Time").size(14.0)); // Smaller font
                    });
                    ui.add_space(5.0);

                    let prey_line = Line::new(PlotPoints::from_iter(self.prey_points.iter().map(|&[x, y]| [x, y])))
                        .name("Prey Population");

                    let predator_line = Line::new(PlotPoints::from_iter(self.predator_points.iter().map(|&[x, y]| [x, y])))
                        .name("Predator Population");

                    Plot::new("populations_over_time")
                        .view_aspect(2.0) // Wider aspect ratio for better visualization
                        .legend(Legend::default().position(Corner::RightTop)) // Add legend
                        .show(ui, |plot_ui| {
                            plot_ui.line(prey_line);
                            plot_ui.line(predator_line);
                        });
                });
            });

            ui.add_space(10.0);

            // Collapsible section for simulation parameters
            egui::CollapsingHeader::new("Simulation Parameters")
                .default_open(true)
                .show(ui, |ui| {
                    ui.columns(2, |columns| {
                        // Left column: Initial conditions
                        columns[0].vertical(|ui| {
                            ui.label("Initial Conditions:");
                            if ui.add(egui::Slider::new(&mut self.params.initial_prey, 0.0..=100.0).text("Prey")).changed() {
                                self.solve_system();
                            }
                            if ui.add(egui::Slider::new(&mut self.params.initial_predator, 0.0..=50.0).text("Predator")).changed() {
                                self.solve_system();
                            }
                            if ui.add(egui::Slider::new(&mut self.t_start, 0.0..=self.t_end - 10.0).text("Start Time")).changed() {
                                self.solve_system();
                            }
                            if ui.add(egui::Slider::new(&mut self.t_end, 50.0..=500.0).text("Time End")).changed() {
                                self.solve_system();
                            }
                        });

                        // Right column: Model parameters
                        columns[1].vertical(|ui| {
                            ui.label("Model Parameters:");
                            if ui.add(egui::Slider::new(&mut self.params.alpha, 0.0..=2.0).text("Alpha (Prey Birth Rate)")).changed() {
                                self.solve_system();
                            }
                            if ui.add(egui::Slider::new(&mut self.params.beta, 0.0..=0.5).text("Beta (Predation Rate)")).changed() {
                                self.solve_system();
                            }
                            if ui.add(egui::Slider::new(&mut self.params.delta, 0.0..=0.5).text("Delta (Predator Reproduction Rate)")).changed() {
                                self.solve_system();
                            }
                            if ui.add(egui::Slider::new(&mut self.params.gamma, 0.0..=2.0).text("Gamma (Predator Death Rate)")).changed() {
                                self.solve_system();
                            }
                        });
                    });
                });
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
