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
use embedded_list::{List, ListSource};

#[derive(Clone)]
struct BasicItem {
    text: String,
}

struct Source {
    items: Vec<BasicItem>,
}

impl ListSource for Source {
    type C = Rgb565;

    fn total_count(&self) -> u8 {
        self.items.len() as u8
    }

    fn height_for_index(&self, _index: u8) -> i32 {
        32
    }

    fn draw<D: DrawTarget<Color = Self::C>>(
        &self,
        index: u8,
        is_active: bool,
        display: &mut D,
        origin: Point,
    ) -> Result<Point, D::Error> {
        let item = &self.items[index as usize];

        if is_active {
            let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
            let text = Text::new(&item.text, origin, text_style);
            text.draw(display)
        } else {
            let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_GRAY);
            let text = Text::new(&item.text, origin, text_style);
            text.draw(display)
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
            BasicItem {
                text: "Thirteenth item".to_string(),
            },
            BasicItem {
                text: "Fourteenth item".to_string(),
            },
            BasicItem {
                text: "Fifteenth item".to_string(),
            },
            BasicItem {
                text: "Sixteenth item".to_string(),
            },
            BasicItem {
                text: "Seventeenth item".to_string(),
            },
            BasicItem {
                text: "Eighteenth item".to_string(),
            },
            BasicItem {
                text: "Nineteenth item".to_string(),
            },
            BasicItem {
                text: "Twentieth item".to_string(),
            },
            BasicItem {
                text: "Twenty-first item".to_string(),
            },
        ],
    };

    let items_count = source.total_count();

    let top_simulator_padding = 16;
    let mut list = List::<Rgb565, Source>::new(
        Rectangle::new(Point::new(0, top_simulator_padding), Size::new(240, 532)),
        &source,
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
        display.clear(Rgb565::BLACK).unwrap();
        list.draw(&mut display).unwrap();
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Up => {
                            let next = if list.current() == 0 {
                                items_count - 1
                            } else {
                                list.current() - 1
                            };
                            list.set_current(next);
                        }
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
