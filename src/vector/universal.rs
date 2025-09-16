
pub mod arrow{
    use std::rc::Rc;
    use std::cell::RefCell;

    use super::super::{ 
        Vector, 
        DynVector, 
        Design, 
        BoxedVector, 
        MutableVector, 
        Material, 
    };

    #[derive(Clone)]
    pub struct Point{
        x: f32,
        y: f32,
        z: f32,
    }
    
    impl Point{
        pub fn new(x: f32, y: f32, z: f32) -> Point{
            Point{
                x,
                y,
                z,
            }
        }
    }

    impl Vector for Point{
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
}
