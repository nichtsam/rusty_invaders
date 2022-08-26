use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use invaders::frame::Drawable;
use invaders::player::Player;
use invaders::{frame, render};
use rusty_audio::Audio;
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "audio/explode.wav");
    audio.add("lose", "audio/lose.wav");
    audio.add("move", "audio/move.wav");
    audio.add("pew", "audio/pew.wav");
    audio.add("startup", "audio/startup.wav");
    audio.add("win", "audio/win.wav");

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (render_tx, render_rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut prev_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &prev_frame, &prev_frame, true);

        loop {
            let next_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &prev_frame, &next_frame, false);
            prev_frame = next_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    'game_loop: loop {
        // frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut next_frame = frame::new_frame();

        // polling keyboard input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'game_loop;
                    }
                    KeyCode::Left => {
                        player.move_left();
                    }
                    KeyCode::Right => {
                        player.move_right();
                    }
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    _ => (),
                }
            }
        }

        // Shots
        player.update(delta);

        // Render
        player.draw(&mut next_frame);
        let _ = render_tx.send(next_frame);
        thread::sleep(Duration::from_millis(10));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
