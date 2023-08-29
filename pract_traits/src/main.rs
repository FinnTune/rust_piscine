trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: isize,
}

struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        for y in 0..=2*self.radius {
            for x in 0..=2*self.radius {
                // Using a basic circle equation (x - h)^2 + (y - k)^2 = r^2
                // where (h, k) is the center of the circle.
                let h = self.radius;
                let k = self.radius;
                let x_current = x - h;
                let y_current = y - k;

                if x_current * x_current + y_current * y_current <= self.radius*self.radius {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

fn render<T: Drawable>(item: T) {
    item.draw();
}

fn main() {
    let circle = Circle { radius: 10 };
    let square = Square;

    render(circle);
    render(square);
}
