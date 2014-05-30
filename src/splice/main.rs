#![crate_id = "splice#0.1-pre"]
#![license = "MIT"]
#![crate_type = "bin"]

#![feature(globs)]

extern crate splice;
extern crate ncurses;
extern crate libc;

use ncurses::*;
use splice::conf::Config;
use splice::connect;

fn main() {
    let Config {
        default_server_addr: addr,
        default_server_port: port,
    } = match splice::conf::load_default() {
        Some(cfg) => cfg,
        None => fail!("Config not loaded"),
    };
    initscr();
    cbreak();
    keypad(stdscr, true);
    noecho();
    printw("Splice editor: Just watch for now =)\n");
    match connect(addr.as_slice(), port) {
        Ok(..) => {
            printw("Connection to server succeded!");
        },
        Err(..) => {
            fail!("Connection to server failed!");
        },
    };
    'm: loop {
        refresh();
        match getch() {
            lit => { // Character literals
                if lit == 'q' as i32 { break 'm }
            },
        };
    }
    endwin();
}
