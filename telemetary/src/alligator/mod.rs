use chrono::prelude::Utc;

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
