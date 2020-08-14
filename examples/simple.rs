extern crate nalgebra as na;
use na::IsometryMatrix3;

fn main() {
    let transform = IsometryMatrix3::translation(0.0, 1.0, 3.0);
    let mut tree = mulberry::Tree::<&str, IsometryMatrix3<f32>>::default();
    tree.set_transform(transform, "/f1", "/f2");
    tree.set_transform(transform, "/f3", "/f2");
    tree.set_transform(transform, "/f4", "/f3");
    tree.set_transform(transform, "/f5", "/f2");
    tree.set_transform(transform, "/f6", "/f4");

    dbg!(tree.get_path("/f6", "/f5"));
    dbg!(tree.get_path("/f1", "/f1"));
    dbg!(tree.get_path("/fake", "/fake"));
    dbg!(tree.get_path("/f1", "/fake"));
    dbg!(tree.get_transform("/f6", "/f5"));
}
