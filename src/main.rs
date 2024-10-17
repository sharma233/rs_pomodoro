use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::io::{Write, stdout};

fn main() {
    let now = SystemTime::now();
    let target_secs = 1;

    let mut stdout = stdout();
    let mut secs_elapsed = now.elapsed().unwrap().as_secs();

    //print mm:ss timer
    while secs_elapsed < target_secs {
        secs_elapsed = now.elapsed().unwrap().as_secs();
        let formatted_time = get_formatted_time_string_from_seconds(secs_elapsed);
        print!("\r{}", formatted_time);
        stdout.flush().unwrap();
        sleep(Duration::new(1, 0));
    }

    //ring terminal bell
    println!("\x07");
}

fn get_formatted_time_string_from_seconds(secs: u64) -> String {
    let minutes = secs / 60;
    let seconds = secs % 60;
    return format!("Time elapsed: {:02}m and {:02}s", minutes, seconds);
}
