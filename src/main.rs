use plotters::prelude::{WHITE, ChartBuilder, IntoDrawingArea, IntoFont, Histogram, RED, Color, draw_piston_window};
use piston_window::{EventLoop, PistonWindow, WindowSettings};

const FPS: u32 = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window: PistonWindow = WindowSettings::new("Peetch", [450, 300])
        .samples(4)
        .build()
        .unwrap();
    window.set_max_fps(FPS as u64);

    while let Some(_) = draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Histogram Test", ("sans-serif", 50.0).into_font())
            .build_ranged(0u32..10u32, 0u32..10u32)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .line_style_1(&WHITE.mix(0.3))
            .x_label_offset(30)
            .y_desc("Count")
            .x_desc("Bucket")
            .axis_desc_style(("sans-serif", 15).into_font())
            .draw()?;

        let data = [
            0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
        ];

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5).filled())
                .data(data.iter().map(|x: &u32| (*x, 1))),
        )?;
        Ok(())
    }) {}

    Ok(())
}