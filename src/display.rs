use std::io::{self, Write};
use std::time::Instant;

fn time_string(mut seconds: u64) -> String {
    let hours: u16 = (seconds / 3600) as u16;
    seconds %= 3600;
    let minutes: u8 = (seconds / 60) as u8;
    seconds %= 60;
    format!("{:4}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn display(
    temperature: f64,
    energy: f64,
    accept_percent: f64,
    improve_percent: f64,
    instant: &Instant,
    step_ratio: f64,
) {
    let elapsed: u64 = instant.elapsed().as_secs();
    let remaining: u64 = (elapsed as f64 / step_ratio) as u64;
    let elapsed_string: String = time_string(elapsed);
    let remaining_string: String = time_string(remaining);
    print!(
        "\r{:12.2}  {:12.2}  {:7.2}%  {:7.2}%  {}  {}",
        temperature, energy, accept_percent, improve_percent, elapsed_string, remaining_string
    );
    io::stdout().flush().unwrap();
}

pub fn init_display(temperature: f64, energy: f64, instant: &Instant) {
    println!(" Temperature        Energy    Accept   Improve     Elapsed   Remaining");
    display(temperature, energy, 0.0, 0.0, instant, 0_f64);
}
