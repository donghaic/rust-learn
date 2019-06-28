use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}


fn main() {
    let point = Point { x: 1, y: 2 };
    println!("point => {:?}", point);

    let json_str = serde_json::to_string(&point).unwrap();
    println!("json str => {}", json_str);

    let deserialized: Point = serde_json::from_str(json_str.as_str()).unwrap();
    println!("deserialized point => {:?}", deserialized);
}
