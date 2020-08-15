extern crate nalgebra as na;
use na::{IsometryMatrix2, Vector2};

fn main() {
    let tf = |x, y, rot| IsometryMatrix2::new(Vector2::new(x, y), rot);

    let mut tree = mulberry::Tree::<&str, IsometryMatrix2<f32>>::default();

    tree.set_transform(tf(1.0, 0.0, 3.14), "/f1", "/f2");
    tree.set_transform(tf(1.0, 0.0, 0.0), "/f3", "/f2");
    tree.set_transform(tf(2.0, 1.0, 3.14 / 2.), "/f3", "/f4");
    tree.set_transform(tf(5.0, 3.0, 3.14 / 2.), "/f5", "/f3");

    tree.set_transform(tf(0.0, 7.0, 0.0), "/f6", "/f7");
    tree.set_transform(tf(0.0, 7.0, 0.0), "/f6", "/f8");
    tree.set_transform(tf(0.0, 7.0, 0.0), "/f8", "/f9");

    println!("{}", tree.output_dot("separate"));

    tree.set_transform(tf(0.0, 7.0, 0.0), "/f9", "/f5");

    println!("{}", tree.output_dot("connected"));

    dbg!(tree.get_path("/f6", "/f5"));
    dbg!(tree.get_path("/f1", "/f1"));
    dbg!(tree.get_path("/fake", "/fake"));
    dbg!(tree.get_path("/f1", "/fake"));
    dbg!(tree.get_transform("/f6", "/f5"));
}
