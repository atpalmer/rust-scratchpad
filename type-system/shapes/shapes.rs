

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

fn print_area(s: &dyn Shape) {
    println!("Shape area: {}", s.area());
}

struct UnspecifiedShape {}

impl Shape for UnspecifiedShape {
    fn area(&self) -> f64 {
        return 5.;
    }

    fn perimeter(&self) -> f64 {
        return 7.;
    }
}

struct Point {
    x: f64,
    y: f64,
}

fn display(point: Point) {
    println!("Point x: {}, y: {}", point.x, point.y);
}

trait Polygon {
    fn side(&self) -> f64;
}

struct Square {
    side: f64,
}

impl Polygon for Square {
    fn side(&self) -> f64 {
        return self.side;
    }
}

fn print_side(polygon: &dyn Polygon) {
    println!("Side: {}", polygon.side());
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.side * self.side;
    }

    fn perimeter(&self) -> f64 {
        return 4. * self.side;
    }
}

fn main() {
    let shape = UnspecifiedShape {};
    print_area(&shape);
    let point = Point { x: 3., y: 4. };
    display(point);
    let square = Square { side: 10. };
    print_side(&square);
    print_area(&square);
}
