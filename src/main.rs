extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::*;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

struct Rectangle {
    x: f64,
    y: f64,
    radius: f64,
    speed: f64,
}

impl Rectangle {
    pub fn new(num: Option<f64>) -> Rectangle {
        let radius = (random::<f64>() * (WIDTH / 10.0)) + 3.0;
        let mut rectangle: Rectangle = Rectangle {
            speed: (random::<f64>() * 70.0) + 10.0,
            y: random::<f64>() * (HEIGHT + radius),
            x: random::<f64>() * WIDTH,
            radius: radius,
        };

        if let Some(y) = num {
            rectangle.speed = 0.0;
            rectangle.y = y;
        }
        return rectangle;
    }
}

fn get_rectangles() -> Vec<Rectangle> {
    let mut rectangles = Vec::new();
    let rects_quantity = (random::<u64>() % 15) + 10;
    for _ in 0..rects_quantity {
        rectangles.push(Rectangle::new(None));
    }
    rectangles
}

fn main() {
    let rectangle_color = [255.0, 0.0, 0.0, 1.0];
    let background = [1.0, 1.0, 1.0, 1.0];
    let mut rectangles: Vec<Rectangle> = get_rectangles();
    let mut window: PistonWindow = WindowSettings::new("Rectangles Animation", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut events = window.events;

    while let Some(event) = events.next(&mut window) {
        if let Some(_) = event.render_args() {
            let rects = &rectangles;
            window.draw_2d(&event, |content, graphics, _| {
                clear(background, graphics);
                for r in rects {
                    rectangle(
                        rectangle_color,
                        [
                            r.x - r.radius,
                            r.y - r.radius,
                            r.radius * 2.0,
                            r.radius * 2.0,
                        ],
                        content.transform,
                        graphics,
                    );
                }
            });
            ()
        }

        if let Some(update) = event.update_args() {
            let rects = &mut rectangles;
            for r in rects {
                r.y -= r.speed * update.dt; //dt is the delta timing
                if r.y + r.radius <= 0.0 {
                    r.y = HEIGHT + r.radius
                }
            }
        }
    }
}
