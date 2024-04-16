pub mod helper;
pub mod licensing;
pub mod service;

use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

use crate::licensing::*;
use crate::service::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("protectus")
        .invoke_handler(tauri::generate_handler![
            get_hwid_command,
            set_serial_number_command,
            get_serial_number_state_command,
            get_serial_number_data_command,
            activate_license_command,
            deactivate_license_command,
            is_protected_command,
            is_virtual_machine_command,
            is_debugger_present_command
        ])
        .build()
}
