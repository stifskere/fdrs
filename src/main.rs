use std::{env::args, path::Path};
use colored::Colorize;
use globset::Glob;
use utils::{find::find_recursive, result::DirectoryEntry};


mod utils;

fn main() {
    let args = args()
        .collect::<Vec<_>>();

    let help = args
        .iter()
        .any(|a| a == "--help" || a == "-h");

    if help {
        println!(
            r#"
-- Find ~ Help --
options:
    --ignore-errors | -i => ignores all the errors showing only the found values
    --help | -h => shows this menu
usage:
    The program requires two arguments at least, <path> <glob>, the
    path argument is where to find from, the glob argument is like
    a filter, where everything not matched by the glob will be excluded.

    Even tho there is no point, you can simply pass `*` to include
    everything.

~ thanks for using this crate.
-----------------
            "#
        )
    }

    let ignore_errors = args
        .iter()
        .any(|a| a == "--ignore-errors" || a == "-i");

    let matcher = match Glob::new(&args[2]) {
        Ok(glob) => glob.compile_matcher(),
        Err(err) => {
            println!("{}", err.to_string().red());
            return;
        }
    };

    if args.len() < 3 {
        println!(
            "{}",
            r#"
This program requires two arguments.

<path> <glob>

use `--help` or `-h` to display the help menu.
            "#
                .red()
        )
    }

    match find_recursive(&Path::new(&args[1]), &Some(matcher), ignore_errors) {
        Ok(found) => found
            .iter()
            .for_each(|f| println!("{}", match f {
                DirectoryEntry::Ok(e) => e
                    .to_string_lossy()
                    .to_string()
                    .cyan(),
                DirectoryEntry::Error(err) => err
                    .to_string()
                    .red()
            })),
        Err(err) => println!(
            "{:#}",
            err
                .to_string()
                .red()
        )
    }
}
