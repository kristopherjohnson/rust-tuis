// Simple clock that displays UTC and local time, using the pancurses library
// <https://github.com/ihalila/pancurses>

extern crate chrono;
extern crate pancurses;

use chrono::{Local, Utc};

use pancurses::{curs_set, endwin, initscr};

use std::thread;
use std::time::Duration;

fn main() {
    let window = initscr();
    window.nodelay(true);

    curs_set(0);

    let time_format = "%Y-%m-%d %H:%M:%S%.3f %z";
    loop {
        let utc = Utc::now();
        let loc = Local::now();

        let utc_string = utc.format(time_format);
        let loc_string = loc.format(time_format);

        window.clear();

        let (y, x) = window.get_beg_yx();

        window.mv(y + 1, x + 1);
        window.printw(format!("UTC:   {}", utc_string));

        window.mv(y + 3, x + 1);
        window.printw(format!("Local: {}", loc_string));

        window.refresh();

        // Press any key to exit
        match window.getch() {
            Some(_) => break,
            None => thread::sleep(Duration::from_millis(50)),
        }
    }

    endwin();
}
