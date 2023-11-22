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
