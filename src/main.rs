use std::{env, process::Command, time::Instant};

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

    command.spawn().unwrap().wait().unwrap();

    let time_cost = start.elapsed();
    println!(
        "\nTime Cost: \n\t{:?} s\n\t≈ {:?} ms \n\t≈ {:?} us \n\t≈ {:?} ns",
        time_cost.as_secs_f64(),
        time_cost.as_millis(),
        time_cost.as_micros(),
        time_cost.as_nanos(),
    );
}
