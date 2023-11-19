use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::{Int2D, Real2D};
use ndarray::{Array2, ArrayView, Axis};
use std::collections::BinaryHeap;
use std::format;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;

pub fn normalize_motion_vector(loc: Real2D, dest: Real2D) -> (f32, f32) {
    let initial_vector_magnitude: f32 =
        ((dest.x - loc.x).powf(2.0) + (dest.y - loc.y).powf(2.0)).sqrt();
    let dir_x: f32 = (dest.x - loc.x) / initial_vector_magnitude;
    let dir_y: f32 = (dest.y - loc.y) / initial_vector_magnitude;
    (dir_x, dir_y)
}

pub trait NavigationPoint<N, Other = Self> {
    fn euclidean_distance(&self, other: Other) -> N;
    fn manhattan_distance(&self, other: Other) -> N;
    //fn path_distance(&self, other: Other) -> Option<N>;
    //fn navigable_path(&self, other:Other) -> Option<Vec<Self>>;
}

impl NavigationPoint<i32> for Int2D {
    fn euclidean_distance(&self, other: Int2D) -> i32 {
        ((other.x - self.x).pow(2) as f64 + (other.y - self.y).pow(2) as f64)
            .sqrt()
            .round() as i32
    }

    fn manhattan_distance(&self, other: Int2D) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl NavigationPoint<f32> for Real2D {
    fn euclidean_distance(&self, other: Real2D) -> f32 {
        ((other.x - self.x).powf(2.) + (other.y - self.y).powf(2.)).sqrt()
    }

    fn manhattan_distance(&self, other: Real2D) -> f32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
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
) -> Option<Vec<T>>
where
    T: NavigationPoint<N>,
    N: Clone + PartialEq + Copy + Default + TryFrom<usize>,
{
    let mut position_queue = Vec::<T>::new();

    let mut file = File::create("_LOG.txt").expect("create failed");
    file.write_all(format!("{:#}", grid).as_bytes())
        .expect("write failed");

    None
}
