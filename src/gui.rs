use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::models::LotkaVolterraParameters;
use crate::solver::solve_lotka_volterra;

/// Main application struct for the Lotka-Volterra GUI.
pub struct LotkaVolterraApp {
    params: LotkaVolterraParameters,
    prey_points: Vec<[f64; 2]>,
    predator_points: Vec<[f64; 2]>,
}

impl LotkaVolterraApp {
    /// Create a new instance of the app with the given parameters.
    pub fn new(params: LotkaVolterraParameters) -> Self {
        let mut app = Self {
            params,
            prey_points: Vec::new(),
            predator_points: Vec::new(),
        };
        app.solve_system(); // Solve the system and initialize the plot data
        app
    }

    /// Solve the Lotka-Volterra system and update the plot data.
    fn solve_system(&mut self) {
        // Initial conditions: [prey, predator]
        let y0 = [40.0, 9.0];
        let t0 = 0.0;
        let t_end = 200.0;
        let step = 0.1;

        if let Ok((times, prey, predators)) = solve_lotka_volterra(self.params, y0, t0, t_end, step) {
            self.prey_points = times.iter().zip(prey.iter()).map(|(&x, &y)| [x, y]).collect();
            self.predator_points = times.iter().zip(predators.iter()).map(|(&x, &y)| [x, y]).collect();
        }
    }
}

impl eframe::App for LotkaVolterraApp {
    /// Update the GUI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lotka-Volterra Predator-Prey Model");

            // Add sliders for parameter adjustment
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
            if ui.button("Solve").clicked() {
                self.solve_system();
            }

            // Plot the results
            let prey_line = Line::new(PlotPoints::from_iter(self.prey_points.iter().map(|&[x, y]| [x, y])));
            let predator_line = Line::new(PlotPoints::from_iter(self.predator_points.iter().map(|&[x, y]| [x, y])));

            Plot::new("lotka_volterra_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(prey_line.name("Prey"));
                    plot_ui.line(predator_line.name("Predators"));
                });
        });
    }
}

/// Launch the interactive GUI.
pub fn launch_gui(params: LotkaVolterraParameters) -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lotka-Volterra Model",
        options,
        Box::new(|_cc| Ok(Box::new(LotkaVolterraApp::new(params)))),
    )
    .map_err(|e| e.into())
}
