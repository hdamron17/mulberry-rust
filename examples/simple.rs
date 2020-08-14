extern crate nalgebra as na;
use na::IsometryMatrix3;

fn main() {
    let transform = IsometryMatrix3::translation(0.0, 1.0, 3.0);
    let mut tree = mulberry::Tree::<&str, IsometryMatrix3<f32>>::default();
    tree.set_transform(transform, "/f1", "/f2");
    let inv_transform = tree.get_direct_transform("/f2", "/f1");
    dbg!(inv_transform);
}
