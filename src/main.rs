// Global imports (needed for the simulation to run)
use crate::model::state::state::ModelState;
mod model;
mod system_interface;

use clap::Parser;
use std::error::Error;
use system_interface::object_grid_loader::read_raster;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::model_vis::ModelVis, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = false;

#[derive(Parser, Debug)]
struct Args {
    /// Raster file to read in as obstacle grid
    #[arg(short, long)]
    input: String,
}

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() -> Result<(), image::ImageError> {
    let args = Args::parse();
    let dim: (f32, f32);

    let obj_grid = match args.input.is_empty() {
        true => {
            dim = (400., 400.);
            None
        }
        false => match read_raster(args.input) {
            Ok(grid) => {
                dim = (grid.ncols() as f32, grid.nrows() as f32);
                Some(grid)
            }
            Err(e) => {
                return Err(e);
            }
        },
    };

    let step = 100;

    let num_agents = 2;

    //let (agents, paths)

    let state = ModelState::new(dim, num_agents, obj_grid);

    simulate!(state, step, 10);

    Ok(())
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() -> Result<(), image::ImageError> {
    let args = Args::parse();
    let dim: (f32, f32);
    // Initialize the simulation and its visualization here.
    let obj_grid = match args.input.is_empty() {
        true => {
            dim = (400., 400.);
            None
        }
        false => match read_raster(args.input) {
            Ok(grid) => {
                dim = (grid.ncols() as f32, grid.nrows() as f32);
                Some(grid)
            }
            Err(e) => {
                return Err(e);
            }
        },
    };
    let num_agents = 500;

    let state = ModelState::new(dim, num_agents, obj_grid);
    Visualization::default()
        .with_window_dimensions(1280., 720.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::BLACK)
        .with_name("Template")
        .start::<ModelVis, ModelState>(ModelVis, state);

    Ok(())
}
