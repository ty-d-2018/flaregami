use super::quad::{ Edge, Quad };
use super::{ 
    Vector, 
    DynVector, 
    Design, 
    BoxedVector, 
    MutableVector, 
    Material, 
};

pub trait Connection<'b>{
    fn nested_in_edge(&self) -> (char, &Edge);
    fn nested_in_quad(&self) -> (char, &Quad);
    fn point_in_quad(quad: &Quad, id: char) -> Option::<BoxedVector>{
        match id{
            'a' => Some(quad.get_a()),
            'b' => Some(quad.get_b()),
            'c' => Some(quad.get_c()),
            'd' => Some(quad.get_d()),
            _ => None,
        }
    } 
    fn point_in_edge(edge: &Edge, id: char) -> Option::<BoxedVector>{
        match id{
            'a' => Some(edge.get_a()),
            'b' => Some(edge.get_b()),
            _ => None,
        }
    }
}