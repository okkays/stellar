use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{canvas::Canvas, Block, BorderType, Widget},
};

use crate::{app::App, assets::PointImage};

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("stellar")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        PointImage::default();

        let canvas = Canvas::default()
            .block(block)
            .x_bounds([0.0, 200.0])
            .y_bounds([0.0, 200.0])
            .paint(|ctx| {
                ctx.draw(&PointImage {
                    filename: String::from("src/images/ship.txt"),
                    color: ratatui::style::Color::Green,
                    scale: (1, 3),
                    offset: self.ship_position,
                });
            });

        canvas.render(area, buf);
    }
}
