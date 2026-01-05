pub mod helper;
#[cfg(feature = "licensing")]
pub mod licensing;
#[cfg(feature = "service")]
pub mod service;

use helper::feature_check_command;
use tauri::Runtime;
use tauri::plugin::{Builder, TauriPlugin};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut builder = Builder::new("protectus");

    #[cfg(feature = "licensing")]
    {
        use crate::licensing::*;
        builder = builder.invoke_handler(tauri::generate_handler![
            feature_check_command,
            get_hwid_command,
            set_serial_number_command,
            get_serial_number_state_command,
            get_serial_number_data_command,
            activate_license_command,
            deactivate_license_command,
        ]);
    }

    #[cfg(feature = "service")]
    {
        use crate::service::*;
        builder = builder.invoke_handler(tauri::generate_handler![
            feature_check_command,
            is_virtual_machine_command,
            is_debugger_present_command,
            is_protected_command
        ]);
    }

    builder.build()
}
