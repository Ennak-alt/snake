use crate::utils::*;
use crate::wasm4::*;

static SNAKE_SIZE: u8 = 5;

pub struct Snake {
    pub snake_head: Point,
    pub snake_body: Vec<Point>,
    tail_has_moved: bool,
    pub direction: Direction,
    pub direction_before_change: Direction,
    moved_since_direction_change: bool,
}

impl Snake {
    pub fn new(p: Point, dir: Direction) -> Snake {
        Snake {
            snake_head: p,
            snake_body: vec![p],
            tail_has_moved: false,
            direction: dir,
            direction_before_change: dir,
            moved_since_direction_change: false,
        }
    }

    pub fn change_dir(&mut self, dir: Direction) {
        match (dir, self.direction) {
            (Direction::Left, Direction::Right) | 
            (Direction::Right, Direction::Left) |
            (Direction::Up, Direction::Down) |
            (Direction::Down, Direction::Up) => {},
            (_, _) if dir == self.direction_before_change && !self.moved_since_direction_change => {}
            (_, _) => self.direction = dir,
        }
    }

    pub fn move_snake(&mut self) {
        self.moved_since_direction_change = true;
        self.direction_before_change = self.direction;
        let mut tail = None;
        if let Some(&p) = self.snake_body.last() {
            tail = Some(p);
        };
        let mut p = self.snake_head;
        self.snake_head = {
            match self.direction {
                Direction::Left => Point {
                    x: if p.x == 0 { WINDOW_SIZE-5 } else { p.x - SNAKE_SIZE },
                    y: p.y,
                },
                Direction::Right => Point {
                    x: if p.x == WINDOW_SIZE-5 { 0 } else { p.x + SNAKE_SIZE },
                    y: p.y,
                },
                Direction::Down => Point {
                    x: p.x,
                    y: if p.y == WINDOW_SIZE-5 { 0 } else { p.y + SNAKE_SIZE },
                },
                Direction::Up => Point {
                    x: p.x,
                    y: if p.y == 0 { WINDOW_SIZE-5 } else { p.y - SNAKE_SIZE },
                },
            }
        };
        self.snake_body = self
            .snake_body
            .iter()
            .map(|cp| {
                let sp = p;
                p = *cp;
                sp
            })
            .collect();
        if !self.tail_has_moved {
            self.snake_body.pop().unwrap();
            self.snake_body.push(tail.unwrap());
            self.tail_has_moved = true;
        }
    }

    pub fn eat(&mut self) {
        if let Some(&p) = self.snake_body.last() {
            self.snake_body.push(p);
            self.tail_has_moved = false;
        }
    }

    pub fn render(&mut self) {
        unsafe {
            *DRAW_COLORS = 0x43;
        }
        rect(
            self.snake_head.x as i32,
            self.snake_head.y as i32,
            SNAKE_SIZE as u32,
            SNAKE_SIZE as u32,
        );
        unsafe {
            *DRAW_COLORS = 2;
        }
        self.snake_body.iter().for_each(|p| {
            rect(p.x as i32, p.y as i32, SNAKE_SIZE as u32, SNAKE_SIZE as u32);
        });
    }
}
