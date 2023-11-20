use std::{any::Any, collections::HashMap};

use super::{
    calc_utils::navigation_distance::{find_origin_destination_path, make_navigable_matrix},
    object::{Object, ObjectType},
    pedestrian::Pedestrian,
    state_components::*,
};
use crate::{DISCRETIZATION, TOROIDAL};
use krabmaga::engine::fields::field::Field;
use krabmaga::{
    engine::{
        fields::{field_2d::Field2D, sparse_object_grid_2d::SparseGrid2D},
        location::{Int2D, Real2D},
        schedule::Schedule,
        state::State,
    },
    rand::{self, Rng},
};

/// Expand the state definition according to your model, for example by having a grid struct field to
/// store the agents' locations.
pub struct ModelState {
    pub step: u64,
    pub peds: Vec<Pedestrian>,
    pub field: Field2D<Pedestrian>,
    pub obj_grid: SparseGrid2D<Object>,
    pub ped_paths: HashMap<u32, std::vec::IntoIter<Real2D>>,
    pub dim: (f32, f32),
    pub num_agents: u32,
}

impl ModelState {
    pub fn new(dim: (f32, f32), num_agents: u32) -> ModelState {
        //Initialize pedestrian records
        let peds = make_peds(num_agents, dim);

        //Make field for pedestrians
        let field = make_field(dim);

        //Make object grid
        let obj_grid = make_object_grid(dim);

        //Make matrix representation of grid that will be used for pathfinding
        let navigable_object_grid = make_navigable_matrix::<i32, Object>(&obj_grid);

        //Calculate paths, given pedestrians
        let ped_paths = make_paths(&peds, &navigable_object_grid);

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

    pub fn get_obstacle(&self, loc: &Int2D) -> Option<Vec<Object>> {
        self.obj_grid
            .get_objects(loc)
            .filter(|vec| vec.first().unwrap().value == ObjectType::Obstacle)
    }
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
        self.obj_grid = make_object_grid(self.dim);
    }

    /// Put the code that should be executed to initialize simulation:
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;

        let mut peds_iter = self.peds.iter();

        while let Some(agent) = peds_iter.next() {
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
