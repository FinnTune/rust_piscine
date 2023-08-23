#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn main() {

    println!("Webevent: {:?}", WebEvent::KeyPress('x'));
}
