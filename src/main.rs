extern crate pancurses;

use std::borrow::Borrow;
use pancurses::{ALL_MOUSE_EVENTS, endwin, getmouse, initscr, mousemask, Input, Window, napms};

fn main() {
    let window = initscr();

    window.keypad(true); // Set keypad mode
    mousemask(ALL_MOUSE_EVENTS, Some(&mut 0)); // Listen to all mouse events

    window.printw("press q to exit\npress p to play\npress c to clear");
    window.refresh();

    clean_game(window);

    endwin();
}

fn clean_game(window: Window) {
    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    let (x, y) = (mouse_event.x, mouse_event.y);
                    window.get_max_yx();
                    window.get_beg_x();
                    window.mvprintw(y, x,"0");
                };
            }
            Some(Input::Character(x)) if x == 'q' => break,
            Some(Input::Character(x)) if x == 'p' => start_game(window.borrow()),
            Some(Input::Character(x)) if x == 'c' => {
                window.erase();
            },
            _ => (),
        }
    }
}

fn start_game(window: &Window) {
    loop {
        match window.getch() {
            Some(Input::Character(x)) if x == 'q' => break,
            Some(Input::Character(x)) if x == 'c' => {
                window.erase();
            },
            _ => (),
        }
    }
}