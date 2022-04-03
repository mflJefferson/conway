mod life;

extern crate pancurses;

use std::borrow::Borrow;
use pancurses::{ALL_MOUSE_EVENTS, endwin, getmouse, initscr, mousemask, Input, Window, nl, noecho, curs_set};
use crate::life::Life;

const MAX_SIZE: usize = 300;

fn main() {
    let window = initscr();

    window.keypad(true); // Set keypad mode
    mousemask(ALL_MOUSE_EVENTS, Some(&mut 0)); // Listen to all mouse events

    nl();
    noecho();
    curs_set(0);
    window.timeout(0);
    window.keypad(true);

    let mut data_even = [[0u32;300]; 300];

    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    let (x, y) = (mouse_event.x, mouse_event.y);
                    window.mvaddch(y, x, '@');
                    data_even[y as usize][x as usize] = 1;
                };
            }
            Some(Input::Character(x)) if x == 'q' => break,
            Some(Input::Character(x)) if x == 'p' => start_game(window.borrow(), &mut data_even),
            Some(Input::Character(x)) if x == '1' => pentadecathlon(window.borrow()),
            Some(Input::Character(x)) if x == '2' => pulsar(window.borrow()),
            Some(Input::Character(x)) if x == 'c' => {
                window.erase();
            },
            _ => (),
        }
    }

    endwin();
}

fn pentadecathlon(window: &Window) {
    let mut data_even = [[0u32;300]; 300];

    data_even[26][113] = 1;
    data_even[26][118] = 1;

    data_even[27][111] = 1;
    data_even[27][112] = 1;
    data_even[27][114] = 1;
    data_even[27][115] = 1;
    data_even[27][116] = 1;
    data_even[27][117] = 1;
    data_even[27][119] = 1;
    data_even[27][120] = 1;

    data_even[28][113] = 1;
    data_even[28][118] = 1;


    start_game(&window, &mut data_even);
}

fn start_game(window: &Window, data_even: &mut [[u32; 300]; 300]) {
    let mut life = Life {
        generation: 0
    };

    let mut data_odd = [[0u32;300]; 300];

    loop {
        if life.is_generation_even() {
            for (i, x) in data_even.iter().enumerate() {
                for (j, y) in x.iter().enumerate() {
                    if *y == 1 {
                        window.mvaddch(i as i32, j as i32, '@');
                    }
                    arranje_new_generation(&mut data_even.clone(), &mut data_odd, i, j, y)
                }
            }
        } else {
            for (i, x) in data_odd.iter().enumerate() {
                for (j, y) in x.iter().enumerate() {
                    if *y == 1 {
                        window.mvaddch(i as i32, j as i32, '@');
                    }
                    arranje_new_generation(&mut data_odd.clone(),  data_even, i, j, y)
                }
            }
        }
        window.mvprintw(0, 0, format!("generation: {}", life.generation));
        match window.getch() {
            Some(Input::Character(x)) if x == 'q' => break,
            Some(Input::Character(x)) if x == 'c' => {
                window.erase();
            },
            _ => (),
        }
        life.increment();
        // napms(100);
        window.erase();
    }
}

fn arranje_new_generation(current_data: &mut [[u32; 300]; 300], next_data: &mut [[u32; 300]; 300], i: usize, j: usize, y: &u32) {
    let mut neighbors = 0;
    let mut next_i;
    let mut next_j;

    if i > 0 && j > 0 {
        next_i = i - 1;
        next_j = j - 1;
        if next_i < MAX_SIZE && next_j < MAX_SIZE {
            if current_data[next_i][next_j] == 1 { neighbors += 1 };
        }

        next_j = j;
        if next_i < MAX_SIZE && next_j < MAX_SIZE {
            if current_data[next_i][next_j] == 1 { neighbors += 1 };
        }

        next_j = j + 1;
        if next_i < MAX_SIZE && next_j < MAX_SIZE {
            if current_data[next_i][next_j] == 1 { neighbors += 1 };
        }
    }

    if j > 0 {
        next_i = i;
        next_j = j - 1;
        if next_i > 0 && next_i < MAX_SIZE && next_j > 0 && next_j < MAX_SIZE {
            if current_data[next_i][next_j] == 1 { neighbors += 1 };
        }

        next_j = j + 1;
        if next_i > 0 && next_i < MAX_SIZE && next_j > 0 && next_j < MAX_SIZE {
            if current_data[next_i][next_j] == 1 { neighbors += 1 };
        }
        next_i = i + 1;
        next_j = j - 1;
        if next_i > 0 && next_i < MAX_SIZE && next_j > 0 && next_j < MAX_SIZE {
            if current_data[next_i][next_j] == 1 { neighbors += 1 };
        }

        next_j = j;
        if next_i > 0 && next_i < MAX_SIZE && next_j > 0 && next_j < MAX_SIZE {
            if current_data[next_i][next_j] == 1 { neighbors += 1 };
        }

        next_j = j + 1;
        if next_i > 0 && next_i < MAX_SIZE && next_j > 0 && next_j < MAX_SIZE {
            if current_data[next_i][next_j] == 1 { neighbors += 1 };
        }
    }

    if (*y == 1 && (neighbors == 2 || neighbors == 3)) || (*y == 0 && neighbors == 3) {
        next_data[i][j] = 1;
    } else {
        next_data[i][j] = 0;
    }
}


fn pulsar(window: &Window) {
    let mut data_even = [[0u32;300]; 300];

    data_even[25][113] = 1;
    data_even[25][114] = 1;
    data_even[25][115] = 1;
    data_even[25][119] = 1;
    data_even[25][120] = 1;
    data_even[25][121] = 1;

    data_even[27][111] = 1;
    data_even[27][116] = 1;
    data_even[27][118] = 1;
    data_even[27][123] = 1;

    data_even[28][111] = 1;
    data_even[28][116] = 1;
    data_even[28][118] = 1;
    data_even[28][123] = 1;

    data_even[29][111] = 1;
    data_even[29][116] = 1;
    data_even[29][118] = 1;
    data_even[29][123] = 1;

    data_even[30][113] = 1;
    data_even[30][114] = 1;
    data_even[30][115] = 1;
    data_even[30][119] = 1;
    data_even[30][120] = 1;
    data_even[30][121] = 1;

    data_even[32][113] = 1;
    data_even[32][114] = 1;
    data_even[32][115] = 1;
    data_even[32][119] = 1;
    data_even[32][120] = 1;
    data_even[32][121] = 1;

    data_even[33][111] = 1;
    data_even[33][116] = 1;
    data_even[33][118] = 1;
    data_even[33][123] = 1;

    data_even[34][111] = 1;
    data_even[34][116] = 1;
    data_even[34][118] = 1;
    data_even[34][123] = 1;

    data_even[35][111] = 1;
    data_even[35][116] = 1;
    data_even[35][118] = 1;
    data_even[35][123] = 1;

    data_even[37][113] = 1;
    data_even[37][114] = 1;
    data_even[37][115] = 1;
    data_even[37][119] = 1;
    data_even[37][120] = 1;
    data_even[37][121] = 1;


    start_game(&window, &mut data_even);
}