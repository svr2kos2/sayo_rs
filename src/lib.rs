pub mod sayo_device;
pub mod structure;
pub mod cmd_response;

pub use sayo_device::SayoDevice;
pub use structure::{DeviceInfo, KeyData, KeyInfo};
pub use cmd_response::DeviceInfoDiff;

// Re-export frequently used raw protocol structures so upper layers (e.g. FFI host)
// can depend only on `sayo_rs`.
pub use sayo_api_rs::structures::BroadCast;
pub use sayo_api_rs::structures::HidReportHeader;

