use embedded_graphics::prelude::{DrawTarget, PixelColor, Point};

pub trait ListItem {
    type C: PixelColor;

    fn draw<D: DrawTarget<Color = Self::C>>(
        &self,
        display: &mut D,
        origin: Point,
        is_active: bool,
    ) -> Result<Point, D::Error>;

    fn height(&self) -> i32;
}
