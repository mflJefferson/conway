mod life;

extern crate pancurses;

use std::borrow::Borrow;
use pancurses::{ALL_MOUSE_EVENTS, endwin, getmouse, initscr, mousemask, Input, Window, napms, nl, noecho, curs_set};
use crate::life::Life;

fn main() {
    let window = initscr();

    window.keypad(true); // Set keypad mode
    mousemask(ALL_MOUSE_EVENTS, Some(&mut 0)); // Listen to all mouse events

    nl();
    noecho();
    curs_set(0);
    window.timeout(0);
    window.keypad(true);

    let mut data_even = [[0i32;300]; 300];

    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    let (x, y) = (mouse_event.x, mouse_event.y);
                    window.mvaddch(y, x, 'o');
                    data_even[y as usize][x as usize] = 1;
                };
            }
            Some(Input::Character(x)) if x == 'q' => break,
            Some(Input::Character(x)) if x == 'p' => start_game(window.borrow(), &data_even),
            Some(Input::Character(x)) if x == 'c' => {
                window.erase();
            },
            _ => (),
        }
    }

    endwin();
}

fn start_game(window: &Window, data_even: &[[i32; 300]; 300]) {
    let (y, x) = window.get_max_yx();
    let mut life = Life {
        cols: x,
        rows: y,
        generation: 0
    };

    let mut data_odd = [[0i32;300]; 300];

    loop {
        if life.is_generation_even() {
            for (i, x) in data_even.iter().enumerate() {
                for (j, y) in x.iter().enumerate() {
                    if *y == 1 {
                        window.mvaddch(i as i32, j as i32, 'o');
                    }
                }
            }
        } else {
            for (i, x) in data_odd.iter().enumerate() {
                for (j, y) in x.iter().enumerate() {
                    if *y == 1 {
                        window.mvaddch(i as i32, j as i32, 'o');
                    }
                }
            }
        }
        window.printw(format!("generation: {}", life.generation));
        match window.getch() {
            Some(Input::Character(x)) if x == 'q' => break,
            Some(Input::Character(x)) if x == 'c' => {
                window.erase();
            },
            _ => (),
        }
        life.increment();
        napms(200);
        window.erase();
    }
}