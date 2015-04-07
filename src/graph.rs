use std::default::Default;


pub trait Edge<Vertex = Self> where Self: Eq {

    fn to(&self) -> Vertex;
}

impl<T: Copy + Eq> Edge for T {
    fn to(&self) -> T {
        *self
    }
}


pub trait SimpleGraph {
    type Vertex;
    type Edge: Edge<Self::Vertex>;

    fn adjacent(&self, from: Self::Vertex, to: Self::Vertex) -> bool;
    fn add_edge(&mut self, from: Self::Vertex, edge: Self::Edge);
    // fn del_edge(&mut self, from: Self::Vertex, edge: Self::Edge);
    fn has_edge(&self, from: Self::Vertex, edge: Self::Edge) -> bool;
}

pub trait Graph where Self: SimpleGraph {

    fn add_vertex(&mut self);
    // still need to decide how to implement this
    // fn del_vertex(&self, Self::Vertex);
}


#[derive(Debug, Default)]
pub struct AdjacencyList<T=usize>(Vec<Vec<T>>) where T: Edge<usize>;


impl<T> SimpleGraph for AdjacencyList<T> where T: Edge<usize> {
    type Vertex = usize;
    type Edge = T;

    fn adjacent(&self, from: usize, to: usize) -> bool {
        let &AdjacencyList(ref list) = self;
        list[from].iter().any(|e| e.to() == to)
    }

    fn add_edge(&mut self, from: usize, edge: T) {
        let &mut AdjacencyList(ref mut list) = self;
        list[from].push(edge);
    }

    fn has_edge(&self, from: usize, edge: T) -> bool {
        let &AdjacencyList(ref list) = self;
        list[from].iter().any(|e| *e == edge)
    }
}

impl<T> Graph for AdjacencyList<T> where T: Edge<usize> {

    fn add_vertex(&mut self) {
        let &mut AdjacencyList(ref mut list) = self;
        list.push(Vec::new());
    }
}


#[derive(Debug, Default)]
pub struct AdjacencyMatrix<T>(Vec<Vec<T>>);

#[derive(Debug, PartialEq, Eq)]
pub struct DataEdge<V: Eq, D: Eq> {
    to: V,
    data: D
}

impl<V: Eq + Copy, D: Eq> Edge<V> for DataEdge<V, D> {

    fn to(&self) -> V {
        self.to
    }
}

impl<T: Eq> SimpleGraph for AdjacencyMatrix<Option<T>> {
    type Vertex = usize;
    type Edge = DataEdge<usize, T>;

    fn adjacent(&self, from: usize, to: usize) -> bool {
        from == to
    }

    fn add_edge(&mut self, from: usize, edge: DataEdge<usize, T>) {
        let &mut AdjacencyMatrix(ref mut matrix) = self;
        let to = edge.to();
        matrix[from][to] = Some(edge.data);
    }

    fn has_edge(&self, from: usize, edge: DataEdge<usize, T>) -> bool {
        let &AdjacencyMatrix(ref matrix) = self;
        match matrix[from][edge.to] {
            Some(_) => true,
            None => false
        }
    }
}


impl SimpleGraph for AdjacencyMatrix<bool> {
    type Vertex = usize;
    type Edge = usize;

    fn adjacent(&self, from: usize, to: usize) -> bool {
        let &AdjacencyMatrix(ref matrix) = self;
        matrix[from][to]
    }

    fn add_edge(&mut self, from: usize, edge: usize) {
        let &mut AdjacencyMatrix(ref mut matrix) = self;
        matrix[from][edge] = true;
    }

    fn has_edge(&self, from: usize, edge: usize) -> bool {
        self.adjacent(from, edge)
    }
}


impl<T: Default> Graph for AdjacencyMatrix<T> where AdjacencyMatrix<T>: SimpleGraph {

    fn add_vertex(&mut self) {
        let &mut AdjacencyMatrix(ref mut matrix) = self;

        for row in matrix.iter_mut() {
            row.push(T::default());
        }
        
        let len = matrix.len() + 1;
        let mut new_row = Vec::with_capacity(len);

        for _ in 0..len {
            new_row.push(T::default());
        }
        matrix.push(new_row);
    }
}
