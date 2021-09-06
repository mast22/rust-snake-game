use kiss3d::camera::{Camera, FirstPerson};
use kiss3d::event::{Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::{geometry::Point, UnitQuaternion, Vector3};
use kiss3d::nalgebra::{Point3, Transform3, Translation3};
use kiss3d::window::Window;
use std::time::Duration;
use std::{thread, time};

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
        None
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

pub fn run() {
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

fn main() {
    let mut window = Window::new_with_size("Kiss3d: Cube", 700, 500);
    window.set_background_color(0.0, 0.0, 0.3);

    let mut field = vec![];

    let look_at_y = (H - 1) as f32 / 2.0;
    let look_at_x = (W - 1) as f32 / 2.0;

    let mut camera = FirstPerson::new(
        Point3::new(look_at_x, look_at_y, 15.0),
        Point3::new(look_at_x, look_at_y, 0.0),
    );

    for row in 0..W {
        for cell in 0..H {
            let mut cube = window.add_cube(1.0, 1.0, 0.0);
            cube.append_translation(&Translation3::new(row as f32, cell as f32, 0.0));
            if (row + cell) % 2 == 0 {
                cube.set_color(0.7, 0.7, 0.7);
            } else {
                cube.set_color(1.0, 1.0, 1.0);
            }
            field.push(cube);
        }
    }

    let mut head = window.add_cube(0.9, 0.9, 0.9);
    head.append_translation(&Translation3::new(5.0, 6.0, 1.1));
    head.set_color(0.3, 0.7, 0.3);

    window.set_light(Light::StickToCamera);

    let mut movement = &Translation3::new(1.0, 0.0, 0.0);

    while window.render_with_camera(&mut camera) {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(key, action, modif) => {
                    // movement = match key {
                    //     Key::D => &Translation3::new(1.0, 0.0, 0.0),
                    //     Key::A => &Translation3::new(-1.0, 0.0, 0.0),
                    //     Key::W => &Translation3::new(0.0, 1.0, 0.0),
                    //     Key::S => &Translation3::new(0.0, -1.0, 0.0),
                    //     _ => movement,
                    // };
                }
                _ => {}
            }
        }
        head.prepend_to_local_translation(movement);
    }
}
