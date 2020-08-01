#![feature(associated_type_bounds)]

use piston_window::{EventLoop, PistonWindow, WindowSettings};
use rust_mic_test::{draw_mic_input_on_plotters_backend, MicTest};
use std::error::Error;
use plotters::prelude::draw_piston_window;


const FPS: u32 = 30;


fn draw_window(mut window: &mut PistonWindow, points: &[f32], caption: &str) -> bool {
    draw_piston_window(&mut window, |backend| draw_mic_input_on_plotters_backend(backend, points, caption)).is_some()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut window: PistonWindow = WindowSettings::new("rust-mic-test", [600, 400])
        .samples(4)
        .build()
        .expect("Couldn't build window");

    window.set_max_fps(FPS as u64);
    window.set_lazy(false);
    window.set_bench_mode(true);
    let mic_test = MicTest::new();
    let device_name = mic_test.device_name()?;
    mic_test.start(|data| draw_window(&mut window, data, &device_name))
}
