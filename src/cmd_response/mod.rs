//! Helpers for handling passive device updates coming from CMD responses / broadcasts.
//!
//! The key rule here: these helpers must **not** send writes back to the device.

pub mod device_info;
pub mod traits;

pub use device_info::DeviceInfoDiff;
pub use traits::CmdResponseObject;
