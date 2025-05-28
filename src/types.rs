use crate::app::{AREA_HEIGHT, AREA_WIDTH};
use anyhow::Context;
use rand::{Fill, Rng};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use std::collections::VecDeque;

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Snake {
    pub body: VecDeque<Point>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    pub fn new(body: VecDeque<Point>) -> Self {
        Self {
            body,
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, &direction) {
            (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up) => {}
            _ => self.next_direction = direction,
        }
    }
    pub fn r#move(&mut self, food: &mut Point) -> anyhow::Result<()> {
        let front_point = self
            .body
            .front()
            .context("get snake head failed")?
            .clone()
            .moved(&self.next_direction);
        self.direction = self.next_direction.clone();

        if front_point == *food {
            self.body.push_front(front_point);
            while self.body.contains(food) {
                rand::rng().fill(food);
            }
        } else {
            self.body.push_front(front_point);
            _ = self.body.pop_back();
        }

        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Point {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Fill for Point {
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.x = rng.random_range(1..=AREA_WIDTH);
        self.y = rng.random_range(1..=AREA_HEIGHT);
    }
}

impl Point {
    fn r#move(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }

    fn moved(self, direction: &Direction) -> Point {
        let mut p = self.clone();
        p.r#move(direction);
        p
    }
}

impl Widget for Point {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // locate buf
        let buf_x = self.x;
        let buf_y = self.y.div_ceil(2);
        let is_upper_half = self.y % 2 == 1;
        let existed = &buf[(buf_x, buf_y)];
        match existed.symbol() {
            "█" => {}
            "▀" => {
                if !is_upper_half {
                    buf[(buf_x, buf_y)].set_symbol("█");
                }
            }
            "▄" => {
                if is_upper_half {
                    buf[(buf_x, buf_y)].set_symbol("█");
                }
            }
            _ => {
                if is_upper_half {
                    buf[(buf_x, buf_y)].set_symbol("▀");
                } else {
                    buf[(buf_x, buf_y)].set_symbol("▄");
                }
            }
        };
    }
}
