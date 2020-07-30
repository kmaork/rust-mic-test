use std::sync::mpsc::sync_channel;
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::{BLACK, ChartBuilder, IntoDrawingArea, IntoFont, draw_piston_window, LineSeries, WHITE, GREEN};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

mod rolling;

const FPS: u32 = 30;

fn draw(mut window: &mut PistonWindow, points: &[f32], caption: &str) -> bool {
    draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&BLACK)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 30.0).into_font().color(&WHITE))
            .build_ranged(0..((points.len() + 2) as i32), -1f32..1f32)?;

        let data = (0..points.len()).map(|i| (i as i32, points[i]));
        chart.draw_series(LineSeries::new(data, &GREEN))?;
        Ok(())
    }).is_some()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device available");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let (sender, receiver) = sync_channel(100);
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| { sender.try_send(data.to_vec()).ok(); },
        err_fn,
    )?;

    let mut window: PistonWindow = WindowSettings::new("rust-mic-test", [600, 400])
        .samples(4)
        .build()
        .expect("Couldn't build window");

    stream.play()?;
    window.set_max_fps(FPS as u64);
    window.set_lazy(false);
    window.set_bench_mode(true);
    let mut roller = rolling::SmoothChunkRoller::new_default(1500);
    while draw(&mut window, roller.data(), &device.name()?) {
        for chunk in receiver.try_iter() {
            roller.enqueue(&chunk);
        }
        roller.roll();
    }
    drop(stream);
    Ok(())
}

// TODO: make faster. Sample rate? ASIO? FPS?
// TODO: what are channels?
