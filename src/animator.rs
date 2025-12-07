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
}

const FPS: u32 = 60;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS as u64);
impl Animator {
    pub fn new() -> Self {
        let stdout = stdout();
        Animator { stdout }
    }

    pub fn animate(
        &mut self,
        solution: impl FnOnce(&mut dyn FnMut(Vec<String>)) -> String,
    ) -> String {
        self.stdout.execute(EnterAlternateScreen).unwrap();
        self.stdout.execute(Hide).unwrap();
        let mut now = Instant::now();
        let mut cb = |current_state: Vec<String>| {
            if let Err(e) = (|| -> Result<(), std::io::Error> {
                self.stdout.execute(Clear(ClearType::All))?;
                self.stdout.execute(MoveTo(0, 0))?;

                for (y, row) in current_state.iter().enumerate() {
                    self.stdout.execute(MoveTo(0, y as u16))?;
                    write!(self.stdout, "{}", row)?;
                }
                let elapsed = now.elapsed();
                if elapsed < FRAME_DURATION {
                    thread::sleep(FRAME_DURATION - elapsed);
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
