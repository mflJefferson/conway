extern crate pancurses;

use pancurses::{initscr, endwin, curs_set};

fn main() {
    let window = initscr();
    window.mvprintw(40,40,"Hello Rust");
    let (y, x) = window.get_max_yx();
    window.mvprintw(40,80,format!("y = {}, x = {}", y, x));
    window.mvprintw(44,228,"a");
    window.mvprintw(y / 2,x / 2,"000");
    curs_set(0);

    // window.erase();

    let mut counter = 0;
    loop {
        window.mvprintw((y / 2) + counter,(x / 2) + counter,"000");
        counter += 1;
        // window.erase();
        // napms(47);
        if counter == 10 { break }
    }
    window.mvprintw(y / 2,x / 2,"000");
    window.refresh();
    window.getch();
    endwin();
}


// fn array_test() {
//     let array = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
//     for i in array {
//         for j in i {
//             println!("{:?}", j);
//         }
//     }
// }