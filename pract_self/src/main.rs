struct Point {
    x: i32,
    y: i32,
}

trait Display {
    fn display(&self);
}

impl Display for Point {
    fn display(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}


impl Point {
    // Takes ownership of the instance and returns a new Point
    fn move_right(mut self, dx: i32) -> Point {
        self.x += dx;
        self
    }

    // Borrows self immutably to print the coordinates
    fn print_coordinates(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }

    // Borrows self mutably to modify the coordinates
    fn shift(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

enum Message {
    Hello,
    Bye,
}

impl Message {
    // Uses self in match patterns with an enum
    fn display(&self) {
        match self {
            Self::Hello => println!("Hello, Rust!"),
            Self::Bye => println!("Goodbye, Rust!"),
        }
    }
}

fn main() {
    let mut p = Point { x: 5, y: 10 };
    p.print_coordinates();  // prints: x: 5, y: 10

    p.display();  // prints: Point(5, 10)

    p.shift(3, 4);
    p.print_coordinates();  // prints: x: 8, y: 14

    let p2 = p.move_right(2);
    p2.print_coordinates();  // prints: x: 10, y: 14

    let hello_message = Message::Hello;
    hello_message.display();  // prints: Hello, Rust!

    let bye_message = Message::Bye;
    bye_message.display();  // prints: Goodbye, Rust!
}
