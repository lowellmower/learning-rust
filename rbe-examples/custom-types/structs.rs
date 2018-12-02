#![allow(dead_code)] // silences warnings on unused code which in this case
                     // would be the Pair and unit structs
use std::fmt;

// tuple structs
struct Pair(i32, i8);

// unit structs
struct Nil;

// C style struct
struct Point {
    x: f32,
    y: f32,
}

// use this as a custom printer versus the default debug
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {0}, y: {1})", self.x, self.y)
    }
}

// #[allow(dead_code)] <- can also be applied on singular object
// Nesting structs
struct GraphPlots {
    p1: Point,
    p2: Point,
}

fn main() {
    let point: Point = Point{x: 1.3, y: 3.2};
    println!("Coordinate: x -> {}, y -> {}", point.x, point.y);

    let graph: GraphPlots = GraphPlots {
        p1: point,
        p2: Point {x: 0.2, y: 1.1},
    };

    // our printer above let's us look at the nested struct
    println!("Graph points: One -> {}, Two -> {}", graph.p1, graph.p2);
}