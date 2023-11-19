use std::any::Any;

use super::{object::Object, pedestrian::Pedestrian};
use krabmaga::engine::fields::field::Field;
use krabmaga::{
    engine::{
        fields::sparse_object_grid_2d::SparseGrid2D, location::Int2D, schedule::Schedule,
        state::State,
    },
    rand::{self, Rng},
};

/// Expand the state definition according to your model, for example by having a grid struct field to
/// store the agents' locations.
pub struct ModelState {
    pub step: u64,
    pub ped_grid: SparseGrid2D<Pedestrian>,
    pub obj_grid: SparseGrid2D<Object>,
    pub dim: (i32, i32),
    pub num_agents: u32,
}

impl ModelState {
    pub fn new(dim: (i32, i32), num_agents: u32) -> ModelState {
        ModelState {
            step: 0,
            ped_grid: SparseGrid2D::new(dim.0, dim.1),
            obj_grid: SparseGrid2D::new(dim.0, dim.1),
            dim,
            num_agents,
        }
    }
}

impl State for ModelState {
    /// Put the code that should be executed for each state update here. The state is updated once for each
    /// schedule step.
    fn update(&mut self, _step: u64) {
        self.ped_grid.lazy_update();
    }

    /// Put the code that should be executed to reset simulation state
    fn reset(&mut self) {
        self.step = 0;
        self.ped_grid = SparseGrid2D::new(self.dim.0, self.dim.1);
        self.obj_grid = SparseGrid2D::new(self.dim.0, self.dim.1)
    }

    /// Put the code that should be executed to initialize simulation:
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;

        let mut rng = rand::thread_rng();

        for i in 0..self.num_agents {
            let lx: i32 = rng.gen_range(0..self.dim.0);
            let ly: i32 = rng.gen_range(0..self.dim.1);
            let dx: i32 = rng.gen_range(0..self.dim.0);
            let dy: i32 = rng.gen_range(0..self.dim.1);
            let speed: i32 = rng.gen_range(1..5);

            let last_d = Int2D { x: 0, y: 0 };

            let loc = Int2D { x: lx, y: ly };

            let dest = Some(Int2D { x: dx, y: dy });

            let agent = Pedestrian::new(i, loc, last_d, dest, 1);
            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.step += 1
    }
}
