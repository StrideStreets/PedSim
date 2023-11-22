use crate::model::{
    calc_utils::navigation_distance::*,
    calc_utils::navigation_point::*,
    object::{Object, ObjectType},
    pedestrian::Pedestrian,
};

use crate::{DISCRETIZATION, TOROIDAL};
use itertools::iproduct;
use krabmaga::{
    engine::{
        fields::{field::Field, field_2d::Field2D, sparse_number_grid_2d::SparseNumberGrid2D},
        location::{Int2D, Real2D},
    },
    rand::{self, Rng},
};
use ndarray::Array2;
use std::collections::HashMap;

pub fn make_field(dim: (f32, f32)) -> Field2D<Pedestrian> {
    Field2D::<Pedestrian>::new(dim.0, dim.1, DISCRETIZATION, TOROIDAL)
}

pub fn make_object_grid(dim: (f32, f32), grid: Option<Array2<u8>>) -> SparseNumberGrid2D<u8> {
    let (width, height) = (dim.0 as i32, dim.1 as i32);
    let mut obj_grid = SparseNumberGrid2D::new(width, height);

    if let Some(grid) = grid {
        //println!("{}", &grid);
        iproduct!(0..width, 0..height).for_each(|(col, row)| {
            match grid[[row as usize, col as usize]] {
                0 => {
                    //println!("Obstacle at {}, {}", col, row);
                    let location = Int2D { x: col, y: row };
                    obj_grid.set_value_location(0, &location);
                }
                _ => {}
            }
        });
    }

    //Make a test obstacle

    // for x in ((dim.0 / 4.) as i32)..(3 * (dim.0 / 4.) as i32) {
    //     for y in ((dim.1 / 5.) as i32)..((dim.1 / 4.) as i32) {
    //         let obstacle_location = Int2D { x, y };
    //         obj_grid.set_object_location(
    //             Object {
    //                 id: obstacle_id,
    //                 value: ObjectType::Obstacle,
    //                 location: obstacle_location,
    //             },
    //             &obstacle_location,
    //         );

    //         obstacle_id += 1;
    //     }
    // }

    obj_grid.update();
    //println!("{:#}", obj_grid.get_empty_bags().len());
    obj_grid
}

pub fn make_peds(
    num_peds: u32,
    dim: (f32, f32),
    obj_grid: &SparseNumberGrid2D<u8>,
) -> Vec<Pedestrian> {
    // Gather list of available positions

    let available_positions: Vec<Real2D> = iproduct!(0..obj_grid.width, 0..obj_grid.height)
        .filter(|(x, y)| obj_grid.get_value(&Int2D { x: *x, y: *y }).is_none())
        .map(|(x, y)| Real2D {
            x: x as f32,
            y: y as f32,
        })
        .collect();

    println!(
        "{} Available Starting Positions and Destinations out of {} total positions",
        available_positions.len(),
        (dim.1 * dim.0)
    );

    let mut pedestrians = Vec::<Pedestrian>::new();
    let mut rng = rand::thread_rng();

    for i in 0..num_peds {
        let _speed: f32 = rng.gen_range(1.0..5.0);
        let last_d = Real2D { x: 0., y: 0. };
        let loc = available_positions[rng.gen_range(0..available_positions.len())];
        let dest = Some(available_positions[rng.gen_range(0..available_positions.len())]);

        pedestrians.push(Pedestrian::new(i, loc, last_d, dest, 1.0));
    }
    pedestrians
}

//Pre-compute shortest paths for agents, and place in ped_paths
// In this case, we should convert vector of Int2D to Real2D, since we will use these
// values as positions for our agents on a real field
pub fn make_paths(
    pedestrians: &Vec<Pedestrian>,
    obj_grid: &SparseNumberGrid2D<u8>,
) -> HashMap<u32, std::vec::IntoIter<Real2D>> {
    let mut ped_path_map = HashMap::<u32, std::vec::IntoIter<Real2D>>::new();
    let mut failed_path_ids = Vec::<u32>::new();

    for ped in pedestrians {
        let Pedestrian { id, loc, dest, .. } = ped;

        if let Some(this_dest) = dest {
            let possible_path =
                NavigationPoint::<i32, Int2D, SparseNumberGrid2D<u8>>::path_to_destination(
                    &Int2D {
                        x: loc.x as i32,
                        y: loc.y as i32,
                    },
                    &Int2D {
                        x: this_dest.x as i32,
                        y: this_dest.y as i32,
                    },
                    obj_grid,
                );

            match possible_path {
                Ok(shortest_path) => {
                    let real_vec: Vec<Real2D> = shortest_path
                        .into_iter()
                        .map(|node| Real2D {
                            x: node.x as f32,
                            y: node.y as f32,
                        })
                        .collect();

                    ped_path_map.insert(*id, real_vec.into_iter());
                }
                Err(e) => {
                    failed_path_ids.push(*id);
                    println!("{}", e);
                }
            }
        }
    }
    println!("{} Failed Path Calculations", failed_path_ids.len());
    ped_path_map
}
