use core::panic;
use rand::Rng;
use std::collections::VecDeque;
use std::io::{self, Write};

const H: i32 = 10;
const W: i32 = 15;

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
        return None;
    }
}

impl Snack {
    fn new_place(&mut self, snake: &Snake) {
        let mut rng = rand::thread_rng();

        let mut new_x: i32;
        let mut new_y: i32;

        // do while
        while {
            new_x = rng.gen_range(0..W);
            new_y = rng.gen_range(0..H);

            snake.collide(new_x, new_y)
        } {}

        self.x = new_x;
        self.y = new_y;
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
        return None;
    }
}

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

struct Snake {
    body: VecDeque<BodyPart>,
    direction: Direction,
    skip_next_pop: bool, // removal need to be skipped in case snake was feed
}

impl Drawable for Snake {
    fn should_draw(&self, x: i32, y: i32) -> Option<char> {
        for body_part in &self.body {
            let body_part_should_draw = body_part.should_draw(x, y);
            if body_part_should_draw.is_some() {
                if std::ptr::eq(body_part, self.body.front().unwrap()) {
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
        let head = self.body.front().unwrap();
        let new_body_part = match direction {
            Direction::Up => (head.x, head.y - 1),
            Direction::Right => (head.x + 1, head.y),
            Direction::Left => (head.x - 1, head.y),
            Direction::Down => (head.x, head.y + 1),
        };
        self.body.push_front(BodyPart {
            x: new_body_part.0,
            y: new_body_part.1,
        });
        if !self.skip_next_pop {
            self.body.pop_back();
        } else {
            self.skip_next_pop = false;
        }
    }

    fn feed(&mut self, snack: &mut Snack) {
        let head = self.body.front().unwrap();
        if snack.x == head.x && snack.y == head.y {
            self.skip_next_pop = true;
            snack.new_place(&self);
        }
    }

    fn collide(&self, x: i32, y: i32) -> bool {
        for body_part in &self.body {
            if body_part.x == x && body_part.y == y {
                return true;
            }
        }

        false
    }

    fn check_for_game_over(&self) {
        let head = self.body.front().unwrap();
        // if self.collide(head.x, head.y) {
        //     panic!("You ate yourself, Uroboros!");
        // }

        if head.x <= 0 || head.x >= W || head.y <= 0 || head.y >= H {
            panic!("Out of bounds!");
        }
    }
}

fn render_field(snake: &Snake, snack: &Snack) {
    for i in 0..H {
        let mut row = String::from("|");
        for j in 0..W {
            let mut symbol_to_draw = snack.should_draw(j, i);
            match snake.should_draw(j, i) {
                Some(symbol) => symbol_to_draw = Some(symbol),
                None => {}
            }
            match symbol_to_draw {
                None => {
                    row.push_str("_|");
                }
                Some(sym) => {
                    row.push_str(format!("{}|", sym).as_str());
                }
            };
        }

        println!("{}", row);
    }
}

fn main() {
    let mut snack = Snack { x: 3, y: 5 };

    let mut snake_body = VecDeque::new();
    snake_body.push_front(BodyPart { x: 1, y: 1 });
    snake_body.push_front(BodyPart { x: 2, y: 1 });
    snake_body.push_front(BodyPart { x: 3, y: 1 });

    let mut snake = Snake {
        body: snake_body,
        direction: Direction::Right,
        skip_next_pop: false,
    };

    loop {
        snake.check_for_game_over();
        render_field(&snake, &snack);

        let mut next_move = String::new();
        io::stdout().flush().expect("Some error");
        io::stdin()
            .read_line(&mut next_move)
            .expect("Failed to read line");

        snake.feed(&mut snack);
        match &*next_move.as_str().trim().replace("\n", "") {
            "w" => snake.move_snake(Direction::Up),
            "s" => snake.move_snake(Direction::Down),
            "d" => snake.move_snake(Direction::Right),
            "a" => snake.move_snake(Direction::Left),
            _ => {
                panic!("Wrong direction {} given", next_move);
            }
        };
    }
}
