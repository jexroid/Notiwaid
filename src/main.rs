use chrono;
use colored::*;
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time;

fn main() {
    let mut sp = Spinner::new(Spinners::Aesthetic, "".into());
    sleep(time::Duration::from_secs(1));
    sp.stop();
    println!("");

    println!(
        "{} {}{}{}{}{}  {}{}  {}\n{}{}",
        "Welcome Lord",
        "JEXROID\n\n".magenta().bold(),
        "┌\n".cyan(),
        "│\n".cyan(),
        "│\n".cyan(),
        "◇".bright_yellow(), "Today\n".bright_black(),
        "│".cyan(),
        chrono::offset::Utc::now().format("❯ %Y-%m-%d %a").to_string(),
        "│\n".cyan(),
        "└\n".cyan()
    );
    
    // thread::spawn(|| {
    //     TODO: add a notification in this thread to check the github status or anything on web you need
    // });
}
