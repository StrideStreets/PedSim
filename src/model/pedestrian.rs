use crate::model::sea::Sea;
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::{toroidal_transform, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct Crab {
    pub id: u32,
    pub loc: Real2D,
    pub last_d: Real2D,
    pub dir_x: f32,
    pub dir_y: f32,
}

#[derive(Copy, Clone)]
pub struct Pedestrian {
    pub id: u32,
    pub loc: Real2D,
    pub dest: Option<Real2D>,
    pub vec_x: f32,
    pub vec_y: f32
    pub done: bool,
}

impl Pedestrian {
    pub fn new(id: u32, loc: Real2D, dest: Option<Real2D>, vec_x: f32, vec_y: f32, done: bool) -> Pedestrian {
        Pedestrian {
            id,
            loc,
            dest,
            vec_x,
            vec_y,
            done,
        }
    }

    fn update_position(&mut self, state: ) {
        if self.done {
            return self.loc;
        }

        match self.dest {
            //Eventually, this will be refined to check for boundary conditions and adjust direction as needed to maintain
            //progress toward destination
          Some(dest) => {
            if (dest === self.loc){
                self.done = true;
                return self.loc;
            }
            let loc_x = toroidal_transform(self.loc.x + self.vec_x, state.field.width);
            let loc_y = toroidal_transform(self.loc.y + self.vec_y, state.field.height);
            return Real2D { x: loc_x, y: loc_y };
          },
          //Eventually, this will be changed to have person wander randomly
          None => {
            let loc_x = toroidal_transform(self.loc.x + self.vec_x, state.field.width);
            let loc_y = toroidal_transform(self.loc.y + self.vec_y, state.field.height);
            return Real2D { x: loc_x, y: loc_y };
          }
        }
    }
}


impl Agent for Pedestrian {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any().downcast_ref::<Sea>().unwrap();
        let mut rng = rand::thread_rng();

        if rng.gen_bool(0.5) {
            self.vec_x -= 1.0;
        }
        if rng.gen_bool(0.5) {
            self.vec_y -= 1.0;
        }

        let new_loc = self.update_position(state);
        self.loc = new_loc;

        state
            .field
            .set_object_location(*self, new_loc);
    }

    /// Put the code that decides if an agent should be removed or not
    /// for example in simulation where agents can die
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }
}

impl Hash for Pedestrian {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Location2D<Real2D> for Pedestrian {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}

impl fmt::Display for Pedestrian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Pedestrian {}

impl PartialEq for Pedestrian {
    fn eq(&self, other: &Pedestrian) -> bool {
        self.id == other.id
    }
}
