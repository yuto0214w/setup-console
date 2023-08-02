#[cfg(target_os = "windows")]
pub fn init() {
    use windows_sys::Win32::{
        Foundation::{GetLastError, INVALID_HANDLE_VALUE},
        System::{
            Console::{
                GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
                STD_OUTPUT_HANDLE,
            },
            Threading::ExitProcess,
        },
    };
    unsafe {
        let exit_with_error = |message: &str| {
            eprintln!("Error: {}", message);
            ExitProcess(GetLastError());
        };
        let std_output = GetStdHandle(STD_OUTPUT_HANDLE);
        if std_output == INVALID_HANDLE_VALUE {
            exit_with_error("stdout is invalid");
        }
        let mut console_mode = 0;
        if GetConsoleMode(std_output, &mut console_mode) == 0 {
            exit_with_error("failed to get console mode");
        }
        console_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        if SetConsoleMode(std_output, console_mode) == 0 {
            exit_with_error("failed to set console mode");
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn init() {}
