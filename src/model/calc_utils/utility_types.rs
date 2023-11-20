use enumx::def_impls;
use krabmaga::engine::location::{Int2D, Real2D};
use std::{
    hash::{Hash, Hasher},
    ops::Sub,
};
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Num2D<N> {
    pub x: N,
    pub y: N,
}

pub trait NavigationPoint<N> {
    fn x(&self) -> N;
    fn y(&self) -> N;
    fn euclidean_distance(&self, other: &Self) -> N;
    fn manhattan_distance(&self, other: &Self) -> N;
    //fn path_distance(&self, other: Other) -> Option<N>;
    //fn navigable_path(&self, other:Other) -> Option<Vec<Self>>;
}

impl<N> From<Int2D> for Num2D<N>
where
    N: From<i32>,
{
    fn from(value: Int2D) -> Self {
        Num2D {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}

impl<N> From<Real2D> for Num2D<N>
where
    N: From<f32>,
{
    fn from(value: Real2D) -> Self {
        Num2D {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}

impl<N> NavigationPoint<N> for Num2D<N>
where
    N: Sub<Output = N> + Into<f64> + From<f64>,
{
    fn euclidean_distance(&self, other: &Self) -> N {
        let dx: f64 = (other.x - self.x).into();
        let dy: f64 = (other.y - self.y).into();
        (dx.powf(2.) + dy.powf(2.)).sqrt().round().into()
    }

    fn manhattan_distance(&self, other: &Self) -> N {
        let dx: f64 = (other.x - self.x).into();
        let dy: f64 = (other.y - self.y).into();
        (dx.abs() + dy.abs()).into()
    }

    fn x(&self) -> N {
        self.y
    }

    fn y(&self) -> N {
        self.x
    }
}
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
