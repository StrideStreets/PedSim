use super::pathfinding::astar;
use super::utility_types::*;
use anyhow::{anyhow, Error};
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::{Int2D, Real2D};
use ndarray::Array2;
use num_traits::AsPrimitive;
use std::cmp::Eq;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::format;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::ops::Add;
use std::ops::Sub;

pub fn normalize_motion_vector(loc: Real2D, dest: Real2D) -> (f32, f32) {
    let initial_vector_magnitude: f32 =
        ((dest.x - loc.x).powf(2.0) + (dest.y - loc.y).powf(2.0)).sqrt();
    let dir_x: f32 = (dest.x - loc.x) / initial_vector_magnitude;
    let dir_y: f32 = (dest.y - loc.y) / initial_vector_magnitude;
    (dir_x, dir_y)
}

pub fn make_navigable_matrix<N, O>(grid: &SparseGrid2D<O>) -> Array2<i8>
where
    N: Clone + PartialEq + Copy + Default + TryFrom<usize>,
    O: Eq + Hash + Clone + Copy,
{
    let mut navigable_array = Array2::<i8>::default((grid.height as usize, grid.width as usize));

    //GET RID OF THESE EXPECTS WITH PROPER ERROR HANDLING
    navigable_array
        .indexed_iter_mut()
        .for_each(|((row, col), value_ref)| {
            let loc = Int2D {
                x: col
                    .try_into()
                    .expect("Column index should be convertible from usize"),
                y: row
                    .try_into()
                    .expect("Row index should be convertible from usize"),
            };
            *value_ref = match grid.get_objects(&loc) {
                Some(_) => 0 as i8,
                None => 1 as i8,
            };
        });

    navigable_array
}

pub fn find_origin_destination_path<T, N>(
    origin: T,
    destination: T,
    grid: &Array2<i8>,
) -> Result<VecDeque<T>, Error>
where
    T: NavigationPoint<N> + Hash + Eq + Copy + TryInto<Num2D<N>> + TryFrom<Num2D<N>>,
    N: Clone
        + Ord
        + Copy
        + Default
        + Hash
        + TryFrom<usize>
        + TryInto<usize>
        + TryInto<f64>
        + Sub<Output = N>
        + Add<Output = N>
        + 'static,
    <N as TryFrom<usize>>::Error: Debug,
    f64: AsPrimitive<N>,
{
    let converted_origin = origin
        .try_into()
        .map_err(|e| anyhow!("Failed to convert origin point to Num2D"))?;

    let converted_destination = destination
        .try_into()
        .map_err(|e| anyhow!("Failed to convert destinations point to Num2D"))?;

    astar(converted_origin, converted_destination, grid.clone()).and_then(|queue| {
        let converted_path: VecDeque<T> = queue
            .iter()
            .filter_map(|node| {
                (*node)
                    .try_into()
                    .map_err(|e| {
                        anyhow!(
                    "Failed while converting path from intermediary Num2D to original node format"
                )
                    })
                    .ok()
            })
            .collect();
        Ok(converted_path)
    })
}
