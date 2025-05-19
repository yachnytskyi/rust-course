mod content;

use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let audiobook = Media::AudioBook {
        title: String::from("An audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Goov Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let good_book = Media::Book {
        title: String::from("Good Book"),
        author: String::from("Good Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}",good_movie.description());
    // println!("{}",bad_book.description());
    // println!("{}",good_book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(good_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(7);
    let placeholder = Media::Placeholder;

    println!("{:#?}", item.unwrap_or(&placeholder));
}
