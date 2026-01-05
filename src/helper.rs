use std::time::Duration;

use base64::{Engine as _, engine::general_purpose};
use serde::Serialize;
use tauri::command;
use vmprotect::licensing::{ActivationStatus, SerialNumberData, SerialState};

#[derive(Serialize)]
pub struct SerializableSerialNumberData {
    state: Vec<&'static str>,
    user_name: String,
    email: String,
    expire: Option<String>,
    max_build: Option<String>,
    running_time: String, // Represented as a readable string
    user_data: String,    // Base64 encoded
}

#[inline]
fn format_duration(duration: Duration) -> String {
    format!("{} seconds", duration.as_secs())
}

pub fn serialize_serial_state(serial_state: SerialState) -> Vec<&'static str> {
    let flags = [
        (SerialState::CORRUPTED, "Corrupted"),
        (SerialState::INVALID, "Invalid"),
        (SerialState::BLACKLISTED, "Blacklisted"),
        (SerialState::DATE_EXPIRED, "Date Expired"),
        (SerialState::RUNNING_TIME_OVER, "Running Time Over"),
        (SerialState::BAD_HWID, "Bad HWID"),
        (SerialState::MAX_BUILD_EXPIRED, "Max Build Expired"),
    ];

    flags
        .iter()
        .filter(|(flag, _)| serial_state.contains(*flag))
        .map(|(_, desc)| *desc)
        .collect()
}

impl From<SerialNumberData> for SerializableSerialNumberData {
    fn from(data: SerialNumberData) -> Self {
        SerializableSerialNumberData {
            state: serialize_serial_state(data.state),
            user_name: data.user_name,
            email: data.email,
            expire: data
                .expire
                .and_then(|date| date.and_hms_opt(0, 0, 0))
                .map(|dt| dt.and_utc().to_rfc3339()),
            max_build: data
                .max_build
                .and_then(|date| date.and_hms_opt(0, 0, 0))
                .map(|dt| dt.and_utc().to_rfc3339()),
            running_time: format_duration(data.running_time),
            user_data: general_purpose::STANDARD.encode(&data.user_data),
        }
    }
}

pub fn activation_status_to_error_message(status: ActivationStatus) -> &'static str {
    match status {
        ActivationStatus::Ok => "OK",
        ActivationStatus::SmallBuffer => "The provided buffer is too small",
        ActivationStatus::NoConnection => "No connection to the activation server.",
        ActivationStatus::BadReply => "Received a bad reply from the server.",
        ActivationStatus::Banned => "The license has been banned",
        ActivationStatus::Corrupted => "The license data is corrupted",
        ActivationStatus::BadCode => "The provided license code is invalid.",
        ActivationStatus::AlreadyUsed => "The license has already been used.",
        ActivationStatus::SerialUnknown => "The serial number is unknown.",
        ActivationStatus::Expired => "The license has expired.",
        ActivationStatus::NotAvailable => "The license is not available.",
    }
}

#[derive(Serialize)]
pub struct Features {
    pub licensing: bool,
    pub service: bool,
}

impl Default for Features {
    fn default() -> Self {
        Self::new()
    }
}

impl Features {
    pub fn new() -> Features {
        Features {
            licensing: cfg!(feature = "licensing"),
            service: cfg!(feature = "service"),
        }
    }
}

#[command]
pub fn feature_check_command() -> Features {
    Features::new()
}
