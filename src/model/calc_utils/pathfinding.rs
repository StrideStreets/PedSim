use super::navigation_distance::NavigationPoint;
use ndarray::Array2;
use std::cmp::Eq;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Hash)]
struct NodeDistance<N, T: NavigationPoint<N>> {
    node: T,
    dist: N,
}

impl<N, T> PartialEq for NodeDistance<N, T>
where
    N: PartialEq,
    T: NavigationPoint<N>,
{
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<N, T> Eq for NodeDistance<N, T>
where
    N: Eq,
    T: NavigationPoint<N>,
{
}

impl<N, T> PartialOrd for NodeDistance<N, T>
where
    N: PartialOrd,
    T: NavigationPoint<N>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl<N, T> Ord for NodeDistance<N, T>
where
    N: Ord + Eq,
    T: NavigationPoint<N>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

todo!("Use this when converting to use networks")
fn get_additional_distance<T>(current: T, neighbor: T) -> i32 {
  1
}

//Processing routine for each node
fn process_node(node_dist: NodeDistance, grid: Array2){

  let NodeDistance {
            node: node,
            dist: dist,
        } = node_dist;

  if node == destination {
      return Some(position_queue);
  }

  let u_x: usize = node
      .x
      .try_into()
      .expect("Node X coordinate should be convertible to usize");

  let u_y: usize = node
      .y
      .try_into()
      .expect("Node Y coordinate should be convertible to usize");

  let mut neighbors = Vec::new();
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
      .filter(|(col, row)| grid[[row, col]] == 1)
      .for_each(|(col, row)| {
          let node = T {
              x: col
                  .try_into()
                  .expect("Grid X coordinate should be convertible into N"),
              y: col
                  .try_into()
                  .expect("Grid Y coordinate should be convertible into N"),
          };

          let added_dist = get_additional_distance()
        //  = open_set.push(Reverse(NodeDistance {
        //       node,
        //       dist: dist + 1,
        //   }));
      });
    }



pub fn astar<T, N>(origin: T, destination: T, grid: Array2<i8>) -> Option<Vec<T>>
where
    T: NavigationPoint<N> + Hash + Eq + Copy,
    N: Clone + PartialEq + Copy + Default,
    NodeDistance<N, T>: Ord + Eq,
{
    let x_min: usize = 0;
    let x_max: usize = grid.ncols() - 1;
    let y_min: usize = 0;
    let y_max: usize = grid.nrows() - 1;

    let mut position_queue = Vec::<T>::new();
    let mut open_set = BinaryHeap::<Reverse<NodeDistance<N, T>>>::new();
    let mut prev_position = HashMap::<T, T>::new();
    let mut current_shortest_distance = HashMap::<T, N>::new();
    let mut estimated_shortest_distance = HashMap::<T, N>::new();

    open_set.push(Reverse(NodeDistance {
        node: origin,
        dist: N::default(),
    }));

    current_shortest_distance.insert(origin, N::default());

    estimated_shortest_distance.insert(origin, origin.manhattan_distance(&destination));

    while let Some(Reverse(node_dist)) = open_set.pop() {
    }

    None
}
