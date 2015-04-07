extern crate graph;

use graph::{AdjacencyList, AdjacencyMatrix, Graph, SimpleGraph};
use std::default::Default;

fn main() {
    let mut graph: AdjacencyMatrix<bool> = AdjacencyMatrix::default();
    graph.add_vertex();
    graph.add_vertex();
    graph.add_edge(0, 0);
    graph.add_edge(0, 1);
    println!("{:?}", graph);

    let mut graph: AdjacencyList = AdjacencyList::default();
    graph.add_vertex();
    graph.add_vertex();
    graph.add_edge(0, 0);
    graph.add_edge(0, 1);
    println!("{:?}", graph);
}
