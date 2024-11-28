
//! Simple Find implementation.
//!
//! By using the globset create matches all the files
//! in a specified diretory and returns a Vec with results
//! that can be serialized to a String as these implement
//! Display.

mod utils;

pub use utils::find::{find, find_glob};
pub use utils::result::*;
