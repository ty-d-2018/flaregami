
pub trait Vector{
    fn get_dimension(&self) -> (f32, f32, f32);
    fn get_x(arrow: &Self) -> f32{
        let (x, y, z) = arrow.get_dimension();
        x
    }
    fn get_y(arrow: &Self) -> f32{
        let (x, y, z) = arrow.get_dimension();
        y
    }
    fn get_z(arrow: &Self) -> f32{
        let(x, y, z) = arrow.get_dimension();        
        z
    }
    fn get_xy(arrow: &Self) -> (f32, f32){
        let(x, y, z) = arrow.get_dimension();
        (x, y)
    }
    fn get_xz(arrow: &Self) -> (f32, f32){
        let (x, y, z) = arrow.get_dimension();
        (x, z)
    }
    fn get_yx(arrow: &Self) -> (f32, f32){
        let (x, y, z) = arrow.get_dimension();
        (y, x)
    }
    fn get_yz(arrow: &Self) -> (f32, f32){
        let (x, y, z) = arrow.get_dimension();
        (y, z)
    }
    fn get_zx(arrow: &Self) -> (f32, f32){
        let (x, y, z) = arrow.get_dimension();
        (z, x)
    }
    fn get_zy(arrow: &Self) -> (f32, f32){
        let (x, y, z) = arrow.get_dimension();
        (z, y)
    }
}
