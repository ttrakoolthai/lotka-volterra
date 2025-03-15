pub mod models;
pub mod solver;
pub mod plot;
pub mod gui;


// pub mod models;
// pub mod solver;
// pub mod plot;
// pub mod gui;

// (Re-exports modules)

// // (Expose modules for main.rs)

// use eframe::egui;
// use egui_plot::{Line, Plot, PlotPoints};
// use ode_solvers::{SVector, System, dopri5::Dopri5};
// use plotters::prelude::*; // Add this import
// use std::error::Error;

// /// Parameters for the Lotka-Volterra model
// #[derive(Debug, Clone, Copy)]
// pub struct LotkaVolterraParameters {
//     pub alpha: f64,  // Prey birth rate
//     pub beta : f64,  // Predation rate
//     pub delta: f64,  // Predator reproduction rate
//     pub gamma: f64,  // Predator death rate
// }

// impl Default for LotkaVolterraParameters {
//     fn default() -> Self {
//         Self {
//             alpha: 0.1,
//             beta: 0.02,
//             delta: 0.01,
//             gamma: 0.1,
//         }
//     }
// }

// // Define the system of ODEs
// type State = SVector<f64, 2>;

// struct LotkaVolterraSystem {
//     params: LotkaVolterraParameters,
// }

// impl LotkaVolterraSystem {
//     fn new(params: LotkaVolterraParameters) -> Self {
//         Self { params }
//     }
// }

// impl System<f64, State> for LotkaVolterraSystem {
//     fn system(&self, _t: f64, y: &State, dydt: &mut State) {
//         // Prey dynamics
//         dydt[0] = self.params.alpha * y[0] - self.params.beta * y[0] * y[1];
//         // Predator dynamics
//         dydt[1] = self.params.delta * y[0] * y[1] - self.params.gamma * y[1];
//     }
// }

// /// Function to generate a static plot using `plotters`
// pub fn plot_lotka_volterra(
//     params: LotkaVolterraParameters,
//     y0: [f64; 2],
//     t0: f64,
//     t_end: f64,
//     step: f64,
//     output_file: &str,
// ) -> Result<(), Box<dyn Error>> {
//     if y0.iter().any(|&x| x < 0.0) {
//         return Err("Initial population values must be non-negative".into());
//     }

//     let mut solver = Dopri5::new(
//         LotkaVolterraSystem::new(params),
//         t0,
//         t_end,
//         step,
//         SVector::from(y0),
//         1e-6,
//         1e-6,
//     );

//     let result = solver.integrate();

//     let (times, prey, predators) = match result {
//         Ok(_) => {
//             let times: Vec<f64> = solver.x_out().to_vec();
//             let prey: Vec<f64> = solver.y_out().iter().map(|y| y[0]).collect();
//             let predators: Vec<f64> = solver.y_out().iter().map(|y| y[1]).collect();
//             (times, prey, predators)
//         }
//         Err(e) => return Err(Box::new(e)),
//     };

//     let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
//     root.fill(&WHITE)?;

//     let mut chart = ChartBuilder::on(&root)
//         .caption("Lotka-Volterra Predator-Prey Model", ("sans-serif", 30))
//         .margin(20)
//         .x_label_area_size(40)
//         .y_label_area_size(40)
//         .build_cartesian_2d(0.0..t_end, 0.0..50.0)?;

//     chart.configure_mesh().draw()?;

//     chart
//         .draw_series(LineSeries::new(
//             times.iter().zip(prey.iter()).map(|(&x, &y)| (x, y)),
//             &BLUE,
//         ))?
//         .label("Prey")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

//     chart
//         .draw_series(LineSeries::new(
//             times.iter().zip(predators.iter()).map(|(&x, &y)| (x, y)),
//             &RED,
//         ))?
//         .label("Predators")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

//     chart
//         .configure_series_labels()
//         .background_style(&WHITE.mix(0.8))
//         .border_style(&BLACK)
//         .draw()?;

//     Ok(())
// }

// /// Struct for the interactive GUI
// pub struct LotkaVolterraApp {
//     params: LotkaVolterraParameters,
//     prey_points: Vec<[f64; 2]>,
//     predator_points: Vec<[f64; 2]>,
// }

// impl LotkaVolterraApp {
//     pub fn new(params: LotkaVolterraParameters) -> Self {
//         Self {
//             params,
//             prey_points: Vec::new(),
//             predator_points: Vec::new(),
//         }
//     }

//     fn solve_system(&mut self) {
//         let y0 = [40.0, 9.0]; // Initial conditions: [prey, predator]
//         let t0 = 0.0; // Initial time
//         let t_end = 200.0; // End time
//         let step = 0.1; // Step size

//         let mut solver = Dopri5::new(
//             LotkaVolterraSystem::new(self.params),
//             t0,
//             t_end,
//             step,
//             SVector::from(y0),
//             1e-6,
//             1e-6,
//         );

//         if let Ok(_) = solver.integrate() {
//             self.prey_points = solver
//                 .x_out()
//                 .iter()
//                 .zip(solver.y_out().iter())
//                 .map(|(&t, y)| [t, y[0]])
//                 .collect();

//             self.predator_points = solver
//                 .x_out()
//                 .iter()
//                 .zip(solver.y_out().iter())
//                 .map(|(&t, y)| [t, y[1]])
//                 .collect();
//         }
//     }
// }

// impl eframe::App for LotkaVolterraApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Lotka-Volterra Predator-Prey Model");

//             // Input fields for parameters
//             ui.horizontal(|ui| {
//                 ui.label("Alpha (prey birth rate):");
//                 ui.add(egui::Slider::new(&mut self.params.alpha, 0.0..=1.0).text("Alpha"));
//             });
//             ui.horizontal(|ui| {
//                 ui.label("Beta (predation rate):");
//                 ui.add(egui::Slider::new(&mut self.params.beta, 0.0..=1.0).text("Beta"));
//             });
//             ui.horizontal(|ui| {
//                 ui.label("Delta (predator reproduction rate):");
//                 ui.add(egui::Slider::new(&mut self.params.delta, 0.0..=1.0).text("Delta"));
//             });
//             ui.horizontal(|ui| {
//                 ui.label("Gamma (predator death rate):");
//                 ui.add(egui::Slider::new(&mut self.params.gamma, 0.0..=1.0).text("Gamma"));
//             });

//             // Solve the system when the button is clicked
//             if ui.button("Solve").clicked() {
//                 self.solve_system();
//             }

//             // Plot the results using egui_plot
//             let prey_line = Line::new(PlotPoints::from_iter(
//                 self.prey_points.iter().map(|&[x, y]| [x, y]),
//             ))
//             .name("Prey")
//             .color(egui::Color32::BLUE);

//             let predator_line = Line::new(PlotPoints::from_iter(
//                 self.predator_points.iter().map(|&[x, y]| [x, y]),
//             ))
//             .name("Predators")
//             .color(egui::Color32::RED);

//             Plot::new("lotka_volterra_plot")
//                 .view_aspect(2.0)
//                 .show(ui, |plot_ui| {
//                     plot_ui.line(prey_line);
//                     plot_ui.line(predator_line);
//                 });
//         });
//     }
// }



// src/
// ├── bin/main.rs        # Main entry point
// ├── lib.rs             # Public API re-exports modules
// ├── models.rs          # Defines Lotka-Volterra parameters & system
// ├── solver.rs          # Handles numerical solving (using ode_solvers)
// ├── plot.rs            # Handles static plotting with `plotters`
// ├── gui.rs             # Handles interactive GUI with `egui`
