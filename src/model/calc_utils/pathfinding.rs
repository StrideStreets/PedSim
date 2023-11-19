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

pub fn astar<T, N>(origin: T, destination: T, grid: Array2<i8>) -> Option<Vec<T>>
where
    T: NavigationPoint<N> + Hash + Eq + Copy,
    N: Clone + PartialEq + Copy + Default,
    NodeDistance<N, T>: Ord + Eq,
{
    //let mut position_queue = Vec::<T>::new();
    let mut open_set = BinaryHeap::<NodeDistance<N, T>>::new();
    let mut prev_position = HashMap::<T, T>::new();
    let mut current_shortest_distance = HashMap::<T, N>::new();
    let mut estimated_shortest_distance = HashMap::<T, N>::new();

    open_set.push(NodeDistance {
        node: origin,
        dist: N::default(),
    });
    current_shortest_distance.insert(origin, N::default());
    estimated_shortest_distance.insert(origin, origin.manhattan_distance(&destination));

    while let Some(node) = open_set.pop() {}

    None
}
