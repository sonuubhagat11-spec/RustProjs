use std::io;
use colored::Colorize;
use rand::RngExt;
use std::{thread, time::Duration};

fn main() {
    println!("loading the simulation 10%  [|*********]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 20%  [||********]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 30%  [|||*******]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 40%  [||||******]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 50%  [|||||*****]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 60%  [||||||****]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 70%  [|||||||***]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 80%  [||||||||**]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 90%  [|||||||||*]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 100% [||||||||||]");
    thread::sleep(Duration::from_secs(1));
    println!("loading the simulation 200- jk jk");
    thread::sleep(Duration::from_secs(1));
    println!("sudo takes a finder from toml the cargo lock ): )-:");
    let x = rand::rng().random_range(1..=10);
    if x == 5
    {
        thread::sleep(Duration::from_secs(1));
        println!("you win )-:");
    }

    if x != 5 {
        loop {
            {
                println!(
                    "ok pass this test what plus/minus x is 5? x is {} also dont say alphabets or non integers or (number).(number) ):< ",
                    x.to_string().blue().bold()
                );

                let mut abx = String::new();

                io::stdin()
                    .read_line(&mut abx)
                    .unwrap();

                let ring: i32 =
                    abx.trim().parse().unwrap();

                if x + ring == 5 {
                    thread::sleep(Duration::from_secs(1));
                    println!("{}", "good boy".green().bold());
                    break;
                }

                if x - ring == 5 {
                    thread::sleep(Duration::from_secs(1));
                    println!("{}", "good boy".green().bold());
                    break;
                }
                else {
                    thread::sleep(Duration::from_secs(1));println!("{}", "bad boy".red().bold()); }
            }

        }
    thread::sleep(Duration::from_secs(5));}
}