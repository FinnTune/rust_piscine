use pract_traits1::*;

fn main() {
    let my_book = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
    };

    let my_movie = Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
    };

    my_book.print();
    my_movie.print();
}