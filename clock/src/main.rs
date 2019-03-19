extern crate chrono;
extern crate cursive;

use chrono::{Local, Utc};

use cursive::views::{Dialog, LinearLayout, TextContent, TextView};
use cursive::Cursive;

fn main() {
    let mut app = Cursive::default();

    // Hit 'q' to quit
    app.add_global_callback('q', |app| app.quit());

    let time_format = "%Y-%m-%d %H:%M:%S%.3f";

    // Create text views for the clock elements
    let mut utc_content = TextContent::new("");
    let utc_text = TextView::new_with_content(utc_content.clone());
    let mut local_content = TextContent::new("");
    let local_text = TextView::new_with_content(local_content.clone());

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

        utc_content.set_content(format!("UTC:   {}", utc.format(time_format)));
        local_content.set_content(format!("Local: {}", local.format(time_format)));

        app.step();
    }
}
