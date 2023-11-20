use super::utility_types::NavigationPoint;
use super::utility_types::Num2D;
use anyhow::anyhow;
use anyhow::Error;
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

#[derive(Clone, Debug, Hash)]
pub struct NodeDistance<N> {
    node: Num2D<N>,
    dist: N,
}

impl<N> Display for NodeDistance<N>
where
    N: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{{`Node: {},\ndist: {}`}}`", self.node, self.dist)
    }
}

impl<N> PartialEq for NodeDistance<N>
where
    N: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<N> Eq for NodeDistance<N> where N: Eq {}

impl<N> PartialOrd for NodeDistance<N>
where
    N: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl<N> Ord for NodeDistance<N>
where
    N: Ord + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

//TO BE EDITED WHEN SWITCHING TO GRAPH REPRESENTATION
fn get_additional_distance<T>(current: T, neighbor: T) -> f64 {
    1.
}

fn get_distance_estimate<N>(current: &Num2D<N>, dest: &Num2D<N>) -> Result<N, Error>
where
    N: Sub<Output = N> + TryInto<f64> + Copy + 'static,
    f64: AsPrimitive<N>,
{
    current.manhattan_distance(&dest)
}

fn reconstruct_path<N>(
    node: &Num2D<N>,
    prev_position_map: &HashMap<Num2D<N>, Num2D<N>>,
) -> VecDeque<Num2D<N>>
where
    Num2D<N>: Hash + PartialEq + Eq + Copy,
{
    let mut current_node = node;
    let mut path = VecDeque::<Num2D<N>>::new();
    while let Some(prev_node) = prev_position_map.get(current_node) {
        path.push_front(*prev_node);
        current_node = prev_node;
    }

    path
}
pub fn astar<N>(
    origin: Num2D<N>,
    destination: Num2D<N>,
    grid: Array2<i8>,
) -> Result<VecDeque<Num2D<N>>, Error>
where
    //T: NavigationPoint<N> + Hash + Eq + Copy,
    N: Clone
        + PartialEq
        + Copy
        + Default
        + Display
        + TryFrom<usize>
        + TryInto<usize>
        + Sub<Output = N>
        + Add<Output = N>
        + TryInto<f64>
        + PartialOrd
        + 'static,
    Num2D<N>: Eq + PartialEq + Hash,
    NodeDistance<N>: Ord + Eq,
    <N as TryFrom<usize>>::Error: Debug,
    f64: AsPrimitive<N>,
{
    //println!("Origin: {}\nDestination: {}", origin, destination);
    let x_min: usize = 0;
    let x_max: usize = grid.ncols() - 1;
    let y_min: usize = 0;
    let y_max: usize = grid.nrows() - 1;

    //Priority queue for examining nodes
    let mut node_queue = BinaryHeap::<Reverse<NodeDistance<N>>>::new();
    //Set to keep track of elements in open_set more efficiently
    let mut queued_node_set = HashSet::<Num2D<N>>::new();
    let mut prev_position = HashMap::<Num2D<N>, Num2D<N>>::new();
    let mut current_shortest_distance = HashMap::<Num2D<N>, N>::new();
    let mut estimated_shortest_distance = HashMap::<Num2D<N>, N>::new();

    //Add to priority queue an item holding the node and its distance estimate
    node_queue.push(Reverse(NodeDistance {
        node: origin,
        dist: get_distance_estimate(&origin, &destination)?,
    }));

    current_shortest_distance.insert(origin, N::default());

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

        if let (Ok(u_x), Ok(u_y)) = (node.x().try_into(), node.y().try_into()) {
            let mut neighbors = Vec::<(usize, usize)>::new();
            //For now, only considering four-directional moves
            if u_x != 0 {
                neighbors.push((u_x - 1, u_y));
            }
            if u_x != x_max {
                neighbors.push((u_x + 1, u_y));
            }
            if u_y != 0 {
                neighbors.push((u_x, u_y - 1));
            }
            if u_y != y_max {
                neighbors.push((u_x, u_y + 1));
            }

            let valid_neighbors = neighbors
                .into_iter()
                .filter(|(col, row)| grid[[*row, *col]] == 1)
                .for_each(|(col, row)| {
                    let neib_node = Num2D {
                        x: col
                            .try_into()
                            .expect("Grid X coordinate should be convertible into N"),
                        y: row
                            .try_into()
                            .expect("Grid Y coordinate should be convertible into N"),
                    };

                    let added_dist: N = get_additional_distance(neib_node, destination).as_();
                    if let Some(curr_dist) = current_shortest_distance.get(&neib_node) {
                        if *curr_dist <= current_dist + added_dist {
                            return;
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

                        if !queued_node_set.contains(&neib_node) {
                            queued_node_set.insert(neib_node);
                            node_queue.push(Reverse(NodeDistance {
                                node: neib_node,
                                dist: new_estimated_dist,
                            }))
                        }
                    }
                });
        }
    }
    return Err(anyhow!(
        "Failed to locate valid path from origin to destination"
    ));
}
