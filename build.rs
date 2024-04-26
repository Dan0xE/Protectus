pub fn main() {
    tauri_plugin::Builder::new(&[
        "get_hwid_command",
        "set_serial_number_command",
        "get_serial_number_state_command",
        "get_serial_number_data_command",
        "activate_license_command",
        "deactivate_license_command",
        "is_protected_command",
        "is_virtual_machine_command",
        "is_debugger_present_command",
        "feature_check_command"
    ]).build();
}
