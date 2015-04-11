use graph::{Graph, del_edge};
use edge::Edge;


pub struct ALst<T>(Vec<Vec<T>>);


impl<T> Graph for ALst<T> where T: Edge<usize> {
    type Vertex = usize;
    type Edge = T;

    fn add_vertex(&mut self) {
        self.0.push(vec![]);
    }

    fn adjacent(&self, from: usize, to: usize) -> bool {
        self.0[from].iter().any(|e| e.to() == to)
    }

    fn add_edge(&mut self, from: usize, edge: T) {
        self.0[from].push(edge);
    }

    fn has_edge(&self, from: usize, to: T::K) -> bool  {
        self.0[from].iter().any(|e| e.key() == to)
    }

    fn del_edge(&mut self, from: usize, to: T::K) {
        del_edge(&mut self.0[from], to, |e| e.key());
    }

    fn get_edgeval(&self, from: usize, to: T::K) -> Option<&T::V> {
        self.0[from].iter().find(|e| e.key() == to).map(|e| e.val())
    }

    fn get_edge(&self, from: usize, val: &T::V) -> Option<T::K> {
        self.0[from].iter().find(|e| e.val() == val).map(|e| e.key())
    }
}
