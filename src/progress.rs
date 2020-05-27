use std::io::Write;

use crate::stats::PIXELS_RENDERED;

const BARS: usize = 50;
const DONE: &'static str = "█";
const REST: &'static str = "░";

const HIDE: &'static str = "\x1B[?25l";
const SHOW: &'static str = "\x1B[?25h";
const SAVE: &'static str = "\x1B[s";
const RESTORE: &'static str = "\x1B[u";

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
        | Spinner::A => '-',
        | Spinner::B => '\\',
        | Spinner::C => '|',
        | Spinner::D => '/',
        };
        write!(fmt, "{}", spinner)
    }
}

pub fn run(total: usize) -> Result<(), std::io::Error> {
    let start = std::time::Instant::now();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut spinner = Spinner::A;
    let mut rendered = 0;

    write!(out, "{}{}", HIDE, SAVE)?;

    while rendered < total {
        spinner.rotate();
        rendered = PIXELS_RENDERED.read();

        let now = std::time::Instant::now();
        let mut span = now.duration_since(start).as_secs();
        let h = span / 3600; span %= 3600;
        let m = span / 60;   span %= 60;
        let s = span;
        let done = rendered * BARS / total;
        let rest = BARS - done;

        write!(
            out,
            "{}[{}] | Elapsed: {:0>2}:{:0>2}:{:0>2} | [{}{}] | {:.2}% | {} out of {} pixels",
            RESTORE,
            spinner,
            h, m, s,
            DONE.repeat(done),
            REST.repeat(rest),
            100.0 * rendered as f32 / total as f32,
            rendered,
            total,
        )?;

        std::thread::sleep(
            std::time::Duration::from_millis(10)
        );
    }

    writeln!(out, "{}", SHOW)?;

    Ok(())
}
