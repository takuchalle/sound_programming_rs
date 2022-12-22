use helpers::{SAMPLING_RATE, create_sine_wave};
use plotters::prelude::*;

fn write_bitmap(data: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("wave.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("wave", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..SAMPLING_RATE / 2, -1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(i, x)| (i, *x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wave = create_sine_wave(500.0, 0.5, 1);
    write_bitmap(&wave)?;

    Ok(())
}
