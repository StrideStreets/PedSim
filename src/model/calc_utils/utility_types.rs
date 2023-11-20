use enumx::def_impls;
use krabmaga::engine::location::{Int2D, Real2D};
use std::hash::{Hash, Hasher};
#[derive(Clone, Hash, PartialEq, Eq)]
// pub enum Num2D {
//     Int2D(Int2D),
//     Real2D(Real2D),
// }

pub struct Num2D<N> {
    x: N,
    y: N,
}

pub trait NavigationPoint<N> {
    fn x(&self) -> N;
    fn y(&self) -> N;
    fn euclidean_distance(&self, other: &Self) -> N;
    fn manhattan_distance(&self, other: &Self) -> N;
    //fn path_distance(&self, other: Other) -> Option<N>;
    //fn navigable_path(&self, other:Other) -> Option<Vec<Self>>;
}

// impl<N> NavigationPoint<N> for Num2D {
//     fn euclidean_distance(&self, other: &Self) -> N {
//         match self {
//             Num2D::Int2D(node) => *node.euclidean_distance(other),
//             Num2D::Real2D(node) => *node.euclidean_distance(other),
//         }
//     }

//     fn manhattan_distance(&self, other: &Self) -> N {
//         match self {
//             Num2D::Int2D(node) => *node.manhattan_distance(other),
//             Num2D::Real2D(node) => *node.manhattan_distance(other),
//         }
//     }

//     fn x(&self) -> N {
//         match self {
//             Num2D::Int2D(node) => node.x(),
//             Num2D::Real2D(node) => node.x(),
//         }
//     }

//     fn y(&self) -> N {
//         match self {
//             Num2D::Int2D(node) => node.y(),
//             Num2D::Real2D(node) => node.y(),
//         }
//     }
// }
impl NavigationPoint<i32> for Int2D {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
    fn euclidean_distance(&self, other: &Self) -> i32 {
        ((other.x - self.x).pow(2) as f64 + (other.y - self.y).pow(2) as f64)
            .sqrt()
            .round() as i32
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl NavigationPoint<f32> for Real2D {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
    fn euclidean_distance(&self, other: &Self) -> f32 {
        ((other.x - self.x).powf(2.) + (other.y - self.y).powf(2.)).sqrt()
    }

    fn manhattan_distance(&self, other: &Self) -> f32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
