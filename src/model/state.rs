use std::{any::Any, collections::HashMap};

use super::{
    calc_utils::navigation_distance::{find_origin_destination_path, make_navigable_matrix},
    object::{Object, ObjectType},
    pedestrian::Pedestrian,
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
    pub field: Field2D<Pedestrian>,
    pub obj_grid: SparseGrid2D<Object>,
    pub ped_paths: HashMap<u32, std::vec::IntoIter<Real2D>>,
    pub dim: (f32, f32),
    pub num_agents: u32,
}

impl ModelState {
    pub fn new(dim: (f32, f32), num_agents: u32) -> ModelState {
        ModelState {
            step: 0,
            field: Field2D::new(dim.0, dim.1, DISCRETIZATION, TOROIDAL),
            obj_grid: SparseGrid2D::new(dim.0 as i32, dim.1 as i32),
            ped_paths: HashMap::<u32, std::vec::IntoIter<Real2D>>::new(),
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
        self.field = Field2D::new(self.dim.0, self.dim.1, DISCRETIZATION, TOROIDAL);
        self.obj_grid = SparseGrid2D::new(self.dim.0 as i32, self.dim.1 as i32)
    }

    /// Put the code that should be executed to initialize simulation:
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;

        let mut rng = rand::thread_rng();

        //Make a test obstacle
        let mut obstacle_id = 0;
        for x in ((self.dim.0 / 5.) as i32)..((self.dim.0 / 4.) as i32) {
            for y in ((self.dim.1 / 5.) as i32)..((self.dim.1 / 4.) as i32) {
                let obstacle_location = Int2D { x: x, y: y };
                self.obj_grid.set_object_location(
                    Object {
                        id: obstacle_id,
                        value: ObjectType::Obstacle,
                        location: *&obstacle_location,
                    },
                    &obstacle_location,
                );

                obstacle_id += 1;
            }
        }

        self.obj_grid.update();

        //Make matrix representation of grid that will be used for pathfinding
        let navigable_object_grid = make_navigable_matrix::<i32, Object>(&self.obj_grid);

        //Make agents
        for i in 0..self.num_agents {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();
            let d1: f32 = rng.gen();
            let d2: f32 = rng.gen();
            let speed: f32 = rng.gen();

            let last_d = Real2D { x: 0., y: 0. };

            let loc = Real2D {
                x: self.dim.0 * r1,
                y: self.dim.1 * r2,
            };

            let dest = Some(Real2D {
                x: self.dim.0 * d1,
                y: self.dim.1 * d2,
            });

            let agent = Pedestrian::new(i, loc, last_d, dest, 1.0);
            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);

            //Pre-compute shortest paths for agents, and place in ped_paths
            // In this case, we should convert vector of Int2D to Real2D, since we will use these
            // values as positions for our agents on a real field
            let this_dest = dest.unwrap_or(Real2D { x: 1., y: 1. });

            if let Ok(shortest_path) = find_origin_destination_path::<Int2D, i32>(
                Int2D {
                    x: loc.x as i32,
                    y: loc.y as i32,
                },
                Int2D {
                    x: this_dest.x as i32,
                    y: this_dest.y as i32,
                },
                &navigable_object_grid,
            )
            .and_then(|node_vec| {
                let real_vec: Vec<Real2D> = node_vec
                    .into_iter()
                    .map(|node| Real2D {
                        x: node.x as f32,
                        y: node.y as f32,
                    })
                    .collect();
                Ok(real_vec)
            }) {
                self.ped_paths.insert(i, shortest_path.into_iter());
            }
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
