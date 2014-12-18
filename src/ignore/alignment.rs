struct Point {
    x: f64,
    y: f64,
}

enum MaybePoint { ThePoint(Point), Nil, }

fn main() {
    let p1 = Point{x: 1.0, y: 2.0,};
    let p2 = box() Point{x: 3.0, y: 4.0,};
    let p3 = ThePoint(Point{x: 5.0, y: 6.0,});
    let p4 = box() ThePoint(Point{x: 5.0, y: 6.0,});
    println!("Point on stack: {}" , & p1);
    println!("Point on heap: {}" , & p2);
    println!("MaybePoint on stack: {}" , & p3);
    println!("MaybePoint on heap: {}" , & p4);
}
