use crate::model::{calc_utils::normalize_motion_vector, state::ModelState};
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
    pub last_d: Real2D,
    pub dest: Option<Real2D>,
    pub dir_x: f32,
    pub dir_y: f32,
    pub speed: f32,
}

impl Pedestrian {
    pub fn new(
        id: u32,
        loc: Real2D,
        last_d: Real2D,
        dest: Option<Real2D>,
        speed: f32,
    ) -> Pedestrian {
        let dir_x: f32;
        let dir_y: f32;

        match dest {
            Some(dest) => {
                (dir_x, dir_y) = normalize_motion_vector(loc, dest);
            }
            None => {
                dir_x = 0.0;
                dir_y = 0.0;
            }
        };

        Pedestrian {
            id,
            loc,
            last_d,
            dest,
            dir_x,
            dir_y,
            speed,
        }
    }
}

impl Agent for Pedestrian {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any().downcast_ref::<ModelState>().unwrap();
        let mut rng = rand::thread_rng();

        if let Some(dest) = self.dest {
            (self.dir_x, self.dir_y) = normalize_motion_vector(self.loc, dest)
        }
        let loc_x = self.loc.x + self.dir_x * self.speed;
        let loc_y = self.loc.y + self.dir_y * self.speed;

        let new_loc = Real2D { x: loc_x, y: loc_y };
        self.loc = new_loc;

        state.field.set_object_location(*self, new_loc);
    }

    /// Put the code that decides if an agent should be removed or not
    /// for example in simulation where agents can die
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        match self.dest {
            Some(dest) => ((self.loc.x - dest.x).abs() < 1.0) & ((self.loc.y - dest.y).abs() < 1.0),
            None => false,
        }
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
