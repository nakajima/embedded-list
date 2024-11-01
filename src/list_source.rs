use embedded_graphics::prelude::{DrawTarget, PixelColor, Point};

pub trait ListSource {
    type C: PixelColor;

    fn total_count(&self) -> u8;
    fn height_for_index(&self, index: u8) -> i32;

    fn draw<D: DrawTarget<Color = Self::C>>(
        &self,
        index: u8,
        is_active: bool,
        display: &mut D,
        origin: Point,
    ) -> Result<Point, D::Error>;
}
