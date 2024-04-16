use std::time::Duration;

use serde::Serialize;
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
            expire: data.expire.map(|date| date.and_hms(0, 0, 0).to_rfc3339()),
            max_build: data
                .max_build
                .map(|date| date.and_hms(0, 0, 0).to_rfc3339()),
            running_time: format_duration(data.running_time),
            user_data: base64::encode(&data.user_data),
        }
    }
}

pub fn activation_status_to_error_message(status: ActivationStatus) -> String {
    match status {
        ActivationStatus::Ok => "Unexpected OK status".to_string(),
        ActivationStatus::SmallBuffer => "The provided buffer is too small.".to_string(),
        ActivationStatus::NoConnection => "No connection to the activation server.".to_string(),
        ActivationStatus::BadReply => "Received a bad reply from the server.".to_string(),
        ActivationStatus::Banned => "The license has been banned.".to_string(),
        ActivationStatus::Corrupted => "The license data is corrupted.".to_string(),
        ActivationStatus::BadCode => "The provided license code is invalid.".to_string(),
        ActivationStatus::AlreadyUsed => "The license has already been used.".to_string(),
        ActivationStatus::SerialUnknown => "The serial number is unknown.".to_string(),
        ActivationStatus::Expired => "The license has expired.".to_string(),
        ActivationStatus::NotAvailable => "The license is not available.".to_string(),
        ActivationStatus::NulError => "The license contains a NUL character.".to_string(),
    }
}
