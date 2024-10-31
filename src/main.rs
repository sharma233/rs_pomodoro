use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::io::{Write, stdout};
use getopts::Options;
use std::env;

const POMO_TIME:u64 = 1500;
const BREAK_TIME:u64 = 300;

fn print_usage(opts: Options) {
    let brief = format!("Usage: pomo [options]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("b", "break", "start a break timer");
    opts.optflag("h", "help", "print help");


    let now = SystemTime::now();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m},
        Err(e) => { panic!("{}", e.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    let target_secs = if matches.opt_present("b") {
        BREAK_TIME
    } else {
        POMO_TIME
    };

    let mut stdout = stdout();
    let mut secs_elapsed = now.elapsed().unwrap().as_secs();

    //print mm:ss timer
    print!("Target Time: {:02}m and {:02}s\n", target_secs / 60, target_secs % 60);
    while secs_elapsed < target_secs {
        secs_elapsed = now.elapsed().unwrap().as_secs();
        print!("\rTime Elapsed: {:02}m and {:02}s", secs_elapsed / 60, secs_elapsed % 60);
        stdout.flush().unwrap();
        sleep(Duration::new(1, 0));
    }

    //ring terminal bell
    println!("\x07");
}
