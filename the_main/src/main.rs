use comp_fns::geometry;

fn main() {
    let a = geometry::Point { x: 5i32, y: 10i32 };

    let b = geometry::Point {
        x: 1.5f64,
        y: 5.1f64,
    };

    let c = geometry::Point {
        x: 7i32,
        y: 11.1f64,
    };

    println!("The point of two integers: ({}, {})", a.x(), a.y());
    println!("The point of two reals: ({}, {})", b.x(), b.y());
    println!("The point of integer and real: ({}, {})", c.x(), c.y());
}
