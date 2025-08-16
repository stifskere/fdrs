
# FDRS

> [!warning]
> This project is unmaintained as of the last commit it has, it is not promised that the project will continue being in crates.io, but that won't change anytime soon.
>
> This was my first rust project uploaded to crates.io, and it will remain in here for history.

A super simple implementation of the unix command `fd`.

The following is as stated contextually in the command help:

options:
    --ignore-errors | -i => ignores all the errors showing only the found values
    --help | -h => shows this menu
usage:
    The program requires two arguments at least, <path> <glob>, the
    path argument is where to find from, the glob argument is like
    a filter, where everything not matched by the glob will be excluded.

    Even tho there is no point, you can simply pass `*` to include
    everything.

## Installation

You can use cargo to install this crate, as a library or as a CLI tool.

 - `cargo install fdrs` to install this as a CLI tool.
 - `cargo add fdrs` to install this as a library and use the exported functions.

## Contributing

I currently don't mantain this project, as I don't have time to do so, and it's super
small, there is no license, you can copy or do whatever you feel like with this.

If you for any reason want this project, simply fork it or do whatever you please
with it.
