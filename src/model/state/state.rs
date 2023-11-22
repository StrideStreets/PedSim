use std::{any::Any, collections::HashMap};

use crate::model::{
    calc_utils::navigation_distance::make_navigable_matrix,
    object::{Object, ObjectType},
    pedestrian::Pedestrian,
    state::components::*,
};

use krabmaga::engine::fields::field::Field;
use krabmaga::engine::{
    fields::{field_2d::Field2D, sparse_number_grid_2d::SparseNumberGrid2D},
    location::{Int2D, Real2D},
    schedule::Schedule,
    state::State,
};
use ndarray::Array2;

/// Expand the state definition according to your model, for example by having a grid struct field to
/// store the agents' locations.
pub struct ModelState {
    pub step: u64,
    pub peds: Vec<Pedestrian>,
    pub field: Field2D<Pedestrian>,
    pub obj_grid: SparseNumberGrid2D<u8>,
    pub ped_paths: HashMap<u32, std::vec::IntoIter<Real2D>>,
    pub dim: (f32, f32),
    pub num_agents: u32,
}

impl ModelState {
    pub fn new(dim: (f32, f32), num_agents: u32, grid: Option<Array2<u8>>) -> ModelState {
        let obj_grid;
        //let navigable_object_grid;
        //Make object grid
        match grid {
            Some(ngrid) => {
                obj_grid = make_object_grid(dim, Some(ngrid.clone()));
                //navigable_object_grid = ngrid;
            }
            None => {
                obj_grid = make_object_grid(dim, grid);
                //navigable_object_grid = make_navigable_matrix::<i32, u8>(&obj_grid)
            }
        };
        //Initialize pedestrian records
        let peds = make_peds(num_agents, dim, &obj_grid);

        //Make field for pedestrians
        let field = make_field(dim);

        //Calculate paths, given pedestrians
        let ped_paths = make_paths(&peds, &obj_grid);

        ModelState {
            step: 0,
            peds,
            field,
            obj_grid,
            ped_paths,
            dim,
            num_agents,
        }
    }

    // pub fn get_obstacle(&self, loc: &Int2D) -> Option<Vec<Object>> {
    //     self.obj_grid
    //         .get_value(loc)
    //         .filter(|vec| vec.first().unwrap().value == ObjectType::Obstacle)
    //}
}

impl State for ModelState {
    /// Put the code that should be executed for each state update here. The state is updated once for each
    /// schedule step.
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
    }

    /// Put the code that should be executed to reset simulation state
    fn reset(&mut self) {
        self.step = 0;
        self.field = make_field(self.dim);
    }

    /// Put the code that should be executed to initialize simulation:
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;

        let peds_iter = self.peds.iter();

        for agent in peds_iter {
            schedule.schedule_repeating(Box::new(*agent), 0., 0);
        }

        self.obj_grid.update();
        //println!("{:?}", self.ped_paths);
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
