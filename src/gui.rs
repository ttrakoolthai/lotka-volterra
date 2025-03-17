use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Legend, Corner};
use crate::models::LotkaVolterraParameters;
use crate::solver::solve_lotka_volterra;
use std::error::Error;
use crate::error::SimulationError;
    use indicatif::{ProgressBar, ProgressStyle};

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
                // 🔹 Print the table **once** when the app starts
                app.print_parameter_table();
        app.solve_system(); // Solve the system and initialize the plot data
        app
    }

    fn print_parameter_table(&self) {
        use prettytable::{Table, Row, Cell};

        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new("+------------------------------------------+--------+"),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| Parameter                                | Value  |"),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("+------------------------------------------+--------+"),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| Alpha (Prey Birth Rate)                  |"),
            Cell::new(&format!("{:.4}  |", self.params.alpha))
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| Beta (Predation Rate)                     |"),
            Cell::new(&format!("{:.4}  |", self.params.beta))
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| Delta (Predator Reproduction Rate)        |"),
            Cell::new(&format!("{:.4}  |", self.params.delta))
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| Gamma (Predator Death Rate)               |"),
            Cell::new(&format!("{:.4}  |", self.params.gamma))
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| Initial Prey Population                   |"),
            Cell::new(&format!("{:.2}  |", self.params.initial_prey))
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| Initial Predator Population               |"),
            Cell::new(&format!("{:.2}  |", self.params.initial_predator))
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| t_start                                   |"),
            Cell::new(&format!("{:.2}  |", self.t_start))
        ]));
        table.add_row(Row::new(vec![
            Cell::new("| t_end                                     |"),
            Cell::new(&format!("{:.2}  |", self.t_end))
        ]));
        table.add_row(Row::new(vec![
            Cell::new("+------------------------------------------+--------+"),
        ]));

        table.printstd(); // ✅ Print table once at startup
    }

    /// Solve the Lotka-Volterra system and update the plot data.
    fn solve_system(&mut self) {
        let y0 = [self.params.initial_prey, self.params.initial_predator];

        let num_steps = ((self.t_end - self.t_start) / 0.1) as u64; // 🔹 Total steps based on time range
        let pb = ProgressBar::new(num_steps);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} steps")
                .unwrap()
                .progress_chars("#>-"),
        );

        match solve_lotka_volterra(self.params, y0, self.t_start, self.t_end, 0.1) {
            Ok((times, prey, predators)) => {
                self.prey_points.clear();
                self.predator_points.clear();
                self.phase_points.clear();

                for (i, (time, prey_val, pred_val)) in itertools::izip!(times.iter(), prey.iter(), predators.iter()).enumerate() {
                    self.prey_points.push([*time, *prey_val]);
                    self.predator_points.push([*time, *pred_val]);
                    self.phase_points.push([*prey_val, *pred_val]);

                    if i as u64 % 10 == 0 { // Update every 10 steps
                        pb.inc(10);
                    }
                }

                pb.finish_with_message("✅ Simulation Complete!");
                self.error_message = None;
            }
            Err(e) => {
                pb.finish_with_message("❌ Error during simulation");
                self.error_message = Some(format!("Error solving equations: {}", e));
            }
        }
    }
}

impl eframe::App for LotkaVolterraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 🔹 Centered main title
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Lotka-Volterra Predator-Prey Model");
            });
            ui.add_space(10.0);

            // 🔹 Display error message if something goes wrong
            if let Some(ref msg) = self.error_message {
                ui.colored_label(egui::Color32::RED, msg);
                ui.add_space(10.0);
            }

            // 🔹 Use two columns: one for Phase Plot, one for Population Over Time
            ui.columns(2, |columns| {
                // 📌 Phase Plot (Predator vs Prey)
                columns[0].vertical(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Predator vs. Prey (Phase Plot)").size(14.0)); // Smaller font
                    });

                    ui.add_space(5.0);

                    if !self.phase_points.is_empty() {
                        let min_x = self.phase_points.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
                        let max_x = self.phase_points.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
                        let min_y = self.phase_points.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min);
                        let max_y = self.phase_points.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max);

                        let phase_line = Line::new(PlotPoints::from_iter(self.phase_points.iter().map(|&[x, y]| [x, y])))
                            .name("Phase Plot");

                        Plot::new("phase_plot")
                            .view_aspect(1.0) // 🔹 Keeps aspect ratio square
                            .legend(Legend::default().position(Corner::LeftTop))
                            .include_x(min_x).include_x(max_x) // 🔹 Auto-scale X-axis
                            .include_y(min_y).include_y(max_y) // 🔹 Auto-scale Y-axis
                            .show(ui, |plot_ui| {
                                plot_ui.line(phase_line);
                            });
                    } else {
                        ui.label("No data available for phase plot.");
                    }
                });

                // 📌 Population Over Time Plot
                columns[1].vertical(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Prey and Predator Populations Over Time").size(14.0)); // Smaller font
                    });

                    ui.add_space(5.0);

                    if !self.prey_points.is_empty() && !self.predator_points.is_empty() {
                        let min_time = self.prey_points.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
                        let max_time = self.prey_points.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
                        let min_pop = self.prey_points.iter().map(|p| p[1]).chain(self.predator_points.iter().map(|p| p[1]))
                            .fold(f64::INFINITY, f64::min);
                        let max_pop = self.prey_points.iter().map(|p| p[1]).chain(self.predator_points.iter().map(|p| p[1]))
                            .fold(f64::NEG_INFINITY, f64::max);

                        let prey_line = Line::new(PlotPoints::from_iter(self.prey_points.iter().map(|&[x, y]| [x, y])))
                            .name("Prey Population");

                        let predator_line = Line::new(PlotPoints::from_iter(self.predator_points.iter().map(|&[x, y]| [x, y])))
                            .name("Predator Population");

                        Plot::new("populations_over_time")
                            .view_aspect(2.0) // 🔹 Ensures a wide view
                            .legend(Legend::default().position(Corner::RightTop))
                            .include_x(min_time).include_x(max_time) // 🔹 Auto-scale Time axis
                            .include_y(min_pop).include_y(max_pop) // 🔹 Auto-scale Population axis
                            .show(ui, |plot_ui| {
                                plot_ui.line(prey_line);
                                plot_ui.line(predator_line);
                            });
                    } else {
                        ui.label("No data available for population over time.");
                    }
                });
            });

            ui.add_space(10.0);

            // 🔹 Collapsible section for user inputs
            egui::CollapsingHeader::new("Simulation Parameters")
                .default_open(true)
                .show(ui, |ui| {
                    ui.columns(2, |columns| {
                        // 🔹 Left column: Initial conditions
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

                        // 🔹 Right column: Model parameters
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
        /// 🔹 Handle window close event
        fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
            println!("Simulation terminating");
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
