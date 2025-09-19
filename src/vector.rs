pub mod quad;
pub mod universal;
pub mod connection;
pub mod matrix;

use std::rc::Rc;
use std::cell::RefCell;

pub type DynVector = dyn Vector;
pub type Design = Rc::<DynVector>;
pub type BoxedVector = Box::<DynVector>;
pub type MutableVector = RefCell::<BoxedVector>;
pub type Material = Rc::<MutableVector>;

use matrix::{ Matrix, Dimension };

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
    fn get_matrix(&self) -> Vec<f32>{
        todo!();
    }
}

pub trait RuleVector<R>: Vector{
    /// One or zero for the dimension of x, y or z. One meaning true, zero meaning false. True meaning this component is part of the constraint.
    fn get_constraint(&self) -> (u8, u8, u8);
}

pub trait MatrixProto{
    fn multiply(&self) -> Matrix{
        todo!();
    }
}