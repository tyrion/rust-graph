pub trait Edge<Vertex=Self> where Vertex: Eq + Copy {
    type K: Eq + Copy;
    type V: Eq;

    fn to(&self) -> Vertex;
    fn key(&self) -> Self::K;
    fn val(&self) -> &Self::V;

}

impl<Vertex: Eq + Copy, Key: Eq + Copy> PartialEq<Key> for Edge<Vertex, K=Key> {

    fn eq(&self, key: &Key) -> bool {
        self.key() == *key
    }
}

static TRUE: bool = true;

impl<T: Copy + Eq> Edge for T {
    type K = bool;
    type V = bool;

    fn to(&self) -> T { *self }
    fn key(&self) -> bool { true }
    fn val(&self) -> &bool { &TRUE }
}


pub struct EV<Val: Eq, Vertex=usize> {
    pub to: Vertex,
    pub val: Val
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct EK<Key: Eq, Vertex: Eq=usize> {
    pub to: Vertex,
    pub key: Key
}

pub struct E<K, V, Vertex=usize> {
    pub to: Vertex,
    pub key: K,
    pub val: V
}

impl<Val: Eq, Vertex: Eq + Copy> Edge<Vertex> for EV<Val, Vertex> {
    type K = Vertex;
    type V = Val;

    fn to(&self) -> Vertex { self.to }
    fn key(&self) -> Vertex { self.to }
    fn val(&self) -> &Val { &self.val }
}

impl<Key: Eq + Copy, Vertex: Eq + Copy> Edge<Vertex> for EK<Key, Vertex> {
    type K = Self;
    type V = Key;

    fn to(&self) -> Vertex { self.to }
    fn key(&self) -> Self { *self }
    fn val(&self) -> &Key { &self.key }
}

impl<Key: Eq + Copy, Val: Eq, Vertex: Eq + Copy> Edge<Vertex> for E<Key, Val, Vertex> {
    type K = EK<Key, Vertex>;
    type V = Val;

    fn to(&self) -> Vertex { self.to }
    
    fn key(&self) -> EK<Key, Vertex> {
        EK { to: self.to, key: self.key }
    }
    
    fn val(&self) -> &Val { &self.val }
}
