pub trait Edge<Vertex=Self> where Vertex: Eq + Copy {
    type I: Eq + Copy;
    type K: Eq + Copy;
    type V;


    // id must return an unique identifier for this edge
    // i.e. (to, key)
    fn id(&self) -> Self::I;

    fn to(&self) -> Vertex;
    fn key(&self) -> Self::K;
    fn val(&self) -> &Self::V;
}

pub trait EdgeRef<Vertex=Self> where Vertex: Eq + Copy, Self: Edge<Vertex> {
    type Ref: Edge<Vertex, I=Self::I, K=Self::K>;

    fn new_ref(Vertex, Self::K, &Self::V) -> Self::Ref;
    fn as_ref(&self) -> Self::Ref;
}

impl<Vertex: Eq + Copy, Id: Eq + Copy> PartialEq<Id> for Edge<Vertex, I=Id> {

    fn eq(&self, id: &Id) -> bool {
        self.id() == *id
    }
}

static TRUE: bool = true;

impl<T: Copy + Eq> Edge for T {
    type I = T;
    type K = bool;
    type V = bool;

    fn id(&self) -> T { *self }
    fn to(&self) -> T { *self }
    fn key(&self) -> bool { true }
    fn val(&self) -> &bool { &TRUE }
}

impl<T: Copy + Eq> EdgeRef for T {
    type Ref = T;

    fn new_ref(v: T, _: bool, _: &bool) -> T { v }
    fn as_ref(&self) -> T { *self }
}



#[derive(Debug)]
pub struct EV<Val, Vertex=usize> {
    pub to: Vertex,
    pub val: Val
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct EK<Key: Eq + Copy, Vertex: Eq + Copy = usize> {
    pub to: Vertex,
    pub key: Key
}

#[derive(Debug)]
pub struct V<Key: Eq + Copy, Val> {
    pub key: Key,
    pub val: Val
}

pub type E<Key, Val, Vertex=usize> = EV<V<Key, Val>, Vertex>;

impl<Key: Eq + Copy, Val, Vertex: Eq + Copy> E<Key, Val, Vertex> {
    pub fn new(v: Vertex, key: Key, val: Val) -> E<Key, Val, Vertex> {
        EV { to: v, val: V { key: key, val: val }}
    }
}

impl<Val: Eq, Vertex: Eq + Copy> Edge<Vertex> for EV<Val, Vertex> {
    type I = Vertex;
    type K = Vertex;
    type V = Val;

    fn id(&self) -> Vertex { self.to }
    fn to(&self) -> Vertex { self.to }
    fn key(&self) -> Vertex { self.to }
    fn val(&self) -> &Val { &self.val }
}

impl<'a, Val: 'a + Eq, Vertex: Eq + Copy> EdgeRef<Vertex> for EV<Val, Vertex> {
    type Ref = EV<&'a Val, Vertex>;

    fn new_ref(v: Vertex, _: Vertex, val: &Val) -> EV<&Val, Vertex> {
        EV { to: v, val: val }
    }

    fn as_ref(&self) -> EV<&Val, Vertex> {
        EV { to: self.to, val: &self.val }
    }
}

impl<Key: Eq + Copy, Vertex: Eq + Copy> Edge<Vertex> for EK<Key, Vertex> {
    type I = Self;
    type K = Key;
    type V = Key;

    fn id(&self) -> Self { *self }
    fn to(&self) -> Vertex { self.to }
    fn key(&self) -> Key { self.key }
    fn val(&self) -> &Key { &self.key }

}

impl<Key: Eq + Copy, Vertex: Eq + Copy> EdgeRef<Vertex> for EK<Key, Vertex> {
    type Ref = Self;

    fn new_ref(v: Vertex, key: Key, _: &Key) -> Self {
        EK { to: v, key: key }
    }

    fn as_ref(&self) -> Self { *self }
}

impl<Key: Eq + Copy, Val, Vertex: Eq + Copy> Edge<Vertex> for E<Key, Val, Vertex> {
    type I = EK<Key, Vertex>;
    type K = Key;
    type V = V<Key, Val>;

    fn id(&self) -> EK<Key, Vertex> {
        EK { to: self.to, key: self.val.key }
    }

    fn to(&self) -> Vertex { self.to }
    fn key(&self) -> Key { self.val.key }
    fn val(&self) -> &V<Key, Val> { &self.val }
}

impl<'a, Key: Eq + Copy, Val: 'a, Vertex: Eq + Copy> EdgeRef<Vertex> for E<Key, Val, Vertex> {

    type Ref = E<Key, &'a Val, Vertex>;

    fn new_ref(v: Vertex, key: Key, val: &V<Key, Val>) -> E<Key, &Val, Vertex> {
        E::new(v, key, &val.val)
    }

    fn as_ref(&self) -> E<Key, &Val, Vertex> {
        E::new(self.to, self.val.key, &self.val.val)
    }
}
