use super::connection::{ 
    Connection, 
    Quad, 
    Edge, 
};

pub trait Vector{
    fn get_dimension(&self) -> (f32, f32, f32);
    fn get_clone(&self) -> Box::<dyn Vector>;
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
    x: f32,
    y: f32,
    z: f32,
}

impl CommonPosition{
    pub fn new(x: f32, y: f32, z: f32) -> CommonPosition{
        CommonPosition{
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
    fn get_clone(&self) -> Box::<dyn Vector>{
        Box::new(self.clone())
    }
}

impl<'c> Connection<'c> for CommonPosition{
    fn nested_in_edge(&self) -> &Edge<'c>{
        todo!();
    }
    fn nested_in_quad(&self) -> (char, &Quad<'c>){
        todo!();
    }
}

