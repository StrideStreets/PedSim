use crate::model::{
    calc_utils::navigation_distance::find_origin_destination_path,
    object::{Object, ObjectType},
    pedestrian::Pedestrian,
};

use crate::{DISCRETIZATION, TOROIDAL};
use itertools::iproduct;
use krabmaga::{
    engine::{
        fields::{field::Field, field_2d::Field2D, sparse_object_grid_2d::SparseGrid2D},
        location::{Int2D, Real2D},
    },
    rand::{self, Rng},
};
use ndarray::Array2;
use std::collections::HashMap;

pub fn make_field(dim: (f32, f32)) -> Field2D<Pedestrian> {
    Field2D::<Pedestrian>::new(dim.0, dim.1, DISCRETIZATION, TOROIDAL)
}

pub fn make_object_grid(dim: (f32, f32), grid: Option<Array2<u8>>) -> SparseGrid2D<Object> {
    let (width, height) = (dim.0 as i32, dim.1 as i32);
    let mut obj_grid = SparseGrid2D::new(width, height);

    let mut obstacle_id = 0;

    if let Some(grid) = grid {
        iproduct!(0..width, 0..height).for_each(|(col, row)| {
            match grid[[row as usize, col as usize]] {
                0 => {
                    let obstacle_location = Int2D { x: col, y: row };
                    obj_grid.set_object_location(
                        Object {
                            id: obstacle_id,
                            value: ObjectType::Obstacle,
                            location: obstacle_location,
                        },
                        &obstacle_location,
                    );
                    obstacle_id += 1;
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

    obj_grid
}

pub fn make_peds(
    num_peds: u32,
    dim: (f32, f32),
    obj_grid: &SparseGrid2D<Object>,
) -> Vec<Pedestrian> {
    // Gather list of available positions

    let available_positions: Vec<Real2D> = iproduct!(0..obj_grid.width, 0..obj_grid.height)
        .filter(|(x, y)| obj_grid.get_objects(&Int2D { x: *x, y: *y }).is_none())
        .map(|(x, y)| Real2D {
            x: x as f32,
            y: y as f32,
        })
        .collect();

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
    navigable_object_grid: &Array2<u8>,
) -> HashMap<u32, std::vec::IntoIter<Real2D>> {
    let mut ped_path_map = HashMap::<u32, std::vec::IntoIter<Real2D>>::new();

    for ped in pedestrians {
        let Pedestrian { id, loc, dest, .. } = ped;

        if let Some(this_dest) = dest {
            match find_origin_destination_path::<Int2D, i32>(
                Int2D {
                    x: loc.x as i32,
                    y: loc.y as i32,
                },
                Int2D {
                    x: this_dest.x as i32,
                    y: this_dest.y as i32,
                },
                navigable_object_grid,
            )
            .map(|node_vec| {
                //println!("Located path for {}", i);
                let real_vec: Vec<Real2D> = node_vec
                    .into_iter()
                    .map(|node| Real2D {
                        x: node.x as f32,
                        y: node.y as f32,
                    })
                    .collect();
                real_vec
            }) {
                Ok(shortest_path) => {
                    //println!("Found path for agent {}", i);
                    ped_path_map.insert(*id, shortest_path.into_iter());
                }
                Err(_e) => {
                    //println!("{}", e);
                }
            }
        }
    }

    ped_path_map
}
