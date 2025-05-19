#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::AudioBook { title } = self {
        //     format!("AudioBook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::AudioBook { title } => {
                format!("AudioBook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcasts: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }

        // match self {
        //     Media::Book { title, author } => format!("Book: {} {}", title, author),
        //     Media::Movie { title, director } => format!("Movie: {} {}", title, director),
        //     Media::AudioBook { title } => format!("AudioBook: {}", title),
        // }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good! We have something to return
            Some(&self.items[index])
        } else {
            // Bad! We don't have anything to return!!!
            None
        }
    }
}

fn print_media(media: &Media) {
    println!("{:?}", media)
}

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

    // print_media(&audiobook);
    // print_media(&good_movie);
    // print_media(&bad_book);
    // print_media(&good_book);

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
