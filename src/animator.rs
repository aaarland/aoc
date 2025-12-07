use std::{
    io::{Stdout, Write, stdout},
    thread::{self},
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Animator {
    stdout: Stdout,
    fps: Option<usize>,
}

impl Animator {
    pub fn new(fps: Option<usize>) -> Self {
        let stdout = stdout();
        Animator { stdout, fps }
    }

    pub fn animate(&mut self, solution: impl FnOnce(&mut dyn FnMut(Vec<String>)) -> String,) -> String {
        let Some(fps) = self.fps else { return solution(&mut |_| {}) };
        self.stdout.execute(EnterAlternateScreen).unwrap();
        self.stdout.execute(Hide).unwrap();
        let mut now = Instant::now();
        let frame_duration: Duration = Duration::from_millis(1000 / fps as u64);
        let mut cb = |current_state: Vec<String>| {
            if let Err(e) = (|| -> Result<(), std::io::Error> {
                self.stdout.execute(Clear(ClearType::All))?;
                self.stdout.execute(MoveTo(0, 0))?;

                for (y, row) in current_state.iter().enumerate() {
                    self.stdout.execute(MoveTo(0, y as u16))?;
                    write!(self.stdout, "{}", row)?;
                }
                let elapsed = now.elapsed();
                if elapsed < frame_duration {
                    thread::sleep(frame_duration - elapsed);
                }
                Ok(())
            })() {
                eprintln!("Error! {}", e);
            }
            now = Instant::now();
        };
        let res = solution(&mut cb);
        self.stdout.execute(Show).unwrap();
        self.stdout.execute(LeaveAlternateScreen).unwrap();
        res
    }
}
