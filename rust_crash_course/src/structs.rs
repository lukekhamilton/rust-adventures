struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// Tuple struct
struct Colorz(u8, u8, u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Colour: {} {} {}", c.red, c.green, c.blue);

    c.red = 200;

    println!("Colour: {} {} {}", c.red, c.green, c.blue);

    let mut cz = Colorz(255, 0, 0);

    cz.2 = 201;

    println!("Colour: {} {} {}", cz.0, cz.1, cz.2);

    let mut p = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Person: {}", p.full_name());

    p.set_last_name("Smith");

    println!("Person: {}", p.full_name());

    p.last_name = String::from("Banks");

    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.to_tuple());

    // TODO why does the next line fail?
    //    println!("Person: {}", p.full_name());

    // TODO What doesnt this work too?
    //    let tuple = &p.to_tuple();
    //    println!("tuple: {} {}", tuple.0, tuple.1);
}
