use tauri::command;
use vmprotect::licensing::{
    activate_license, deactivate_license, get_hwid, get_serial_number_data,
    get_serial_number_state, set_serial_number,
};

use crate::helper::*;

#[command]
pub fn get_hwid_command() -> Result<String, ()> {
    Ok(get_hwid())
}


#[command]
pub fn set_serial_number_command(serial_str: String) -> Result<String, String> {
    let serial_bytes = serial_str.into_bytes();
    
    match set_serial_number(serial_bytes) {
        Ok(serial_state) => {
            let states = serialize_serial_state(serial_state);
            Ok(format!("Serial state: {}", states.join(", ")))
        }
        Err(e) => {
            Err(format!("Invalid input: NUL character found at position {}", e))
        }
    }
}

#[command]
pub fn get_serial_number_state_command() -> Result<String, String> {
    let serial_state = get_serial_number_state();
    let states = serialize_serial_state(serial_state);
    Ok(format!("Serial state: {}", states.join(", ")))
}

#[command]
pub fn get_serial_number_data_command()
-> Result<SerializableSerialNumberData, String> {
    match get_serial_number_data() {
        Some(data) => Ok(data.into()),
        None => Err("No serial number data available".into()),
    }
}

#[command]
pub fn activate_license_command(code: String) -> Result<String, String> {
    activate_license(code)
        .map_err(|status| activation_status_to_error_message(status))
}

#[command]
pub fn deactivate_license_command(serial: String) -> Result<String, String> {
    deactivate_license(serial)
        .map(|_| "License successfully deactivated.".to_string())
        .map_err(|status| activation_status_to_error_message(status))
}
