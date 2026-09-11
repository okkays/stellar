use rand::random_bool;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Painter, Shape};

#[derive(Debug, Clone, PartialEq)]
pub struct Stars {
    pub color: Color,
    /// The max X and Y values for the stars.
    pub range: (i32, i32),
    /// A value between 0 and 1, indicating the probability any given point should be a star.
    pub saturation: f64,
    pub stars: Vec<(i32, i32)>,
}

impl Stars {
    pub fn tick(&mut self) {
        self.stars = self
            .stars
            .iter()
            .filter(|star| star.0 > 0)
            .map(|star| (star.0 - 1, star.1))
            .chain(
                (0..self.range.1)
                    .filter(|_| random_bool(self.saturation))
                    .map(|y| (self.range.0, y)),
            )
            .collect()
    }
}

impl Shape for Stars {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.stars.iter() {
            if let Some((x, y)) = painter.get_point(*x as f64, *y as f64) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use crate::stars::Stars;

    #[test]
    fn tick() {
        let mut stars = Stars {
            color: Color::Reset,
            range: (4, 3),
            saturation: 1.0,
            #[rustfmt::skip]
            stars: vec![
                (0, 0),                 (3, 0), (4, 0),
                                (2, 1),
                (0, 2), (1, 2), (2, 2),
            ],
        };

        stars.tick();

        #[rustfmt::skip]
        assert_eq!(stars.stars, vec![
                                (2, 0), (3, 0),
                        (1, 1),
                (0, 2), (1, 2),
                                                (4, 0),
                                                (4, 1),
                                                (4, 2),
        ]);
    }
}
