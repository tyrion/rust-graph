use graph::{Graph, del_edge};
use edge::{EK, EV, E};


pub struct AMat<T>(Vec<Vec<T>>);


impl<Val: Eq> Graph for AMat<Option<Val>> {
    type Vertex = usize;
    type Edge = EV<Val>;

    fn add_vertex(&mut self) {
    }

    fn adjacent(&self, from: usize, to: usize) -> bool {
        self.0[from][to].is_some()
    }

    fn add_edge(&mut self, from: usize, edge: EV<Val>) {
        self.0[from][edge.to] = Some(edge.val);
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.adjacent(from, to)
    }

    fn del_edge(&mut self, from: usize, to: usize) {
        self.0[from].swap_remove(to);
    }

    fn get_edgeval(&self, from: usize, to: usize) -> Option<&Val> {
        self.0[from][to].as_ref()
    }

    fn get_edge(&self, from: usize, val: &Val) -> Option<usize> {
        self.0[from].iter().enumerate().find(|&(_, v)| match *v {
            Some(ref x) => x == val,
            None => false
        }).map(|(i, _)| i)
    }

}

impl<Key: Copy + Eq> Graph for AMat<Vec<Key>> {
    type Vertex = usize;
    type Edge = EK<Key>;

    fn add_vertex(&mut self) {}

    fn adjacent(&self, from: usize, to: usize) -> bool {
        !self.0[from][to].is_empty()
    }

    fn add_edge(&mut self, from: usize, edge: EK<Key>) {
        self.0[from][edge.to].push(edge.key)
    }

    fn has_edge(&self, from: usize, edge: EK<Key>) -> bool {
        self.0[from][edge.to].iter().any(|k| *k == edge.key)
    }

    fn del_edge(&mut self, from: usize, edge: EK<Key>) {
        del_edge(&mut self.0[from][edge.to], edge.key, |k| *k);
    }

    // not so useful in this case
    fn get_edgeval(&self, from: usize, edge: EK<Key>) -> Option<&Key> {
        self.0[from][edge.to].iter().find(|k| **k == edge.key)
    }

    fn get_edge(&self, from: usize, val: &Key) -> Option<EK<Key>> {
        self.0[from].iter().enumerate().find(|&(_, vec)| 
            vec.iter().any(|k| k == val)).map(|(i, _)| EK { to: i, key: *val })
    }
}



pub struct V<Key, Val> {
    pub key: Key,
    pub val: Val
}

impl<Key: Eq + Copy, Val: Eq> Graph for AMat<Vec<V<Key, Val>>> {
    type Vertex = usize;
    type Edge = E<Key, Val>;

    fn add_vertex(&mut self) {}

    fn adjacent(&self, from: usize, to: usize) -> bool {
        !self.0[from][to].is_empty()
    }

    fn add_edge(&mut self, from: usize, edge: E<Key, Val>) {
        self.0[from][edge.to].push(V { key: edge.key, val: edge.val });
    }

    fn has_edge(&self, from: usize, edge: EK<Key>) -> bool {
        self.0[from][edge.to].iter().any(|x| x.key == edge.key)
    }

    fn del_edge(&mut self, from: usize, edge: EK<Key>) {
        del_edge(&mut self.0[from][edge.to], edge.key, |v| v.key);
    }

    fn get_edgeval(&self, from: usize, edge: EK<Key>) -> Option<&Val> {
        self.0[from][edge.to].iter().find(|x| x.key == edge.key)
            .map(|x| &x.val)
    }

    fn get_edge(&self, from: usize, val: &Val) -> Option<EK<Key>> {
        let vec = &self.0[from];
        for (i, edges) in vec.iter().enumerate() {
            match edges.iter().find(|v| v.val == *val) {
                Some(x) => return Some(EK { to: i, key: x.key }),
                None => ()
            }
        }
        return None;
    }

}
