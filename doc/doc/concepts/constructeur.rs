impl Point {
    /// Initialise un point à partir de coordonnées cartésiennes.
    pub fn from(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
}

let point1 = Point::from(1.0, 2.0);
assert_eq!(point1.abscissa(), 1.0);
assert_eq!(point1.ordinate(), 2.0);

let point2 = Point::from(1.0, 2.0);
assert_eq!(point2.abscissa(), 1.0);
assert_eq!(point2.ordinate(), 2.0);

let circle1 = Circle::from(point1, 2.0); 
assert_eq!(circle1.center(), &Point::from(1.0, 2.0));
assert_eq!(circle1.radius(), 2.0);