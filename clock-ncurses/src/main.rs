// Simple clock that displays UTC and local time, using the pancurses library
// <https://github.com/ihalila/pancurses>

extern crate chrono;
extern crate ncurses;

use chrono::{Local, Utc};

use ncurses::CURSOR_VISIBILITY::*;
use ncurses::{
    attron, clear, curs_set, endwin, getbegyx, getch, has_colors, init_pair,
    initscr, mvaddstr, nodelay, refresh, start_color, A_BOLD, COLOR_BLACK,
    COLOR_PAIR, COLOR_YELLOW, ERR,
};

use std::thread;
use std::time::Duration;

fn main() {
    let window = initscr();
    nodelay(window, true);

    curs_set(CURSOR_INVISIBLE);

    if has_colors() {
        // Display bold yellow text
        start_color();
        init_pair(1, COLOR_YELLOW, COLOR_BLACK);
        attron(COLOR_PAIR(1) | A_BOLD());
    }

    let time_format = "%Y-%m-%d %H:%M:%S%.3f %z";

    loop {
        // Press any key to exit
        if getch() != ERR {
            break;
        }

        clear();

        let utc = Utc::now().format(time_format);
        let local = Local::now().format(time_format);

        let mut y: i32 = 0;
        let mut x: i32 = 9;
        getbegyx(window, &mut y, &mut x);

        mvaddstr(y + 1, x + 1, format!("UTC:   {}", utc).as_ref());
        mvaddstr(y + 2, x + 1, format!("Local: {}", local).as_ref());

        refresh();

        thread::sleep(Duration::from_millis(50));
    }

    endwin();
}
