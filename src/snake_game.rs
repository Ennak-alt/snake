use crate::snake::*;
use crate::utils::*;
use crate::wasm4::*;
use fastrand::Rng;

pub struct SnakeGame {
    snake: Snake,
    points: i32,
    apple: Option<Point>,
    frame: usize,
    rng: Rng,
}

impl SnakeGame {
    pub fn new() -> SnakeGame {
        SnakeGame {
            snake: Snake::new(Point { x: 0, y: 0 }, Direction::Down),
            points: 0,
            apple: None,
            frame: 0,
            rng: Rng::with_seed(256),
        }
    }

    fn spawn_apple(&mut self) {
        //let d = fastrand::alphanumeric();
        //trace(format!("{}", d));
        let mut p = Point {
            x: self.rng.u8(0..20),
            y: self.rng.u8(0..20),
        };
        while (Point {
            x: p.x * 5,
            y: p.y * 5,
        }) == self.snake.snake_head || self.snake.snake_body.contains(&Point {
                x: p.x * 5,
                y: p.y * 5,
            })
        {
            if p.y == 20 + 1 {
                p = Point { x: 0, y: 0 };
            } else if p.x == WINDOW_SIZE + 1 {
                p = Point { x: 0, y: p.y + 1 };
            } else {
                p = Point { x: p.x + 1, ..p };
            }
        }
        p.x *= 5;
        p.y *= 5;
        self.apple = Some(p);
    }

    fn lost(&mut self) {
        self.snake = Snake::new(Point { x: 0, y: 0 }, Direction::Down );
        self.apple = None;
        self.points = 0;
        self.frame = 0;
        self.rng = Rng::with_seed(256)
    }

    pub fn update(&mut self) {
        self.snake.render();
        if self.frame == 0 {
            self.snake.move_snake();
        }
        self.frame += 1;
        if self.frame > 3 {
            self.frame = 0;
        }
        
        // Can still go bacwards if there you are quick enough
        let gamepad: u8 = unsafe { *GAMEPAD1 };
        if gamepad & BUTTON_RIGHT != 0 {
            self.snake.change_dir(Direction::Right);
        } else if gamepad & BUTTON_LEFT != 0 {
            self.snake.change_dir(Direction::Left);
        } else if gamepad & BUTTON_UP != 0 {
            self.snake.change_dir(Direction::Up);
        } else if gamepad & BUTTON_DOWN != 0 {
            self.snake.change_dir(Direction::Down);
        }

        if self.snake.snake_body.contains(&self.snake.snake_head) {
            self.lost();
        }

        if self.apple == None {
            self.spawn_apple();
        } else if self.apple.unwrap() == self.snake.snake_head {
            self.snake.eat();
            self.points += 1;
            self.spawn_apple();
        } else {
            unsafe {
                *DRAW_COLORS = 4;
            }
            rect(
                self.apple.unwrap().x as i32,
                self.apple.unwrap().y as i32,
                5,
                5,
            );
        }

        text(format!("{}", self.points), WINDOW_SIZE as i32-10 - (f32::log10(self.points as f32) as i32) * 8, 2);
    }
}
