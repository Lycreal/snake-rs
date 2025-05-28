use crate::TICK_DURATION;
use crate::types::{Direction, Point, Snake};
use anyhow::{Context, Result};
use ratatui::backend::Backend;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::{Frame, Terminal};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const INITIAL_LEN: usize = 3;
pub const AREA_WIDTH: u16 = 15;
pub const AREA_HEIGHT: u16 = 15;

pub struct App {
    tick_duration: u64,
    current_tick: u64,
    next_tick: u64,

    pub start_at: Instant,

    food: Point,
    snake: Snake,
    game_over: bool,
}

impl App {
    pub fn new(tick_duration: u64) -> App {
        let snake = Snake::new(VecDeque::from([
            Point { x: 5, y: 4 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 4 },
        ]));
        let mut food = Point::default();
        rand::fill(&mut food);
        while snake.body.contains(&food) {
            rand::fill(&mut food);
        }
        App {
            tick_duration,
            current_tick: 0,
            next_tick: 0,
            start_at: Instant::now(),
            snake,
            food,
            game_over: false,
        }
    }

    pub fn next_tick(&mut self) -> Duration {
        let uptime = self.start_at.elapsed().as_nanos() as u64;
        let (tick_count, remainder) = (uptime / self.tick_duration, uptime % self.tick_duration);
        self.current_tick = tick_count;
        Duration::from_nanos(self.tick_duration - remainder)
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let direction = match key.code {
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            _ => return Ok(()),
        };
        self.snake.change_direction(direction);
        Ok(())
    }

    fn move_interval(&self) -> u64 {
        let speed = 300 + self.score() as u64 * 20; // move per 100 second
        let move_time = 100_000_000_000 / speed; // nano second
        move_time / self.tick_duration // tick count
    }

    fn score(&self) -> usize {
        self.snake.body.len() - INITIAL_LEN
    }

    fn check_game_over(&mut self) -> bool {
        if self.game_over {
            return true;
        }
        let Some(head) = self.snake.body.front() else {
            self.game_over = true;
            return true;
        };
        // hit wall
        if head.x == 0 || head.x > AREA_WIDTH || head.y == 0 || head.y > AREA_HEIGHT {
            self.game_over = true;
            return true;
        }
        // hit body
        for (i, x) in self.snake.body.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if head == x {
                self.game_over = true;
                return true;
            }
        }
        false
    }

    pub fn handle_tick_event<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        if self.game_over {
            return Ok(());
        }
        if self.current_tick < self.next_tick {
            return Ok(());
        }

        self.next_tick = self.current_tick + self.move_interval();
        self.snake
            .r#move(&mut self.food)
            .context("snake move failed")?;

        terminal
            .draw(|frame: &mut Frame| {
                frame.render_widget(
                    Block::bordered().title(format!("score: {}", self.score())),
                    Rect {
                        x: 0,
                        y: 0,
                        width: AREA_WIDTH + 2,
                        height: AREA_HEIGHT / 2 + 2,
                    },
                );

                frame.render_widget(self.food.clone(), Rect::default());
                for point in self.snake.body.iter() {
                    frame.render_widget(point.clone(), Rect::default());
                }

                if self.check_game_over() {
                    frame.render_widget(
                        "GAME OVER!",
                        Rect {
                            x: (AREA_WIDTH - "GAME OVER!".len() as u16) / 2 + 1,
                            y: AREA_HEIGHT / 4 + 1,
                            width: AREA_WIDTH,
                            height: 1,
                        },
                    );
                }
            })
            .context("draw frame failed")?;
        Ok(())
    }
}
