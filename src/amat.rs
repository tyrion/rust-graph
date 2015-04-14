use graph::{Graph, del_edge};
use edge::{EK, EV, E, V};


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

    fn get_edge(&self, from: usize, to: usize) -> Option<EV<&Val>> {
        match self.0[from][to] {
            Some(ref v) => Some(EV { to: to, val: v }),
            None => None
        }
        // (&self.0[from][to]).map(|ref v| EV { to: to, val: v })
        // self.0[from].iter().enumerate().find(|&(_, v)| match *v {
        //     Some(ref x) => x == val,
        //     None => false
        // }).map(|(i, _)| i)
    }

    fn get_edgeval(&self, from: usize, to: usize) -> Option<&Val> {
        self.0[from][to].as_ref()
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

    fn get_edge(&self, from: usize, key: Key) -> Option<EK<Key>> {
        self.0[from].iter().enumerate().find(|&(_, vec)| 
            vec.iter().any(|k| *k == key)).map(|(i, _)| EK { to: i, key: key })
    }

    fn get_edgeval(&self, from: usize, edge: EK<Key>) -> Option<&Key> {
        self.0[from][edge.to].iter().find(|k| **k == edge.key)
    }
}


impl<Key: Eq + Copy, Val: Eq> Graph for AMat<Vec<V<Key, Val>>> {
    type Vertex = usize;
    type Edge = E<Key, Val>;

    fn add_vertex(&mut self) {}

    fn adjacent(&self, from: usize, to: usize) -> bool {
        !self.0[from][to].is_empty()
    }

    fn add_edge(&mut self, from: usize, edge: E<Key, Val>) {
        self.0[from][edge.to].push(edge.val);
    }

    fn has_edge(&self, from: usize, edge: EK<Key>) -> bool {
        self.0[from][edge.to].iter().any(|x| x.key == edge.key)
    }

    fn del_edge(&mut self, from: usize, edge: EK<Key>) {
        del_edge(&mut self.0[from][edge.to], edge.key, |v| v.key);
    }

    fn get_edge(&self, from: usize, key: Key) -> Option<E<Key, &Val>> {
        let vec = &self.0[from];
        for (i, edges) in vec.iter().enumerate() {
            match edges.iter().find(|v| v.key == key) {
                Some(x) => return Some(E::new(i, x.key, &x.val)),
                None => ()
            }
        }
        return None;
    }

    fn get_edgeval(&self, from: usize, edge: EK<Key>) -> Option<&V<Key, Val>> {
        self.0[from][edge.to].iter().find(|x| x.key == edge.key)
    }

}
