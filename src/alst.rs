use graph::{Graph, del_edge};
use edge::EdgeRef;


#[derive(Default, Debug)]
pub struct ALst<T>(Vec<Vec<T>>);


impl<T> Graph for ALst<T> where T: EdgeRef<usize> {
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

    fn has_edge(&self, from: usize, edge_id: T::I) -> bool  {
        self.0[from].iter().any(|e| e.id() == edge_id)
    }

    fn del_edge(&mut self, from: usize, edge_to: T::I) {
        del_edge(&mut self.0[from], edge_to, |e| e.id());
    }

    fn get_edge(&self, from: usize, key: T::K) -> Option<T::Ref> {
        self.0[from].iter().find(|e| e.key() == key).map(|e| e.as_ref())
    }

    fn get_edgeval(&self, from: usize, id: T::I) -> Option<&T::V> {
        self.0[from].iter().find(|e| e.id() == id).map(|e| e.val())
    }
}

impl<T> ALst<T> where T: EdgeRef<usize> {

    pub fn new(size: usize) -> ALst<T> {
        let mut lst = Vec::with_capacity(size);
        for _ in 0..size {
            lst.push(vec![]);
        }
        return ALst(lst);
    }

}
