use std::io::Write;

use crate::stats::PIXELS_RENDERED;

const BARS: usize = 50;
const DONE: &'static str = "█";
const REST: &'static str = "░";

#[derive(Copy, Clone)]
enum Spinner {
    A, B, C, D
}

impl Spinner {
    fn rotate(&mut self) {
        *self = match self {
        | Spinner::A => Spinner::B,
        | Spinner::B => Spinner::C,
        | Spinner::C => Spinner::D,
        | Spinner::D => Spinner::A,
        };
    }
}

impl std::fmt::Display for Spinner {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let spinner = match self {
        | Spinner::A => '◷',
        | Spinner::B => '◶',
        | Spinner::C => '◵',
        | Spinner::D => '◴', 
        };
        write!(fmt, "{}", spinner)
    }
}

pub fn run(total: usize) -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut spinner = Spinner::A;
    write!(out, "{}", termion::cursor::Save)?;
    loop {
        write!(out, "{}", termion::cursor::Restore)?; 
        let rendered = PIXELS_RENDERED.read();
        let done = rendered * BARS / total;
        let rest = BARS - done;
        write!(
            out,
            "[{}] Rendering... | [{}{}] | {:.2}% | {} out of {} pixels",
            spinner,
            DONE.repeat(done),
            REST.repeat(rest),
            100.0 * rendered as f32 / total as f32,
            rendered,
            total,
        )?;
        spinner.rotate();
        std::thread::sleep(
            std::time::Duration::from_millis(100)
        );
    }
}
