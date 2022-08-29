use chrono::Utc;
use std::{thread, time::Duration, process};
use std::io::{stdout, Write};
use ctrlc;
use crossterm::{
    ExecutableCommand, terminal::{size, SetSize}, execute, queue,
    terminal, cursor, style::{Print}
};


fn main() {
    //setup ctrl-c handler
    //calling endwin() here ensures that terminal still has a cursor after exit
    ctrlc::set_handler(|| {
        terminal::Clear(terminal::ClearType::All); 
        process::exit(0)
    }).expect("couldn't set ctrl-c handler");

    //start crossterm
    let mut stdout = stdout();
    //prepare terminal
    stdout.execute(terminal::Clear(terminal::ClearType::All)).expect("Could not clear terminal");
    stdout.execute(cursor::Hide).unwrap();

    stdout.flush().expect("Unable to flush to terminal");

    let mut cur_size: (u16, u16) = size().unwrap();
    loop {

        //handle terminal resizing
        let new_size = size().unwrap();
        if !(new_size == cur_size) {
            let (height, width) = new_size;

            cur_size = new_size;
            SetSize(height, width);
            stdout.execute(terminal::Clear(terminal::ClearType::All)).expect("Could not clear terminal");
        }


        //get the date string we will be using to display to the center of the screen
        let date_time = Utc::now();
        //formatting https://docs.rs/chrono/latest/chrono/format/strftime/index.html#fn1
        let formatted_time = date_time.format("%y:%m:%d:%T:%f").to_string();
        let ft_len = formatted_time.len();
        //get formatted date-time string with the unneccesary digits of precision thrown off
        let (out_string, _extra) = formatted_time.split_at(ft_len - 7);

        //have to make sure to adjust for the output string's length here so it is actually cent.
        let (height, width) = cur_size;
        let mid_x = width / 2;
        let mid_y = height / 2;
        queue!(stdout, cursor::MoveTo(mid_y - ((out_string.len() as u16) / 2), mid_x))
            .expect("Could not move terminal cursor");
        execute!(stdout, Print(out_string)).expect("Could not place text");

        //refresh output here
        stdout.flush().expect("Unable to flush to terminal");

        //delay so that clock doesn't look like it's going too fast
        thread::sleep(Duration::from_millis(35));
    }

}
