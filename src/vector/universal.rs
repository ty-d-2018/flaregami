
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

    #[derive(Clone)]
    pub enum Home{
        x,
        y,
        z,
        xy,
        xz,
        yz,
        xyz,
    }

    #[derive(Clone)]
    pub struct Homing{
        direction: Home,
        scalar: (f32, f32, f32),
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

    impl Home{
        pub fn get_scalar(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32){
            match self{
                Home::x => (x, 0.0, 0.0,),
                Home::y => (0.0, y, 0.0),
                Home::z => (0.0, 0.0, z),
                Home::xy => (x, y, 0.0),
                Home::xz => (x, 0.0, z),
                Home::yz => (0.0, y, z),
                Home::xyz => (x, y, z),
            }
        }
    }

    impl Homing{
        pub fn create_beacon(home: &Home, x: f32, y: f32, z: f32) -> Homing{
            let scalar: (f32, f32, f32) = home.get_scalar(x, y, z);
            Homing{
                direction: home.clone(),
                scalar: scalar,
            }
        }
    }

    impl Vector for Homing{
        fn get_dimension(&self) -> (f32, f32, f32){
            self.scalar
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
