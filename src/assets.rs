use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Shape};
use std::fs;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PointImage {
    pub filename: String,
    pub color: Color,
    /// When reading one point in this dimension, read scale.i points instead
    pub scale: (i32, i32),
    /// This value is added to each coordinate
    pub offset: (f64, f64),
}

// impl Default for PointImage {
//     fn default() -> Self {
//         return Self {
//             filename: String::from("error.txt"),
//             scale: (1, 1),
//             color: Color::Reset,
//         };
//     }
// }

impl PointImage {
    const ERROR_IMAGE: &str = ".  .\n . \n.  .";

    /// Reads an image from a file
    /// Periods ('.') in the file will be read as points.
    ///
    /// The results will be an array of the locations of the "solid" points,
    /// (0, 0) represents the first col, row in the image
    fn to_coords(&self) -> Vec<(f64, f64)> {
        let contents = fs::read_to_string(&self.filename);
        let unwrapped = contents.unwrap_or(Self::ERROR_IMAGE.to_string());

        let mut coords: Vec<(i32, i32)> = Vec::new();
        let mut row: Vec<(i32, i32)> = Vec::new();
        let mut address = (0, 0);
        for char in unwrapped.chars() {
            if char == '\n' {
                address = (0, address.1 + self.scale.1);
                coords.extend((1..self.scale.1).flat_map(|i| {
                    return row.iter().map(move |c| (c.0, c.1 + i));
                }));
                row.clear();
                continue;
            }
            if char == '.' {
                let new_coords: Vec<(i32, i32)> = (0..self.scale.0)
                    .map(|i| (address.0 + i, address.1))
                    .collect();
                row.extend(new_coords.clone());
                coords.extend(new_coords.clone());
            }
            address = (address.0 + self.scale.0, address.1);
        }

        // Because coordinate space goes up and columns goes down, the image is read in upside down
        // and needs to be flipped
        let max_y = address.1 - 1;
        let flipped = coords.iter().map(|c| (c.0, max_y - c.1));

        return flipped
            .map(|c| ((c.0 as f64) + self.offset.0, (c.1 as f64) + self.offset.1))
            .collect();
    }
}

impl Shape for PointImage {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.to_coords() {
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::widgets::canvas::Canvas;
    use ratatui::widgets::Widget;
    use ratatui::{buffer::Buffer, layout::Rect, symbols::Marker};

    use super::*;

    #[test]
    fn reads_images() {
        let hi = &PointImage {
            filename: String::from("src/images/hi.txt"),
            color: Color::Green,
            scale: (1, 1),
            offset: (0.0, 0.0),
        };

        #[rustfmt::skip]
        assert_eq!(hi.to_coords(), vec![
            (0.0, 2.0),             (2.0, 2.0),             (4.0, 2.0), (5.0, 2.0), (6.0, 2.0),
            (0.0, 1.0), (1.0, 1.0), (2.0, 1.0),                         (5.0, 1.0),
            (0.0, 0.0),             (2.0, 0.0),             (4.0, 0.0), (5.0, 0.0), (6.0, 0.0),
        ]);
    }

    #[test]
    fn draw() {
        let hi = &PointImage {
            filename: String::from("src/images/hi.txt"),
            color: Color::Reset,
            scale: (1, 1),
            offset: (0.0, 0.0),
        };

        let rect = Rect::new(0, 0, 7, 3);
        let mut buffer = Buffer::empty(rect);
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 6.0])
            .y_bounds([0.0, 2.0])
            .paint(|context| {
                context.draw(hi);
            });
        canvas.render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "• • •••",
            "•••  • ",
            "• • •••",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draw_scaled_x() {
        let hi = &PointImage {
            filename: String::from("src/images/hi.txt"),
            color: Color::Reset,
            scale: (3, 1),
            offset: (0.0, 0.0),
        };

        let rect = Rect::new(0, 0, 21, 3);
        let mut buffer = Buffer::empty(rect);
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 20.0])
            .y_bounds([0.0, 2.0])
            .paint(|context| {
                context.draw(hi);
            });
        canvas.render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "•••   •••   •••••••••",
            "•••••••••      •••   ",
            "•••   •••   •••••••••",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draw_scaled_y() {
        let hi = &PointImage {
            filename: String::from("src/images/hi.txt"),
            color: Color::Reset,
            scale: (1, 3),
            offset: (0.0, 0.0),
        };

        let rect = Rect::new(0, 0, 7, 9);
        let mut buffer = Buffer::empty(rect);
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 6.0])
            .y_bounds([0.0, 8.0])
            .paint(|context| {
                context.draw(hi);
            });
        canvas.render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "• • •••",
            "• • •••",
            "• • •••",
            "•••  • ",
            "•••  • ",
            "•••  • ",
            "• • •••",
            "• • •••",
            "• • •••",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draw_offset() {
        let hi = &PointImage {
            filename: String::from("src/images/hi.txt"),
            color: Color::Reset,
            scale: (1, 1),
            offset: (5.0, 3.0),
        };

        let rect = Rect::new(0, 0, 20, 7);
        let mut buffer = Buffer::empty(rect);
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 19.0])
            .y_bounds([0.0, 6.0])
            .paint(|context| {
                context.draw(hi);
            });
        canvas.render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "                    ",
            "     • • •••        ",
            "     •••  •         ",
            "     • • •••        ",
            "                    ",
            "                    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draw_scaled() {
        let hi = &PointImage {
            filename: String::from("src/images/hi.txt"),
            color: Color::Reset,
            scale: (2, 2),
            offset: (0.0, 0.0),
        };

        let rect = Rect::new(0, 0, 14, 6);
        let mut buffer = Buffer::empty(rect);
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 13.0])
            .y_bounds([0.0, 5.0])
            .paint(|context| {
                context.draw(hi);
            });
        canvas.render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "••  ••  ••••••",
            "••  ••  ••••••",
            "••••••    ••  ",
            "••••••    ••  ",
            "••  ••  ••••••",
            "••  ••  ••••••",
        ]);
        assert_eq!(buffer, expected);
    }
}
