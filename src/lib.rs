pub mod cli;
pub mod error;
pub mod gui;
pub mod interactive;
pub mod models;
pub mod plot;
pub mod solver;

// (Re-exports modules, expose modules for main.rs)
// src/
// ├── bin/main.rs        # Main entry point
// ├── lib.rs             # Public API re-exports modules
// ├── models.rs          # Defines Lotka-Volterra parameters & system
// ├── solver.rs          # Handles numerical solving (using ode_solvers)
// ├── plot.rs            # Handles static plotting with `plotters`
// ├── gui.rs             # Handles interactive GUI with `egui`
