use std::io;

trait Drawable {
    fn should_draw(&self, x: i32, y: i32) -> Option<char>;
}


struct Snack {
    x: i32,
    y: i32,
}

impl Drawable for Snack {
    fn should_draw(&self, x: i32, y: i32) -> Option<char> {
        if *&self.x == x && *&self.y == y {
            return Some('S');
        };
        return None
    }
}

struct BodyPart {
    x: i32,
    y: i32,
}

impl Drawable for BodyPart {
    fn should_draw(&self, x: i32, y: i32) -> Option<char> {
        if *&self.x == x && *&self.y == y {
            return Some('o');
        };
        return None
    }
}

struct Snake {
    body: Vec<BodyPart>,
}

impl Drawable for Snake {
    fn should_draw(&self, x: i32, y: i32) -> Option<char> {
        for body_part in &self.body {
            let body_part_should_draw = body_part.should_draw(x, y);
            // return match body_part_should_draw {
            //     Some(x) => {
            //         // if ind == 0 {
            //         //     Some(x.to_uppercase());
            //         // }
            //         Some(x)
            //     },
            //     None => {
            //         None
            //     },
            // }
            if body_part_should_draw.is_some() {
                if std::ptr::eq(body_part, self.body.first().unwrap()) {
                    return Some('0');
                }
                return body_part_should_draw;
            }
        }

        None
    }
}

fn render_field(snake: &Snake, snack: &Snack) {
    let h = 10;
    let w = 15;

    for i in 0..h {
        let mut row = String::from("|");
        for j in 0..w {
            let mut symbol_to_draw = snack.should_draw(j, i);
            match snake.should_draw(j, i) {
                Some(symbol) => {symbol_to_draw = Some(symbol)},
                None => {},
            }
            match symbol_to_draw {
                None => {
                    row.push_str("_|");
                },
                Some(sym) => {
                    row.push_str(format!("{}|", sym).as_str());
                }
            };
        }

        println!("{}", row);
    }
}


fn main() {
    let snack = Snack {
        x: 3,
        y: 5
    };
    let snake = Snake {
        body: vec![
            BodyPart {
                x: 1,
                y: 2,
            },
            BodyPart {
                x: 1,
                y: 3,
            },
            BodyPart {
                x: 1,
                y: 4,
            },
        ]
    };

    loop {
        render_field(&snake, &snack);

        let allowed_symbols = ['w', 'a', 's', 'd'];
        
        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("Failed to read line");
        
        if !allowed_symbols.contains(&next_move.chars().next().expect("string is empty")) {
            panic!("Wrong symbol")
        }
    }
}
