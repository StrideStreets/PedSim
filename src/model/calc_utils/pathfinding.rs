use super::utility_types::NavigationPoint;
use super::utility_types::Num2D;
use anyhow::anyhow;
use anyhow::Error;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::location::Real2D;
use ndarray::Array2;
use std::cmp::Eq;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Hash)]
struct NodeDistance<N> {
    node: Num2D,
    dist: N,
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
fn get_additional_distance<T>(current: T, neighbor: T) -> i32 {
    1
}

pub fn astar<N>(origin: Num2D, destination: Num2D, grid: Array2<i8>) -> Result<Vec<Num2D>, Error>
where
    //T: NavigationPoint<N> + Hash + Eq + Copy,
    N: Clone + PartialEq + Copy + Default + TryFrom<usize> + TryInto<usize>,
    Num2D: Eq + PartialEq + Hash,
    NodeDistance<N>: Ord + Eq,
{
    let x_min: usize = 0;
    let x_max: usize = grid.ncols() - 1;
    let y_min: usize = 0;
    let y_max: usize = grid.nrows() - 1;

    let mut position_queue = Vec::<Num2D<N>>::new();
    let mut open_set = BinaryHeap::<Reverse<NodeDistance<N>>>::new();
    let mut prev_position = HashMap::<Num2D<N>, Num2D<N>>::new();
    let mut current_shortest_distance = HashMap::<Num2D<N>, N>::new();
    let mut estimated_shortest_distance = HashMap::<Num2D<N>, N>::new();

    open_set.push(Reverse(NodeDistance {
        node: origin,
        dist: N::default(),
    }));

    current_shortest_distance.insert(origin, N::default());

    estimated_shortest_distance.insert(origin, origin.manhattan_distance(&destination));

    while let Some(Reverse(node_dist)) = open_set.pop() {
        let NodeDistance { node, dist } = node_dist;

        if node == destination {
            return Ok(position_queue);
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
                    let node = match origin {
                        Num2D::Int2D(_) => {
                            Int2D {
                                x: col
                                    .try_into()
                                    .expect("Grid X coordinate should be convertible into N"),
                                y: col
                                    .try_into()
                                    .expect("Grid Y coordinate should be convertible into N"),
                            };
                        }
                        Num2D::Real2D(_) => {
                            Real2D {
                                x: col
                                    .try_into()
                                    .expect("Grid X coordinate should be convertible into N"),
                                y: col
                                    .try_into()
                                    .expect("Grid Y coordinate should be convertible into N"),
                            };
                        }
                    };

                    let added_dist = get_additional_distance(node, destination);
                    //  = open_set.push(Reverse(NodeDistance {
                    //       node,
                    //       dist: dist + 1,
                    //   }));
                });

            return Err(anyhow!("Placeholder error"));
        }
    }
    return Err(anyhow!("Placeholder error"));
}
