// Test

struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl Color {

    fn display() {
        println!("Coucou ceci est une couleur");
    }

    fn format(self, separator: char) -> String{

      return format!("{} {} {} {} {}", self.red, separator, self.green, separator, self.blue);
    }
}

fn main() {
    let red = Color{red: 255, green: 0, blue: 0, alpha:  255};

    Color::display();
    println!("{} ", red.format(';'));
}

