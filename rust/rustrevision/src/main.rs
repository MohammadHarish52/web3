trait Printable {
    fn print(&self);
}

struct Tweet {
    user: String,
    text: String,
}

struct ImagePost {
    url: String,
}

fn show_post<T: Printable>(item: T) {
    item.print();
}

impl Printable for Tweet {
    fn print(&self) {
        println!("Tweet by {} :{}", self.user, self.text)
    }
}

impl Printable for ImagePost {
    fn print(&self) {
        println!("Image :{}", self.url)
    }
}

fn main() {
    let tweet1 = Tweet {
        user: "Harish".to_string(),
        text: "Harish".to_string(),
    };
    let img = ImagePost {
        url: "Everyone is gay".to_string(),
    };

    show_post(tweet1);
    show_post(img);
}
