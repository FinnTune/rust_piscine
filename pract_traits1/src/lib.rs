pub trait Printable {
    fn print(&self);
}

pub struct Book {
    pub title: String,
    pub author: String,
}

pub struct Movie {
    pub title: String,
    pub director: String,
}

impl Printable for Book {
    fn print(&self){
        println!("\"{}\" by author, {}", self.title, self.author);
    }
}

impl Printable for Movie {
    fn print(&self){
        println!("\"{}\" by director, {}", self.title, self.director);
    }
}

