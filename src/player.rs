use std::time::Duration;

use crate::{frame::Drawable, invaders::Invaders, shot::Shot, NUM_COLS, NUM_ROWS};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 2,
            shots: Vec::new(),
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
    pub fn move_down(&mut self) {
        if self.y < NUM_ROWS {
            self.y += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            let shot = Shot::new(self.x, self.y - 1);
            self.shots.push(shot);
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.is_dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut has_hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.is_exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    has_hit_something = true;
                    shot.explode();
                }
            }
        }
        has_hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
