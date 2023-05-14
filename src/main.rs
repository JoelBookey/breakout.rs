use piston_window::*;
use rodio::{source::Source, Decoder, OutputStream};

fn main() {
    // 9: 5 aspect ratio
    // each block is 42 x 5
    let window: PistonWindow = WindowSettings::new("Breakout", [630, 350])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(window);
    app.run();
}

struct App {
    window: PistonWindow,
    // (pos, colours)
    rect_values: (Vec<[f64; 4]>, Vec<[f32; 4]>),
    // x value
    player_pos: f64,
    player_speed: f64,

    ball_pos: (f64, f64),
    ball_speed: (f64, f64),
}

impl App {
    fn run(&mut self) {
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let bounce_file = "pong.mp3";

        while let Some(e) = self.window.next() {
            // bounce ball
            if self.ball_pos.0 <= 0.0 || self.ball_pos.0 + 10.0 >= 630.0 {
                self.ball_speed.0 *= -1.0;
            } else if self.ball_pos.1 <= 0.0 {
                self.ball_speed.1 *= -1.0;
            } else if self.ball_pos.1 + 10.0 >= 350.0 {
                println!("YOU DIED!");
                break;
            }

            self.collide_check();

            if self.ball_pos.0 + 10.0 >= self.player_pos
                && self.ball_pos.0 <= self.player_pos + 84.0
                && self.ball_pos.1 + 10.0 >= 320.0
            {
                let bounce_file_open =
                    std::io::BufReader::new(std::fs::File::open(&bounce_file).unwrap());
                let bounce_source = Decoder::new(bounce_file_open).expect("weird");
                let _ = stream_handle.play_raw(bounce_source.convert_samples());

                if self.ball_pos.1 + 10.0 <= 342.5 {
                    self.ball_speed.1 *= -1.01;
                }

                if self.ball_speed.0 >= 0.0 {
                    if self.ball_pos.0 + 5.0 <= self.player_pos + 42.0 {
                        self.ball_speed.0 *= -1.01;
                    }
                } else if self.ball_pos.0 + 5.0 >= self.player_pos + 42.0 {
                    self.ball_speed.0 *= -1.01;
                }
            }

            // player movement
            if !(self.player_pos + self.player_speed < 0.0)
                && !(self.player_pos + self.player_speed > 630.0 - 84.0)
            {
                self.player_pos += self.player_speed;
            }
            // ball movement
            self.ball_pos = (
                self.ball_pos.0 + self.ball_speed.0,
                self.ball_pos.1 + self.ball_speed.1,
            );

            self.window.draw_2d(&e, |context, graphics, _device| {
                clear([1.0; 4], graphics);
                for (index, rect) in self.rect_values.0.iter().enumerate() {
                    rectangle(
                        self.rect_values.1[index],
                        *rect,
                        context.transform,
                        graphics,
                    );
                }
                rectangle(
                    [0.0, 0.0, 0.0, 1.0],
                    [self.player_pos, 320.0, 84.0, 15.0],
                    context.transform,
                    graphics,
                );

                rectangle(
                    [1.0, 140.0 / 250.0, 151.0 / 255.0, 1.0],
                    [self.ball_pos.0, self.ball_pos.1, 10.0, 10.0],
                    context.transform,
                    graphics,
                );
            });
            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::Left => self.player_speed = -1.0,
                    Key::Right => self.player_speed = 1.0,
                    _ => (),
                };
            }

            if let Some(Button::Keyboard(key)) = e.release_args() {
                if (key == Key::Right && self.player_speed > 0.0)
                    || (key == Key::Left && self.player_speed < 0.0)
                {
                    self.player_speed = 0.0;
                }
            }

            if self.rect_values.0.is_empty() {
                println!("YOU WON!");
                break;
            }
        }
    }

    fn new(window: PistonWindow) -> App {
        App {
            window: window,
            rect_values: get_rect(),
            player_pos: 630.0 / 2.0,
            player_speed: 0.0,
            ball_pos: (630.0 / 2.0, 350.0 / 2.0),
            ball_speed: (0.2, -0.2),
        }
    }

    fn collide_check(&mut self) {
        let mut to_die: usize = 69420;
        for (indx, rect) in self.rect_values.0.iter().enumerate() {
            // if rect x left is less than ball right
            if rect[0] <= self.ball_pos.0 + 10.0
                        // if ball left is less than rect right 
                        && self.ball_pos.0 <=  rect[0] + rect[2]
                        // if rect bottom is greater than ball top
                        && rect[1] >= self.ball_pos.1 - 10.0
                        // if ball bottom is less than rect top
                        && self.ball_pos.1 >= rect[1] - rect[3]
            {
                println!("DEAD");
                if (self.ball_speed.0 + 5.0 > 0.0 && self.ball_pos.0 <= rect[0] + 21.0)
                    || (self.ball_speed.0 + 5.0 < 0.0 && self.ball_pos.0 >= rect[0] + 21.0)
                {
                    self.ball_speed.0 *= -1.01;
                }
                self.ball_speed.1 *= -1.01;
                to_die = indx;
                break;
            }
        }
        if !(to_die == 69420 as usize) {
            self.rect_values.0.remove(to_die);
            self.rect_values.1.remove(to_die);
            assert_eq!(self.rect_values.0.len(), self.rect_values.1.len());
        }
    }
}
// rectangle = [x, y, w, h]

fn get_rect() -> (Vec<[f64; 4]>, Vec<[f32; 4]>) {
    let mut positions: Vec<[f64; 4]> = Vec::new();
    let mut colours: Vec<[f32; 4]> = Vec::new();
    let possible_colours = [
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [160.0 / 255.0, 32.0 / 255.0, 240.0 / 255.0, 1.0],
    ];
    let mut colour_index = 0;
    let mut y = 0.0;
    for _ in 0..6 {
        let mut x = 0.0;
        for _ in 0..15 {
            if colour_index > 3 {
                colour_index = 0;
            }
            positions.push([x, y, 42.0, 15.0]);
            colours.push(possible_colours[colour_index]);
            colour_index += 1;
            x += 42.0;
        }
        y += 15.0;
    }

    (positions, colours)
}
