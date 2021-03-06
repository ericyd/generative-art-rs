// cargo run --release --example hex_1 -- --stroke 2 --padding 15 --radius 20 --layers 20
extern crate chrono;
extern crate nannou;

use nannou::color::*;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

mod util;
use util::args::ArgParser;
use util::captured_frame_path;
use util::color::*;
use util::hexagon::*;
use util::interp::{lerp, nextf};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

fn main() {
  nannou::app(model).run();
}

struct Model {
  stroke_weight: f32,
  radius: f32,
  layers: i32,
  noise_scale: f64,
  padding: f32,
}

fn model(app: &App) -> Model {
  let args = ArgParser::new();
  let stroke_weight = args.get_f32("stroke", 20.);
  let padding = args.get_f32("padding", stroke_weight * 2.);
  let radius = args.get_f32("radius", 40.);
  let layers = args.get_i32("layers", 10);
  let noise_scale = args.get_f64("noise", 300.);
  app
    .new_window()
    .size(WIDTH, HEIGHT)
    .title(app.exe_name().unwrap())
    .view(view)
    .build()
    .unwrap();
  Model {
    stroke_weight,
    radius,
    layers,
    noise_scale,
    padding,
  }
}

fn view(app: &App, model: &Model, frame: Frame) {
  // two frames are necessary for capture_frame to work properly
  app.set_loop_mode(LoopMode::loop_ntimes(1));

  let win = app.window_rect();
  let perlin = Perlin::new();

  let palette = get_palette("red orange blue");

  let draw = app.draw();
  draw.background().color(hsl(47. / 360., 1., 0.94));

  // flow lines
  let ny = 300;
  let seed = 24.;
  for j in 0..ny {
    let y_factor = j as f32 / ny as f32;

    let mut x = lerp(win.x.start, win.x.start + ny as f32, y_factor) + nextf(-4., 4.);
    let mut y = win.y.start + nextf(-4., 4.);

    let hue = lerp(273. / 360., 170. / 360., y_factor);
    let color = hsla(hue, 0.2, 0.38, 0.3);
    let line_weight = 1.0;

    let mut points = vec![(pt2(x, y), color)];
    (0..1000).for_each(|_| {
      let theta = PI / 3.;

      // noisy weighted gradient pointing away from center
      let noise_factor = 1.0;
      let theta_factor = 1.0;
      let noise = perlin.get([
        x as f64 / model.noise_scale,
        y as f64 / model.noise_scale,
        seed as f64,
      ]) as f32;
      let angle = noise * noise_factor + theta * theta_factor;

      x = angle.cos() * 2.0 + x;
      y = angle.sin() * 2.0 + y;

      points.push((pt2(x, y), color));
    });

    draw
      .polyline()
      .start_cap_round()
      .weight(line_weight)
      .points_colored(points);
  }

  // hexagon overlay
  honeycomb_hex(model.radius, model.layers, model.padding, false)
    .iter()
    .for_each(|hex| {
      // let bg: (RgbHue, f32, f32) = Hsl::from(random_palette_1()).into_components();
      // let lighter = Hsla::new(bg.0, bg.1, bg.2 * 1.15, 1.0);
      draw
        .polygon()
        .stroke(random_color(palette))
        .stroke_weight(model.stroke_weight)
        .points(hex.points())
        .hsla(0., 0., 0., 0.);
    });

  // Write to the window frame.
  draw.to_frame(app, &frame).unwrap();

  app
    .main_window()
    .capture_frame_threaded(captured_frame_path(app, &frame));
}
