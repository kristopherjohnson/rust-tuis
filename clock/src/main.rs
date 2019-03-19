extern crate chrono;
extern crate cursive;

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Local, TimeZone, Utc};

use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

use std::fmt::Display;

fn main() {
    let mut app = Cursive::default();

    // Hit 'q' to quit
    app.add_global_callback('q', |app| app.quit());

    // Create text views for the clock elements
    let utc_text = TextView::new("").with_id("utc");
    let local_text = TextView::new("").with_id("local");

    // Add text views to a layout
    let mut layout = LinearLayout::vertical();
    layout.add_child(utc_text);
    layout.add_child(local_text);

    // Put the layout in a centered dialog
    app.add_layer(Dialog::around(layout).title("Clock"));

    // Refresh the screen on every iteration of the loop
    app.set_autorefresh(true);

    // Update the screen as quickly as Cursive::step() will run
    while app.is_running() {
        let utc = Utc::now();
        let local = Local::now();

        app.call_on_id("utc", |v: &mut TextView| {
            let content = format!("UTC:   {}", format_for_clock(utc));
            v.set_content(content);
        });
        app.call_on_id("local", |v: &mut TextView| {
            let content = format!("Local: {}", format_for_clock(local));
            v.set_content(content);
        });

        app.step();
    }
}

/// Format a DateTime<Tz> value for clock display
fn format_for_clock<'a, Tz: TimeZone>(dt: DateTime<Tz>) -> DelayedFormat<StrftimeItems<'a>>
where
    Tz::Offset: Display,
{
    dt.format("%Y-%m-%d %H:%M:%S%.3f")
}
