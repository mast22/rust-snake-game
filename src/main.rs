use std::io::{self, Write};

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

enum Direction {
    Up, Right, Left, Down
}

struct Snake {
    body: Vec<BodyPart>,
    direction: Direction,
}

impl Drawable for Snake {
    fn should_draw(&self, x: i32, y: i32) -> Option<char> {
        for body_part in &self.body {
            let body_part_should_draw = body_part.should_draw(x, y);
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

impl Snake {
    fn move_snake(&mut self, direction: Direction) {
        let last_body_part = self.body.first().unwrap();
        let new_body_part = match direction {
            Direction::Up => {
                BodyPart {
                    x: last_body_part.x,
                    y: last_body_part.y + 1
                }
            },
            Direction::Right => {
                BodyPart {
                    x: last_body_part.x + 1,
                    y: last_body_part.y
                }
            },
            Direction::Left => {
                BodyPart {
                    x: last_body_part.x - 1,
                    y: last_body_part.y 
                }
            },
            Direction::Down => {
                BodyPart {
                    x: last_body_part.x,
                    y: last_body_part.y - 1
                }
            },
        };
        self.body.push(new_body_part)
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
    let mut snake = Snake {
        body: vec![
            BodyPart {
                x: 3,
                y: 1,
            },
            BodyPart {
                x: 2,
                y: 1,
            },
            BodyPart {
                x: 1,
                y: 1,
            },
        ],
        direction: Direction::Right
    };

    loop {
        render_field(&snake, &snack);

        let mut next_move = String::new();
        io::stdout().flush().expect("Some error");
        io::stdin().read_line(&mut next_move).expect("Failed to read line");

        match next_move.as_str() {
            "w\n" => snake.move_snake(Direction::Up),
            "s\n" => snake.move_snake(Direction::Down),
            "d\n" => snake.move_snake(Direction::Right),
            "a\n" => snake.move_snake(Direction::Left),
            _ => { panic!("Wrong direction given"); }
        };
    }
}
