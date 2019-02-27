use crossbeam::channel;
use sdl2::event;
use sdl2::keyboard;
use sdl2::pixels;

pub type Message = (usize, usize, (u8, u8, u8));
pub type Rx = channel::Receiver<Message>;
pub type Tx = channel::Sender<Message>;

pub struct Preview {
    nx: u32,
    ny: u32,
    rx: Rx,
}

impl Preview {
    pub fn new(nx: usize, ny: usize, rx: Rx) -> Self {
        Preview { nx: nx as u32, ny: ny as u32, rx }
    }

    pub fn run(self) {
        let context = sdl2::init().unwrap();   
        let video = context.video().unwrap();
        let mut canvas = video.window("Rendering Preview", self.nx, self.ny)
            .position_centered()
            .build()
            .unwrap()
            .into_canvas()
            .build()
            .unwrap();

        let mut received = 0;
        let mut events = context.event_pump()
            .unwrap();

        loop {
            match events.poll_event() {
            | Some(event::Event::Quit { .. })
            | Some(event::Event::KeyDown { keycode: Some(keyboard::Keycode::Escape), .. }) => {
                return
            }
            | _ => (),
            }

            if let Ok((x, y, (r, g, b))) = self.rx.try_recv() {
                let y = self.ny as usize - y;
                received += 1;
                canvas.set_draw_color(pixels::Color::RGB(r, g, b));
                canvas.draw_point((x as i32, y as i32)).unwrap();
                if received % 1000 == 0 { canvas.present(); }
            }
        }
    }
}
