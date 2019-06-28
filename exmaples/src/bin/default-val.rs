#[derive(Debug, Default)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Point2 {
    fn default() -> Point2 {
        Point2 { x: 0, y: 0, z: 0 }
    }
}

fn main() {
    let p1 = Point::default();
    let p2 = Point { x: 34, ..Default::default() };

    println!("p1 => {:?}", p1);
    println!("p1 => {:?}", p2);
}
