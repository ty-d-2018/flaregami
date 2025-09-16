use super::{ 
    Vector, 
    DynVector, 
    Design, 
    BoxedVector, 
    MutableVector, 
    Material, 
};

pub struct Quad{
    a: BoxedVector,
    b: BoxedVector,
    c: BoxedVector,
    d: BoxedVector,
}

pub struct Edge{
    a: BoxedVector,
    b: BoxedVector,
}

impl Quad{
    pub fn new(a: &BoxedVector, b: &BoxedVector, c: &BoxedVector, d: &BoxedVector) -> Quad{
        Quad{
            a: a.get_clone(),
            b: b.get_clone(),
            c: c.get_clone(),
            d: d.get_clone()
        }
    }
    pub fn get_a(&self) -> BoxedVector{
        self.a.get_clone()
    }
    pub fn get_b(&self) -> BoxedVector{
        self.b.get_clone()
    }
    pub fn get_c(&self) -> BoxedVector{
        self.c.get_clone()
    }
    pub fn get_d(&self) -> BoxedVector{
        self.d.get_clone()
    }
}

impl Edge{
    pub fn new(a: &BoxedVector, b: &BoxedVector) -> Edge{
        Edge{
            a: a.get_clone(),
            b: b.get_clone(),
        }
    }
    pub fn get_a(&self) -> BoxedVector{
        self.a.get_clone()
    }
    pub fn get_b(&self) -> BoxedVector{
        self.b.get_clone()
    }
}