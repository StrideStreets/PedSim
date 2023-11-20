use anyhow::{anyhow, Error};
use krabmaga::engine::location::{Int2D, Real2D};
use num_traits::AsPrimitive;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::Sub,
};
#[derive(Clone, Hash, Debug)]
pub struct Num2D<N> {
    pub x: N,
    pub y: N,
}

impl<N> Copy for Num2D<N> where N: Copy {}

impl<N> Display for Num2D<N>
where
    N: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

impl<N> Eq for Num2D<N> where N: PartialEq {}
impl<N> PartialEq for Num2D<N>
where
    N: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}
pub trait NavigationPoint<N> {
    fn x(&self) -> N;
    fn y(&self) -> N;
    fn euclidean_distance(&self, other: &Self) -> Result<N, Error>;
    fn manhattan_distance(&self, other: &Self) -> Result<N, Error>;
    //fn path_distance(&self, other: Other) -> Option<N>;
    //fn navigable_path(&self, other:Other) -> Option<Vec<Self>>;
}

impl<N> std::error::Error for Num2D<N> where N: Debug + Display {}

impl<N> TryFrom<Int2D> for Num2D<N>
where
    N: TryFrom<i32> + AsPrimitive<i32>,
{
    type Error = anyhow::Error;
    fn try_from(value: Int2D) -> Result<Self, Self::Error> {
        Ok(Num2D {
            x: value
                .x
                .try_into()
                .map_err(|_e| anyhow!("Failed to convert i32 into provided numerical type"))?,
            y: value
                .y
                .try_into()
                .map_err(|_e| anyhow!("Failed to convert i32 into provided numerical type"))?,
        })
    }
}

impl From<Num2D<i32>> for Int2D {
    fn from(value: Num2D<i32>) -> Self {
        Int2D {
            x: value.x,
            y: value.y,
        }
    }
}

impl<N> TryFrom<Real2D> for Num2D<N>
where
    N: TryFrom<f32>,
{
    type Error = anyhow::Error;
    fn try_from(value: Real2D) -> Result<Self, Self::Error> {
        Ok(Num2D {
            x: value
                .x
                .try_into()
                .map_err(|_e| anyhow!("Failed to convert f32 into provided numerical type"))?,
            y: value
                .y
                .try_into()
                .map_err(|_e| anyhow!("Failed to convert f32 into provided numerical type"))?,
        })
    }
}

impl From<Num2D<f32>> for Real2D {
    fn from(value: Num2D<f32>) -> Self {
        Real2D {
            x: value.x,
            y: value.y,
        }
    }
}
impl<N> NavigationPoint<N> for Num2D<N>
where
    N: Sub<Output = N> + TryInto<f64> + Copy + 'static,
    f64: AsPrimitive<N>,
{
    fn euclidean_distance(&self, other: &Self) -> Result<N, Error> {
        let dx: f64 = (other.x - self.x)
            .try_into()
            .map_err(|_e| anyhow!("Could not convert dx into f64"))?;
        let dy: f64 = (other.y - self.y)
            .try_into()
            .map_err(|_e| anyhow!("Could not convert dy into f64"))?;
        Ok((dx.powf(2.) + dy.powf(2.)).sqrt().round().as_())
    }

    fn manhattan_distance(&self, other: &Self) -> Result<N, Error> {
        let dx: f64 = (other.x - self.x)
            .try_into()
            .map_err(|_e| anyhow!("Could not convert dx into f64"))?;
        let dy: f64 = (other.y - self.y)
            .try_into()
            .map_err(|_e| anyhow!("Could not convert dy into f64"))?;
        Ok((dx.abs() + dy.abs()).as_())
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
    fn euclidean_distance(&self, other: &Self) -> Result<i32, Error> {
        Ok(
            ((other.x - self.x).pow(2) as f64 + (other.y - self.y).pow(2) as f64)
                .sqrt()
                .round() as i32,
        )
    }

    fn manhattan_distance(&self, other: &Self) -> Result<i32, Error> {
        Ok((self.x - other.x).abs() + (self.y - other.y).abs())
    }
}

impl NavigationPoint<f32> for Real2D {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
    fn euclidean_distance(&self, other: &Self) -> Result<f32, Error> {
        Ok(((other.x - self.x).powf(2.) + (other.y - self.y).powf(2.)).sqrt())
    }

    fn manhattan_distance(&self, other: &Self) -> Result<f32, Error> {
        Ok((self.x - other.x).abs() + (self.y - other.y).abs())
    }
}
