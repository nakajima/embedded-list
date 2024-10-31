use core::marker::PhantomData;

use embedded_graphics::{
    prelude::{DrawTarget, PixelColor, Point},
    primitives::Rectangle,
    Drawable,
};

use crate::{list_item::ListItem, ListSource};

pub enum ListError {
    OutOfBounds,
}

pub struct List<C: PixelColor, I: ListItem<C = C>, S: ListSource<ListSourceItem = I>> {
    rect: Rectangle,
    current: usize,
    source: S,
    _phantom: PhantomData<(C, I)>,
}

impl<C: PixelColor, I: ListItem<C = C>, S: ListSource<ListSourceItem = I>> List<C, I, S> {
    pub fn new(rect: Rectangle, source: S) -> Self {
        Self {
            rect,
            current: 0,
            source,
            _phantom: PhantomData,
        }
    }
}

impl<C: PixelColor, I: ListItem<C = C>, S: ListSource<ListSourceItem = I>> Drawable
    for List<C, I, S>
{
    type Color = C;
    type Output = ();

    fn draw<D: DrawTarget<Color = C>>(&self, display: &mut D) -> Result<(), D::Error> {
        let mut y_offset = self.rect.top_left.y;

        for (i, item) in self.source.items().enumerate() {
            item.draw::<D>(display, Point::new(0, y_offset), self.current == i)?;
            y_offset += item.height();
        }

        Ok(())
    }
}

impl<C: PixelColor, I: ListItem<C = C>, S: ListSource<ListSourceItem = I>> List<C, I, S> {
    pub fn set_current(&mut self, index: usize) {
        if index >= self.source.items().count() {
            self.current = 0;
        } else {
            self.current = index;
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }
}
