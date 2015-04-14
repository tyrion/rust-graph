use edge::{Edge, EdgeRef};

pub trait Graph {
    type Vertex;

    type Edge: EdgeRef<Self::Vertex>;

    // type Iter: Iterator<Item=Self::Edge>;

    fn add_vertex(&mut self);

    fn adjacent(&self, Self::Vertex, Self::Vertex) -> bool;
    // fn outgoing_edges(&self, Self::Vertex) -> Self::Iter;

    fn add_edge(&mut self, Self::Vertex, Self::Edge);
    fn has_edge(&self, Self::Vertex, <Self::Edge as Edge<Self::Vertex>>::I) -> bool;
    fn del_edge(&mut self, Self::Vertex, <Self::Edge as Edge<Self::Vertex>>::I);
    fn get_edge(&self, Self::Vertex, <Self::Edge as Edge<Self::Vertex>>::K)
        -> Option<<Self::Edge as EdgeRef<Self::Vertex>>::Ref>;

    fn get_edgeval(&self, Self::Vertex, <Self::Edge as Edge<Self::Vertex>>::I)
        -> Option<&<Self::Edge as Edge<Self::Vertex>>::V>;
//
//     fn has_edgeval(&self, Self::Vertex, &<Self::Edge as Edge>::V) -> bool;
    // fn get_edgeval(&self, Self::Vertex, Self::EdgeVal) -> Option<Self::Edge>;

    // fn set_edge(&mut self, Self::Vertex, Self::Edge);
}

// pub struct GraphIterator<G> {
//     graph: &G,
//     from: G::Vertex,
//     next: &G::Edge
// }

pub fn del_edge<T, E, F>(vec: &mut Vec<T>, el: E, map: F)
    where E: Eq + Copy, F: Fn(&T) -> E {
    let mut x = None;
    for (i, e) in vec.iter().enumerate() {
        if map(e) == el {
            x = Some(i);
            break;
        }
    }
    if let Some(i) = x {
        vec.swap_remove(i);
    }
}

