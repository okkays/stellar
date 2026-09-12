use ratatui::{
    buffer::Buffer,
    layout::{Rect},
    widgets::{canvas::Canvas, Block, BorderType, Widget},
    style::Color,
};

use crate::{app::App, point_image::PointImage};

pub const RESOLUTION: (i32, i32) = (200, 200);

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .border_type(BorderType::Rounded);

        PointImage::default();

        let canvas = Canvas::default()
            .background_color(Color::Black)
            .block(block)
            .x_bounds([0.0, RESOLUTION.0 as f64])
            .y_bounds([0.0, RESOLUTION.1 as f64])
            .paint(|ctx| {
                ctx.draw(&self.stars_fg);
                ctx.draw(&self.stars_bg);
                ctx.draw(&PointImage::default()
                    .filename("src/images/ship_tardis.txt")
                    .color(Color::LightCyan)
                    .scale((1, 2))
                    .offset(self.ship_position));
            });

        canvas.render(area, buf);
    }
}
