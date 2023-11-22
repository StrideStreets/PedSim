use std::fmt::Display;

use super::pathfinding::*;
use super::utility_types::Num2D;
use anyhow::{anyhow, Error};
use krabmaga::engine::fields::field_2d::{Field2D, Location2D};
use krabmaga::engine::fields::sparse_number_grid_2d::SparseNumberGrid2D;
use krabmaga::engine::location::{Int2D, Real2D};
use num_traits::AsPrimitive;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd, Reverse};
use std::hash::Hash;
use std::ops::Sub;

#[derive(Clone, Hash)]
pub struct NodeDistance<T, N> {
    pub node: T,
    pub dist: N,
}

impl<T, N> Display for NodeDistance<T, N>
where
    T: Display,
    N: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{{`Node: {},\ndist: {}`}}`", self.node, self.dist)
    }
}

impl<T, N> PartialEq for NodeDistance<T, N>
where
    N: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<T, N> Eq for NodeDistance<T, N> where N: PartialEq + Eq {}

impl<T, N> PartialOrd for NodeDistance<T, N>
where
    N: PartialOrd + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl<T, N> Ord for NodeDistance<T, N>
where
    N: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

pub trait NavigationPoint<N, T, G> {
    fn x(&self) -> N;
    fn y(&self) -> N;
    fn euclidean_distance(&self, other: &Self) -> Result<N, Error>;
    fn manhattan_distance(&self, other: &Self) -> Result<N, Error>;
    fn path_to_destination(origin: &Self, destination: &Self, grid: &G) -> Result<Vec<T>, Error>;
}

impl<N, G> NavigationPoint<N, Num2D<N>, G> for Num2D<N>
where
    N: Sub<Output = N> + TryInto<f64> + Copy + 'static,
    f64: AsPrimitive<N>,
{
    fn x(&self) -> N {
        self.y
    }

    fn y(&self) -> N {
        self.x
    }
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

    fn path_to_destination(
        origin: &Self,
        destination: &Self,
        grid: &G,
    ) -> Result<Vec<Num2D<N>>, Error> {
        todo!()
    }
}

impl NavigationPoint<i32, Int2D, SparseNumberGrid2D<u8>> for Int2D {
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

    fn path_to_destination(
        origin: &Self,
        destination: &Self,
        grid: &SparseNumberGrid2D<u8>,
    ) -> Result<Vec<Int2D>, Error> {
        let dequque = astar_int2d(origin, destination, &grid);

        match dequque {
            Ok(vecd) => {
                let converted_path: Vec<Int2D> = vecd.iter().map(|i| *i).collect();
                Ok(converted_path)
            }
            Err(e) => Err(e),
        }
    }
}

impl<O> NavigationPoint<f32, Real2D, Field2D<O>> for Real2D
where
    O: Location2D<Real2D> + Clone + Hash + Eq + Copy + Display,
{
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

    fn path_to_destination(
        origin: &Self,
        destination: &Self,
        grid: &Field2D<O>,
    ) -> Result<Vec<Real2D>, Error> {
        todo!()
    }
}
