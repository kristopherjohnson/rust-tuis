// Simple clock that displays UTC and local time, using the pancurses library
// <https://github.com/ihalila/pancurses>

extern crate chrono;
extern crate pancurses;

use chrono::{Local, Utc};

use pancurses::{
    curs_set, endwin, has_colors, init_pair, initscr, start_color, A_BOLD,
    COLOR_BLACK, COLOR_PAIR, COLOR_YELLOW,
};

use std::thread;
use std::time::Duration;

fn main() {
    let window = initscr();
    window.nodelay(true);

    curs_set(0);

    if has_colors() {
        // Display bold yellow text
        start_color();
        init_pair(1, COLOR_YELLOW, COLOR_BLACK);
        window.attron(COLOR_PAIR(1) | A_BOLD);
    }

    let time_format = "%Y-%m-%d %H:%M:%S%.3f %z";
    loop {
        // Press any key to exit
        if let Some(_) = window.getch() {
            break;
        }

        let utc = Utc::now();
        let loc = Local::now();

        let utc_string = utc.format(time_format);
        let loc_string = loc.format(time_format);

        window.clear();

        let (y, x) = window.get_beg_yx();

        window.mv(y + 1, x + 1);
        window.printw(format!("UTC:   {}", utc_string));

        window.mv(y + 2, x + 1);
        window.printw(format!("Local: {}", loc_string));

        window.refresh();

        thread::sleep(Duration::from_millis(50));
    }

    endwin();
}
