extern crate chrono;
extern crate cursive;

use chrono::{Local, Utc};

use cursive::theme::BaseColor::*;
use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::{BorderStyle, Palette, Theme};
use cursive::views::{LinearLayout, TextContent, TextView};
use cursive::Cursive;

use std::thread;
use std::time::Duration;

fn main() {
    let mut app = Cursive::default();

    app.set_theme(theme());

    // Hit 'q' to quit
    app.add_global_callback('q', |app| app.quit());

    // Create text views for the clock elements
    let mut utc_content = TextContent::new("UTC:   ");
    let mut loc_content = TextContent::new("Local: ");
    let utc_text = TextView::new_with_content(utc_content.clone());
    let loc_text = TextView::new_with_content(loc_content.clone());

    // Add text views to a layout
    let mut layout = LinearLayout::vertical();
    layout.add_child(utc_text);
    layout.add_child(loc_text);

    app.add_layer(layout);

    // Start a thread to update the display periodically
    thread::spawn(move || {
        let time_format = "%Y-%m-%d %H:%M:%S%.3f %z";

        loop {
            let utc = Utc::now();
            let loc = Local::now();

            let utc_string = utc.format(time_format);
            let loc_string = loc.format(time_format);

            utc_content.set_content(format!("UTC:   {}", utc_string));
            loc_content.set_content(format!("Local: {}", loc_string));

            thread::sleep(Duration::from_millis(100));
        }
    });

    // Run the Cursive event loop
    app.set_autorefresh(true);
    app.run();
}

fn theme() -> Theme {
    Theme {
        shadow: false,
        borders: BorderStyle::None,
        palette: palette(),
    }
}

fn palette() -> Palette {
    let mut pal = Palette::default();

    pal[Background] = Dark(Black);
    pal[View] = Dark(Black);
    pal[Primary] = Light(Yellow);

    pal
}
