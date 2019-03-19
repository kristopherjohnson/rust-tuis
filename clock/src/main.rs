extern crate chrono;
extern crate cursive;

use chrono::{Local, Utc};

use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

fn main() {
    let mut app = Cursive::default();

    // Hit 'q' to quit
    app.add_global_callback('q', |app| app.quit());

    let utc_id = "utc";
    let local_id = "local";
    let time_format = "%Y-%m-%d %H:%M:%S%.3f";

    // Create text views for the clock elements
    let utc_text = TextView::new("").with_id(utc_id);
    let local_text = TextView::new("").with_id(local_id);

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

        app.call_on_id(utc_id, |v: &mut TextView| {
            let content = format!("UTC:   {}", utc.format(time_format));
            v.set_content(content);
        });
        app.call_on_id(local_id, |v: &mut TextView| {
            let content = format!("Local: {}", local.format(time_format));
            v.set_content(content);
        });

        app.step();
    }
}
