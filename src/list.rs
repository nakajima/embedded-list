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
    scroll_offset: i32,
    source: S,
    _phantom: PhantomData<(C, I)>,
}

impl<C: PixelColor, I: ListItem<C = C>, S: ListSource<ListSourceItem = I>> List<C, I, S> {
    pub fn new(rect: Rectangle, source: S) -> Self {
        Self {
            rect,
            current: 0,
            scroll_offset: 0,
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
        let mut y_offset = self.rect.top_left.y - self.scroll_offset;
        let bottom_edge = self.rect.bottom_right().unwrap_or(self.rect.top_left).y;

        for (i, item) in self.source.items().enumerate() {
            let item_height = item.height();
            let item_top = y_offset;
            let item_bottom = y_offset + item_height;

            // Check if the item is within the visible area
            if item_bottom > self.rect.top_left.y && item_top < bottom_edge {
                item.draw::<D>(
                    display,
                    Point::new(self.rect.top_left.x, y_offset),
                    self.current == i,
                )?;
            }

            y_offset += item_height;
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

        // Update scroll_offset to keep current item visible
        self.update_scroll_offset();
    }

    fn update_scroll_offset(&mut self) {
        let mut total_height = 0;
        let mut current_item_top = 0;
        let mut current_item_bottom = 0;

        for (i, item) in self.source.items().enumerate() {
            let item_height = item.height();

            if i == self.current {
                current_item_top = total_height;
                current_item_bottom = total_height + item_height;
                break;
            }

            total_height += item_height;
        }

        let viewport_height = self.rect.size.height as i32;
        let viewport_top = self.scroll_offset;
        let viewport_bottom = self.scroll_offset + viewport_height;

        // Adjust scroll to bring the current item into view
        if current_item_top < viewport_top {
            self.scroll_offset = current_item_top;
        } else if current_item_bottom > viewport_bottom {
            self.scroll_offset = current_item_bottom - viewport_height;
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }
}
