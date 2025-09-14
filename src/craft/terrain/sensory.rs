use super::connection::{ 
    Connection, 
    Quad, 
    Edge, 
};
use super::map::{ Id };

use std::rc::Rc;
use std::cell::RefCell;

pub type DynVector = dyn Vector;
pub type Design = Rc::<DynVector>;
pub type BoxedVector = Box::<DynVector>;
pub type MutableVector = RefCell::<BoxedVector>;
pub type Material = Rc::<MutableVector>;

pub trait Vector{
    fn get_dimension(&self) -> (f32, f32, f32);
    fn get_clone(&self) -> BoxedVector;
    fn get_design(&self) -> Design;
    fn ref_cell_clone(&self) -> MutableVector; 
    fn into_material(&self) -> Material;
    fn get_x(&self) -> f32{
        let (x, y, z) = self.get_dimension();
        x
    }
    fn get_y(&self) -> f32{
        let (x, y, z) = self.get_dimension();
        y
    }
    fn get_z(&self) -> f32{
        let(x, y, z) = self.get_dimension();        
        z
    }
    fn get_xy(&self) -> (f32, f32){
        let(x, y, z) = self.get_dimension();
        (x, y)
    }
    fn get_xz(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (x, z)
    }
    fn get_yx(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (y, x)
    }
    fn get_yz(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (y, z)
    }
    fn get_zx(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (z, x)
    }
    fn get_zy(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (z, y)
    }
}

#[derive(Clone)]
pub struct CommonPosition{
    id: Option::<Id::<String>>,
    x: f32,
    y: f32,
    z: f32,
    //in_edge: Rc::<Edge>,
}

impl CommonPosition{
    pub fn new(x: f32, y: f32, z: f32) -> CommonPosition{
        CommonPosition{
            id: None,
            x,
            y,
            z,
        }
    }
}

impl Vector for CommonPosition{
    fn get_dimension(&self) -> (f32, f32, f32){
        (self.x, self.y, self.z)
    }
    fn get_clone(&self) -> BoxedVector{
        Box::new(self.clone())
    }
    fn get_design(&self) -> Design{
        Rc::new(self.clone())
    }
    fn ref_cell_clone(&self) -> MutableVector{
        RefCell::new(self.get_clone())
    }
    fn into_material(&self) -> Material{
        Rc::new(self.ref_cell_clone())
    }
}

impl<'c> Connection<'c> for CommonPosition{
    fn nested_in_edge(&self) -> (char, &Edge){
        todo!();
    }
    fn nested_in_quad(&self) -> (char, &Quad){
        todo!();
    }
}

