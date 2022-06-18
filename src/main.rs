use ncurses::*;
use chrono::Utc;
use std::{thread, time::Duration, process};
use ctrlc;

fn main() {
    static WIN_HEIGHT: i32 = 0;
    static WIN_WIDTH: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0; 

    //setup ctrl-c handler
    //calling endwin() here ensures that terminal still has a cursor after exit
    ctrlc::set_handler(|| {endwin(); process::exit(0)} ).expect("couldn't set ctrl-c handler");

    //start ncurses
    initscr();
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);


    //get screen bounds
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let row = (max_y - WIN_HEIGHT) /2;
    let col = (max_x - WIN_WIDTH) /2;


    loop {
        clear();
        let date_time = Utc::now();
        //formatting https://docs.rs/chrono/latest/chrono/format/strftime/index.html#fn1
        let formatted_time = date_time.format("%y:%m:%d:%T:%f").to_string();
        let ft_len = formatted_time.len();
        //get formatted date-time string with the unneccesary digits of precision thrown off
        let (out_string, _extra) = formatted_time.split_at(ft_len - 7);

        //move to middle of ncurses window within screen while adjusting for date-time string size
        mvprintw(row, col - (ft_len as i32 / 2), out_string);
        refresh();
        thread::sleep(Duration::from_millis(25));
    }

}
