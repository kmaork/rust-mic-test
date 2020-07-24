// https://github.com/RustAudio/cpal/blob/master/examples/record_wav.rs
// https://docs.rs/plotters/0.2.15/plotters/
use std::sync::mpsc::sync_channel;
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::{BLACK, ChartBuilder, IntoDrawingArea, IntoFont, Color, draw_piston_window, LineSeries, WHITE, GREEN};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

mod rolling;

const FPS: u32 = 30;

fn draw(mut window: &mut PistonWindow, points: &[i16]) -> bool {
    draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&BLACK)?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .right_y_label_area_size(40)
            .margin(5)
            .caption("Microphone", ("sans-serif", 30.0).into_font().color(&WHITE))
            // .axis_desc_style(("sans-serif", 2.0).into_font().color(&WHITE))
            .build_ranged(0..(points.len() as i32), (i16::MIN as i32)..(i16::MAX as i32))?;

        chart
            .configure_mesh()
            .axis_style(&WHITE.mix(0.4))
            .axis_desc_style(("sans-serif", 2.0).into_font().color(&WHITE))
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let data = (0..points.len()).map(|i| (i as i32, points[i] as i32));
        chart.draw_series(LineSeries::new(data, &GREEN))?;
        Ok(())
    }).is_some()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device available");
    println!("Microphone: {}", device.name()?);
    // println!("Supported: {:?}", device.supported_input_configs().unwrap().collect::<Vec<_>>());
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let (sender, receiver) = sync_channel(10);
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[i16], _: &_| { sender.try_send(data.to_vec()).ok(); },
        err_fn,
    )?;

    let mut window: PistonWindow = WindowSettings::new("Peetch", [450, 300])
        .samples(4)
        .build()
        .expect("Couldn't build window");

    stream.play()?;
    window.set_max_fps(FPS as u64);
    let mut roller = rolling::RollingData::new_default(5000);
    while draw(&mut window, roller.data()) {
        for chunk in receiver.try_iter() {
            roller.roll(&chunk);
        }
    }
    drop(stream);
    Ok(())
}

/*
We don't necessarily know the input buffer size, but we don't need to.
Let's decide on a size that is nice for displaying, and roll new data into it.
The window always displays the current buffer.
 */
// TODO: why is text not displayed?
// TODO: make faster. Sample rate? ASIO? FPS?
// TODO: if sample_format F32 is supported, how are we using i16?
// TODO: what are channels?
