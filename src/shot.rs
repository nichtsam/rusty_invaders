use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::Drawable;

pub struct Shot {
    x: usize,
    y: usize,
    // is_exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            // is_exploding: false,
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn is_dead(&self) -> bool {
        self.y == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "|";
    }
}
