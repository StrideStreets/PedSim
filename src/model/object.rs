use core::fmt;
use krabmaga::engine::location::Int2D;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Path,
    Obstacle,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectType::Path => write!(f, "Path"),
            ObjectType::Obstacle => write!(f, "Obstacle"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Object {
    pub id: u32,
    pub value: ObjectType,
    pub location: Int2D,
}

impl Object {
    pub fn new(id: u32, value: ObjectType, location: Int2D) -> Object {
        Object {
            id,
            value,
            location,
        }
    }
}

impl Hash for Object {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Object {}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} value {}", self.id, self.value)
    }
}
