Java-style vs

interface Movable { void moveBy(double dx, double dy); }

class Point implements Movable { ... }
class Circle implements Movable { ... }


trait Movable { fn move_by(&mut self, dx: f64, dy: f64); }

struct Point { x: f64, y: f64 }
struct Circle { center: Point, radius: f64 }

impl Movable for Point { ... }
impl Movable for Circle { ... }
