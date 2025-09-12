use super::sensory::{ Vector, Material, Design, BoxedVector };

pub trait Connection<'b>{
    fn nested_in_edge(&self) -> (char, &Edge);
    fn nested_in_quad(&self) -> (char, &Quad);
    fn point_in_quad(quad: &Quad, id: char) -> Option::<BoxedVector>{
        match id{
            'a' => Some(quad.a.get_clone()),
            'b' => Some(quad.b.get_clone()),
            'c' => Some(quad.c.get_clone()),
            'd' => Some(quad.d.get_clone()),
            _ => None,
        }
    } 
    fn point_in_edge(edge: &Edge, id: char) -> Option::<BoxedVector>{
        match id{
            'a' => Some(edge.a.get_clone()),
            'b' => Some(edge.b.get_clone()),
            _ => None,
        }
    }
}

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
}

impl Edge{
    pub fn new(a: BoxedVector, b: BoxedVector) -> Edge{
        Edge{
            a: a.get_clone(),
            b: b.get_clone(),
        }
    }
}