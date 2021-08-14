//! an example showing how to generate images for embedding in Rust docs.

use docima::{generate_image, DocimaResult, StdResult};
use plotters::prelude::*;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

fn main() -> DocimaResult<()> {
    // an example using the `plotters` crate
    generate_image(
        plot_histogram,
        600,
        400,
        "images/plotters-histogram.html",
        "This is an histogram (alt)",
        "This is an histogram (title)",
        "div",
    )?;

    // an example using a custom closure
    generate_image(
        |buffer, _x, _y| {
            // using a seed so that the rendered image doesn't change
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(1234);
            for chunk in buffer.chunks_mut(3) {
                chunk[0] = rng.gen();
                chunk[1] = rng.gen();
                chunk[2] = rng.gen();
            }
            Ok(())
        },
        32,
        32,
        "images/square-random-pixels.html",
        "This is an square with random pixels (alt)",
        "a square with random color pixels",
        "span",
    )?;

    Ok(())
}

/// source: <https://github.com/38/plotters/blob/master/examples/histogram.rs>
pub fn plot_histogram(buffer: &mut Vec<u8>, max_x: u32, max_y: u32) -> StdResult<()> {
    let root = BitMapBackend::with_buffer(buffer, (max_x, max_y)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
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
}
