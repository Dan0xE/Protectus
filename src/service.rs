use tauri::command;
use vmprotect::service::{is_debugger_present, is_protected, is_virtual_machine};

#[command]
pub fn is_protected_command() -> bool {
    is_protected()
}

#[command]
pub fn is_virtual_machine_command() -> bool {
    is_virtual_machine()
}

#[command]
pub fn is_debugger_present_command(check_kernel_mode: bool) -> bool {
    is_debugger_present(check_kernel_mode)
}
