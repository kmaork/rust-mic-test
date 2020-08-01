#![feature(associated_type_bounds)]

use plotters::prelude::*;
// use cpal::Host;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::error::Error;
use std::sync::mpsc::sync_channel;
use cpal::DeviceNameError;

mod rolling;

pub fn draw_mic_input_on_plotters_backend<DB: DrawingBackend>(backend: DB, points: &[f32], caption: &str) -> Result<(), Box<dyn Error>>
    where <DB as plotters::drawing::DrawingBackend>::ErrorType: 'static {
    let root = backend.into_drawing_area();
    root.fill(&BLACK)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 30.0).into_font().color(&WHITE))
        .build_ranged(0..((points.len() + 2) as i32), -1f32..1f32)?;

    let data = (0..points.len()).map(|i| (i as i32, points[i]));
    chart.draw_series(LineSeries::new(data, &GREEN))?;
    Ok(())
}

pub struct MicTest {
    device: cpal::Device,
}

impl MicTest {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_input_device().expect("no input device available"); //: Host::Device
        Self { device }
    }

    pub fn device_name(&self) -> Result<String, DeviceNameError> {
        self.device.name()
    }

    pub fn start<F: FnMut(&[f32]) -> bool>(&self, mut data_callback: F) -> Result<(), Box<dyn Error>> {
        let config = self.device
            .default_input_config()
            .expect("Failed to get default input config");
        println!("Default input config: {:?}", config);

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let (sender, receiver) = sync_channel(100);
        let stream = self.device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| { sender.try_send(data.to_vec()).ok(); },
            err_fn,
        )?;
        stream.play()?;

        let mut roller = rolling::SmoothChunkRoller::new_default(1500);
        while data_callback(roller.data()) {
            for chunk in receiver.try_iter() {
                roller.enqueue(&chunk);
            }
            roller.roll();
        }
        drop(stream);
        Ok(())
    }
}

// TODO: make faster. Sample rate? ASIO? FPS?
// TODO: what are channels?
// TODO: start fast and becomes slower, why?
