pub trait Perimeter {
    fn perimeter(&self) -> f64;
}

pub trait Area {
    fn area(&self) -> f64;
}

pub trait ShapeName {
    fn name(&self) -> &'static str;
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.height + self.width)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Perimeter for Triangle {
    fn perimeter(&self) -> f64 {
        self.base + self.height + (self.base.powf(2.0) + self.height.powf(2.0)).sqrt()
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
}

impl ShapeName for Rectangle {
    fn name(&self) -> &'static str {
        "Rectangle"
    }
}

impl ShapeName for Triangle {
    fn name(&self) -> &'static str {
        "Triangle"
    }
}

impl ShapeName for Circle {
    fn name(&self) -> &'static str {
        "Circle"
    }
}

pub fn print_info<T: Perimeter + Area + ShapeName>(shape: &T) {
    println!("Shape: {}", shape.name());
    println!("Perimeter: {}", shape.perimeter());
    println!("Area: {}", shape.area());
    println!();
}
