# setup-console
A library that provides a function to fix ANSI escaping problem in Command Prompt on Windows.

[![Latest version](https://img.shields.io/crates/v/setup-console.svg)](https://crates.io/crates/setup-console)

## Usage
Simply run `setup_console::init()` (or `setup_console::try_init()`) in the main function before printing anything.

```rust
fn main() {
    println!("\x1b[31mRed \x1b[32mGreen \x1b[34mBlue\x1b[39m");
    // ･[31mRed ･[32mGreen ･[34mBlue･[39m
    setup_console::init();
    println!("\x1b[31mRed \x1b[32mGreen \x1b[34mBlue\x1b[39m");
    // Red Green Blue
}
```

## Panics
**This only applies to Windows.**
`setup_console::init()` doesn't panic, but shows message and terminates program with error code obtained from
[`GetLastError()`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
instead.
The situations that would lead program to terminate would be
- when stdout is invalid (e.g. `program 1>nul`)
- when program fails to run [`GetConsoleMode()`](https://learn.microsoft.com/en-us/windows/console/getconsolemode)
- when program fails to run [`SetConsoleMode()`](https://learn.microsoft.com/en-us/windows/console/setconsolemode)
If this is not your desired behavior, you can use `setup_console::try_init()` which returns `Result`.
