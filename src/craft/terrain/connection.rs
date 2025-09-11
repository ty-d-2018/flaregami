use super::sensory::{ Vector };

pub trait Connection<'b>{
    fn nested_in_edge(&self) -> &Edge<'b>;
    fn nested_in_quad(&self) -> (char, &Quad<'b>);
    fn point_in_quad(quad: &Quad<'b>, id: char) -> Option::<Box::<dyn Vector>>{
        match id{
            'a' => Some(quad.a.get_clone()),
            'b' => Some(quad.b.get_clone()),
            'c' => Some(quad.c.get_clone()),
            'd' => Some(quad.d.get_clone()),
            _ => None,
        }
    } 
    fn point_in_edge(edge: &Edge<'b>, id: char) -> Option::<Box::<dyn Vector>>{
        match id{
            'a' => Some(edge.a.get_clone()),
            'b' => Some(edge.b.get_clone()),
            _ => None,
        }
    }
}

pub struct Quad<'a>{
    a: &'a dyn Vector,
    b: &'a dyn Vector,
    c: &'a dyn Vector,
    d: &'a dyn Vector,
}

pub struct Edge<'e>{
    a: &'e dyn Vector,
    b: &'e dyn Vector,
}

impl<'a> Quad<'a>{
    pub fn new(a: &'a impl Vector, b: &'a impl Vector, c: &'a impl Vector, d: &'a impl Vector) -> Quad<'a>{
        Quad{
            a,
            b,
            c,
            d
        }
    }
}

impl<'e> Edge<'e>{
    pub fn new(a: &'e impl Vector, b: &'e impl Vector) -> Edge<'e>{
        Edge{
            a,
            b,
        }
    }
}