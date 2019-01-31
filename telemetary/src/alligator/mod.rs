use chrono::prelude::Utc;

mod constants;
mod utils;
pub(crate) mod server;
pub(crate) mod swarm;

fn log(text: &str) {
    println!("{} >> {}", Utc::now(), text);
}

// pub(crate) mod macros {

//     #[macro_export]
//     macro_rules! log {
//         ($s:expr) => {
//             log($s);
//         };
//     }
// }
