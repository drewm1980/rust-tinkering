struct Point {
    x: f64,
    y: f64,
}
struct Rectangle {
    ul: Point,
    lr: Point,
}
struct Triangle(Point, Point, Point);

struct Circle {
    center: Point,
    r: f64,
}

fn point_distance(p1: &Point, p2: &Point) -> f64 {
    let x_d = p1.x - p2.x;
    let y_d = p1.y - p2.y;

    //println!("{},{}",x_d,y_d);
    (x_d * x_d + y_d * y_d).sqrt()
}

enum Shape { Rectangle, Circle, Point, }

fn info(s: Shape) {
    match s {
        Rectangle => println!("Rectangle"),
        Circle => println!("Circle"),
        Point => println!("Point")
    }
}
/*
fn dist(s1:&Shape, s2:&Shape){
	match (s1,s2) {
		(s1:&Circle{center:c1,r:r1},
		 s2:&Circle{center:c2,r:r2})
		=> (dist(r1,r2)) - (r1+r2),
		(s1:&Point, s2:&Point)
		=> compute_distance(s1,s2)
	}
}
*/


fn main() {
    let origin = Point{x: 0.0, y: 0.0,};
    let p1 = Point{x: 0.0, y: 3.0,};
    let p2 = Point{x: 4.0, y: 0.0,};
    let d = point_distance(&p1, &p2);
    println!("d = {}" , d);

    let p3: Box<Point> = box() Point{x: 0.0, y: 3.0,};
    let d = point_distance(p3, &p2);
    println!("d = {}" , d);

    let c = Circle{center: p1, r: 10.0,};
    let t = Triangle(p1, p1, p2);
    let bar: (Point, Point, Point);

}
