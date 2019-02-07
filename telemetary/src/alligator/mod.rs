use chrono::prelude::Utc;
use colored::Colorize;

mod constants;
pub(crate) mod server;
pub(crate) mod swarm;
mod utils;

fn log(text: &str) {
    println!(
        "{} @ {} : {}",
        "New Connection".green().bold(),
        format!("{}", Utc::now().time().format("%-I:%M %p")).yellow(),
        text.red()
    );
}

// pub(crate) mod macros {

//     #[macro_export]
//     macro_rules! log {
//         ($s:expr) => {
//             log($s);
//         };
//     }
// }
