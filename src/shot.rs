use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::Drawable;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub is_exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            is_exploding: false,
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.is_exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.is_exploding = true;
        self.timer = Timer::from_millis(250);
    }
    pub fn is_dead(&self) -> bool {
        (self.is_exploding && self.timer.ready) || self.y == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = if self.is_exploding { "*" } else { "|" };
    }
}
