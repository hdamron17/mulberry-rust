extern crate nalgebra as na;
use na::{IsometryMatrix3};

fn main() {
    let transform = IsometryMatrix3::translation(0.0, 1.0, 3.0);
    let mut tree = mulberry::Tree::<String, IsometryMatrix3<f32>>::default();
    tree.setTransform(transform, "/f1".into(), "/f2".into());
    let inv_transform = tree.getDirectTransform("/f2".into(), "/f1".into());
    dbg!(inv_transform);
}
