#![feature(associated_type_bounds)]

use plotters::prelude::*;
use std::error::Error;

pub fn draw<DB: DrawingBackend>(backend: DB, points: &[f32], caption: &str) -> Result<(), Box<dyn Error>>
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