#![feature(iter_array_chunks)]
#![feature(unbounded_shifts)]

use std::time::Instant;

use clap::{Parser, Subcommand};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    day: Option<Day>,
}

#[derive(Subcommand)]
enum Day {
    All,
    Last,
    Day { day: u8 },
}

fn main() {
    let cli = Cli::parse();

    let problems = [
        day_1::compute,
        day_2::compute,
        day_3::compute,
        day_4::compute,
        day_5::compute,
        day_6::compute,
        day_7::compute,
        day_8::compute,
        day_9::compute,
        day_10::compute,
        day_11::compute,
        day_12::compute,
        day_13::compute,
        day_14::compute,
        day_15::compute,
        day_16::compute,
        day_17::compute,
        day_18::compute,
        day_19::compute,
    ];

    let day_arg = match cli.day.unwrap_or(Day::Last) {
        Day::All => None,
        Day::Last => Some(problems.len() as u8),
        Day::Day { day } => Some(day),
    };

    problems
        .iter()
        .enumerate()
        .filter(|(day, _)| match day_arg {
            Some(day_num) => day_num as usize == *day + 1,
            _ => true,
        })
        .for_each(|(day, problem)| {
            println!("==== DAY {} ====", day + 1);
            println!();
            let start_time = Instant::now();
            problem();
            println!();
            println!("duration: {:?}", start_time.elapsed().as_secs_f64());
            println!();
        });
}
