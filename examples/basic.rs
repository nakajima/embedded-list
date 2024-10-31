use embedded_graphics::{
    mono_font::{iso_8859_7::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_list::{List, ListItem, ListSource};

#[derive(Clone)]
struct BasicItem {
    text: String,
}

impl ListItem for BasicItem {
    type C = Rgb565;

    fn height(&self) -> i32 {
        32
    }

    fn draw<D: DrawTarget<Color = Self::C>>(
        &self,
        display: &mut D,
        origin: Point,
        is_active: bool,
    ) -> Result<Point, D::Error> {
        if is_active {
            let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
            let text = Text::new(&self.text, origin, text_style);
            text.draw(display)
        } else {
            let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_GRAY);
            let text = Text::new(&self.text, origin, text_style);
            text.draw(display)
        }
    }
}

struct Source {
    items: Vec<BasicItem>,
}

impl ListSource for Source {
    type ListSourceItem = BasicItem;

    fn items(&self) -> impl Iterator<Item = Self::ListSourceItem> {
        self.items.iter().cloned()
    }

    fn item(&self, index: usize) -> Option<Self::ListSourceItem> {
        if index < self.items.len() {
            Some(self.items[index].clone())
        } else {
            None
        }
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let source = Source {
        items: vec![
            BasicItem {
                text: "First item".to_string(),
            },
            BasicItem {
                text: "Second item".to_string(),
            },
            BasicItem {
                text: "Third item".to_string(),
            },
            BasicItem {
                text: "Fourth item".to_string(),
            },
            BasicItem {
                text: "Fifth item".to_string(),
            },
            BasicItem {
                text: "Sixth item".to_string(),
            },
            BasicItem {
                text: "Seventh item".to_string(),
            },
            BasicItem {
                text: "Eighth item".to_string(),
            },
            BasicItem {
                text: "Ninth item".to_string(),
            },
            BasicItem {
                text: "Tenth item".to_string(),
            },
            BasicItem {
                text: "Eleventh item".to_string(),
            },
            BasicItem {
                text: "Twelfth item".to_string(),
            },
        ],
    };

    let top_simulator_padding = 16;
    let mut list = List::new(
        Rectangle::new(Point::new(0, top_simulator_padding), Size::new(240, 532)),
        source,
    );

    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(
        240,
        (532 + top_simulator_padding).try_into().unwrap(),
    ));

    let output_settings = OutputSettingsBuilder::new()
        .scale(1)
        .pixel_spacing(0)
        .theme(BinaryColorTheme::Default)
        .build();

    let mut window = Window::new("Basic", &output_settings);

    'running: loop {
        list.draw(&mut display).unwrap();
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Up => list.set_current(list.current() - 1),
                        Keycode::Down => list.set_current(list.current() + 1),
                        _ => (),
                    };
                }
                _ => {}
            }
        }
    }

    Ok(())
}
