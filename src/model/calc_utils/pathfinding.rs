use super::utility_types::NavigationPoint;
use super::utility_types::Num2D;
use anyhow::anyhow;
use anyhow::Error;
use krabmaga::engine::fields::sparse_number_grid_2d::SparseNumberGrid2D;
use krabmaga::engine::location::Int2D;
use ndarray::Array2;
use num_traits::AsPrimitive;
use std::cmp::Eq;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Hash)]
// pub struct NodeDistance<N> {
//     node: Num2D<N>,
//     dist: N,
// }

pub struct NodeDistance {
    node: Int2D,
    dist: i32,
}

// impl<N> Display for NodeDistance<N>
// where
//     N: Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "`{{`Node: {},\ndist: {}`}}`", self.node, self.dist)
//     }
// }

impl Display for NodeDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{{`Node: {},\ndist: {}`}}`", self.node, self.dist)
    }
}

// impl<N> PartialEq for NodeDistance<N>
// where
//     N: PartialEq,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.dist == other.dist
//     }
// }

impl PartialEq for NodeDistance {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

//impl<N> Eq for NodeDistance<N> where N: Eq {}

impl Eq for NodeDistance {}

// impl<N> PartialOrd for NodeDistance<N>
// where
//     N: PartialOrd,
// {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.dist.partial_cmp(&other.dist)
//     }
// }

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

// impl<N> Ord for NodeDistance<N>
// where
//     N: Ord + Eq,
// {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.dist.cmp(&other.dist)
//     }
// }

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

//TO BE EDITED WHEN SWITCHING TO GRAPH REPRESENTATION
fn get_additional_distance(_current: Int2D, _neighbor: Int2D) -> i32 {
    1
}

// fn get_distance_estimate<N>(current: &Num2D<N>, dest: &Num2D<N>) -> Result<N, Error>
// where
//     N: Sub<Output = N> + TryInto<f64> + Copy + 'static,
//     f64: AsPrimitive<N>,
// {
//     current.manhattan_distance(dest)
// }

fn get_distance_estimate(current: &Int2D, dest: &Int2D) -> Result<i32, Error> {
    current.manhattan_distance(dest)
}

// fn reconstruct_path<N>(
//     node: &Num2D<N>,
//     prev_position_map: &HashMap<Num2D<N>, Num2D<N>>,
// ) -> VecDeque<Num2D<N>>
// where
//     Num2D<N>: Hash + PartialEq + Eq + Copy,
// {
//     let mut current_node = node;
//     let mut path = VecDeque::<Num2D<N>>::new();
//     while let Some(prev_node) = prev_position_map.get(current_node) {
//         path.push_front(*prev_node);
//         current_node = prev_node;
//     }

//     path
// }

fn reconstruct_path(node: &Int2D, prev_position_map: &HashMap<Int2D, Int2D>) -> VecDeque<Int2D> {
    let mut current_node = node;
    let mut path = VecDeque::<Int2D>::new();
    while let Some(prev_node) = prev_position_map.get(current_node) {
        path.push_front(*prev_node);
        current_node = prev_node;
    }

    path
}
pub fn astar(
    origin: Int2D,
    destination: Int2D,
    grid: &SparseNumberGrid2D<u8>,
) -> Result<VecDeque<Int2D>, Error> {
    let x_min = 0;
    let x_max = grid.width - 1;
    let y_min = 0;
    let y_max = grid.height - 1;

    //Priority queue for examining nodes
    let mut node_queue = BinaryHeap::<Reverse<NodeDistance>>::new();
    //Set to keep track of elements in open_set more efficiently
    let mut queued_node_set = HashSet::<Int2D>::new();
    let mut prev_position = HashMap::<Int2D, Int2D>::new();
    let mut current_shortest_distance = HashMap::<Int2D, i32>::new();
    let mut estimated_shortest_distance = HashMap::<Int2D, i32>::new();

    //Add to priority queue an item holding the node and its distance estimate
    node_queue.push(Reverse(NodeDistance {
        node: origin,
        dist: get_distance_estimate(&origin, &destination)?,
    }));

    current_shortest_distance.insert(origin, 0);

    estimated_shortest_distance.insert(origin, origin.manhattan_distance(&destination)?);

    while let Some(Reverse(node_dist)) = node_queue.pop() {
        let NodeDistance {
            node,
            dist: est_dist,
        } = node_dist;

        //println!("Examining node {}", node_dist);
        let current_dist = est_dist - get_distance_estimate(&node, &destination)?;

        //Remove node from tracker set...
        queued_node_set.remove(&node);

        if node == destination {
            return Ok(reconstruct_path(&node, &prev_position));
        }

        let mut neighbors = Vec::<Int2D>::new();
        //For now, only considering four-directional moves
        if node.x != x_min {
            neighbors.push(Int2D {
                x: node.x - 1,
                y: node.y,
            });
        }
        if node.x != x_max {
            neighbors.push(Int2D {
                x: node.x + 1,
                y: node.y,
            });
        }
        if node.y != y_min {
            neighbors.push(Int2D {
                x: node.x,
                y: node.y - 1,
            });
        }
        if node.y != y_max {
            neighbors.push(Int2D {
                x: node.x,
                y: node.y + 1,
            });
        }

        for neib_node in neighbors {
            if grid.get_value(&neib_node).is_none() {
                let added_dist = get_additional_distance(neib_node, destination);
                if let Some(curr_dist) = current_shortest_distance.get(&neib_node) {
                    if *curr_dist <= current_dist + added_dist {
                        continue;
                    }
                }

                if let Ok(distance_estimate) = get_distance_estimate(&neib_node, &destination) {
                    //If our new distance (dist + added_dist) is less than curr_dist OR curr_dist does not exist,
                    //update current_shortest_distance; update estimated shorted_distance; update previous position;
                    //and add neib to set for further examination
                    let new_current_dist = current_dist + added_dist;
                    let new_estimated_dist = new_current_dist + distance_estimate;

                    current_shortest_distance.insert(neib_node, new_current_dist);
                    estimated_shortest_distance.insert(neib_node, new_estimated_dist);
                    prev_position.insert(neib_node, node);

                    if queued_node_set.insert(neib_node.clone()) {
                        node_queue.push(Reverse(NodeDistance {
                            node: neib_node.clone(),
                            dist: new_estimated_dist,
                        }))
                    }
                } else {
                    println!("Failed to calculate distance estimate")
                }
            }
        }
    }
    Err(anyhow!(
        "Failed to locate valid path from origin to destination"
    ))
}
