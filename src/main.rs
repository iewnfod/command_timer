use std::{env, io, process::Command, time::Instant};

fn command_failed(c: &Command, e: &io::Error) {
    println!(
        "Failed to run command `{}` with error message: \n\t{}",
        c.get_program().to_str().unwrap(),
        e.to_string()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Add some commands to run and test their time costs");
        println!("\nExample: \n$ {} echo 1", &args[0]);
        let mut try_command = Command::new(&args[0]);
        try_command.args(vec!["echo", "1"]);
        try_command.spawn().unwrap().wait().unwrap();
        return;
    }
    let mut command = Command::new(&args[1]);
    if args.len() >= 2 {
        command.args(&args[2..]);
    }

    let start = Instant::now();

    match command.spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(_) => (),
                Err(e) => command_failed(&command, &e)
            }
        },
        Err(e) => command_failed(&command, &e)
    }

    let time_cost = start.elapsed();
    println!(
        "\nTime Cost: \n\t{:?} s\n\t≈ {:?} ms \n\t≈ {:?} us \n\t≈ {:?} ns",
        time_cost.as_secs_f64(),
        time_cost.as_millis(),
        time_cost.as_micros(),
        time_cost.as_nanos(),
    );
}
