#[derive(Debug, Clone)]
pub enum SetupConsoleErrorType {
    InvalidOutputHandle,
    GetConsoleModeFailed,
    SetConsoleModeFailed,
}

#[derive(Debug, Clone)]
pub struct SetupConsoleError(SetupConsoleErrorType);

impl std::fmt::Display for SetupConsoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                SetupConsoleErrorType::InvalidOutputHandle => "stdout is invalid",
                SetupConsoleErrorType::GetConsoleModeFailed => "failed to get console mode",
                SetupConsoleErrorType::SetConsoleModeFailed => "failed to set console mode",
            }
        )
    }
}

#[cfg(target_os = "windows")]
pub fn try_init() -> Result<(), SetupConsoleError> {
    use windows_sys::Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        System::Console::{
            GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            STD_OUTPUT_HANDLE,
        },
    };
    unsafe {
        let std_output = GetStdHandle(STD_OUTPUT_HANDLE);
        if std_output == INVALID_HANDLE_VALUE {
            return Err(SetupConsoleError(
                SetupConsoleErrorType::InvalidOutputHandle,
            ));
        }
        let mut console_mode = 0;
        if GetConsoleMode(std_output, &mut console_mode) == 0 {
            return Err(SetupConsoleError(
                SetupConsoleErrorType::GetConsoleModeFailed,
            ));
        }
        console_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        if SetConsoleMode(std_output, console_mode) == 0 {
            return Err(SetupConsoleError(
                SetupConsoleErrorType::SetConsoleModeFailed,
            ));
        }
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn try_init() -> Result<(), SetupConsoleError> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn init() {
    use windows_sys::Win32::{Foundation::GetLastError, System::Threading::ExitProcess};
    if let Err(error) = try_init() {
        eprintln!("Error: {}", error);
        unsafe {
            ExitProcess(GetLastError());
        };
    }
}

#[cfg(not(target_os = "windows"))]
pub fn init() {}
