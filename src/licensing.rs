use serde::Serialize;
use std::ffi::{CString, NulError};
use tauri::command;
use vmprotect::licensing::{
    ActivationStatus, SerialNumber, activate_license, deactivate_license, get_hwid,
    get_serial_number_data, get_serial_number_state, set_serial_number,
};

use crate::helper::*;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum LicenseError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Bad license state")]
    BadLicenseState(String),
    #[error("Activation error: {0}")]
    ActivationError(String),
}

impl From<NulError> for LicenseError {
    fn from(err: NulError) -> Self {
        LicenseError::InvalidInput(err.to_string())
    }
}

impl From<ActivationStatus> for LicenseError {
    fn from(status: ActivationStatus) -> Self {
        match status {
            ActivationStatus::Ok => {
                // ok status should never appear in error position
                unreachable!(
                    "ActivationStatus::Ok should not be converted to an error"
                )
            }
            _ => LicenseError::ActivationError(
                activation_status_to_error_message(status).to_string(),
            ),
        }
    }
}

pub type LicenseResult<T> = Result<T, LicenseError>;

#[command]
pub fn get_hwid_command() -> String {
    get_hwid()
}

#[command]
pub fn set_serial_number_command(serial_str: String) -> LicenseResult<()> {
    let serial_number = SerialNumber::from(serial_str);

    let serial_state = set_serial_number(&serial_number);
    let bad_state = serialize_serial_state(serial_state);

    if !bad_state.is_empty() {
        return Err(LicenseError::BadLicenseState(bad_state.join(", ")));
    }

    Ok(())
}

#[command]
pub fn get_serial_number_state_command() -> LicenseResult<()> {
    let serial_state = get_serial_number_state();
    let states = serialize_serial_state(serial_state);

    if !states.is_empty() {
        return Err(LicenseError::BadLicenseState(states.join(", ")));
    }
    Ok(())
}

#[command]
pub fn get_serial_number_data_command() -> Option<SerializableSerialNumberData> {
    get_serial_number_data().map(|data| data.into())
}

#[command]
pub fn activate_license_command(code: String) -> LicenseResult<String> {
    let code = CString::new(code)?;
    activate_license(code.as_c_str())
        .map(|serial| serial.export_to_string())
        .map_err(LicenseError::from)
}

#[command]
pub fn deactivate_license_command(serial: String) -> LicenseResult<()> {
    let serial_number = SerialNumber::from(serial);
    deactivate_license(serial_number).map_err(LicenseError::from)
}
