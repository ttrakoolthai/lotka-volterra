// gui.rs - Handles interactive GUI using egui
// src/gui.rs
use crate::solver::solve_lotka_volterra;
use crate::models::LotkaVolterraParameters;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

pub struct LotkaVolterraApp {
    params: LotkaVolterraParameters,
    prey_points: Vec<[f64; 2]>,
    predator_points: Vec<[f64; 2]>,
}

impl LotkaVolterraApp {
    pub fn new(params: LotkaVolterraParameters) -> Self {
        let mut app = Self {
            params,
            prey_points: Vec::new(),
            predator_points: Vec::new(),
        };
        app.solve_system(); // Solve the system and initialize the plot data
        app
    }

    fn solve_system(&mut self) {
        if let Ok((times, prey, predators)) = solve_lotka_volterra(self.params, [40.0, 9.0], 0.0, 200.0, 0.1) {
            self.prey_points = times.iter().zip(prey.iter()).map(|(&x, &y)| [x, y]).collect();
            self.predator_points = times.iter().zip(predators.iter()).map(|(&x, &y)| [x, y]).collect();
        }
    }
}

impl eframe::App for LotkaVolterraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lotka-Volterra Predator-Prey Model");
            if ui.button("Solve").clicked() {
                self.solve_system();
            }
            let prey_line = Line::new(PlotPoints::from_iter(self.prey_points.iter().map(|&[x, y]| [x, y])));
            let predator_line = Line::new(PlotPoints::from_iter(self.predator_points.iter().map(|&[x, y]| [x, y])));
            Plot::new("lotka_volterra_plot").view_aspect(2.0).show(ui, |plot_ui| {
                plot_ui.line(prey_line);
                plot_ui.line(predator_line);
            });
        });
    }
}
