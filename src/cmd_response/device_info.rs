use sayo_api_rs::byte_converter::RwBytes;
use sayo_api_rs::structures_codec::CodecableHidPackage;

use crate::cmd_response::traits::CmdResponseObject;
use crate::structure::DeviceInfo;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeviceInfoDiff {
    pub key_fn_changed: Option<u8>,
}

impl DeviceInfo {
    /// Construct a `DeviceInfo` from an incoming CMD response payload (cmd=0x00).
    ///
    /// Note: this does NOT trigger any device write; it only initializes the local cached view.
    pub fn from_cmd_response_bytes(uuid: u128, payload: Vec<u8>) -> Self {
        let api = sayo_api_rs::device::SayoDeviceApi::from_uuid(uuid);
        let raw = sayo_api_rs::structures::DeviceInfo::new(RwBytes::new(payload));
        Self::new(api, raw)
    }

    /// Apply an incoming CMD response payload (cmd=0x00) to the local cache and return a diff.
    ///
    /// This method must NEVER call `end_change()`; it is meant for passive updates (broadcast/cmd response).
    pub fn apply_cmd_response_local_bytes(&self, payload: &[u8]) -> DeviceInfoDiff {
        let incoming = sayo_api_rs::structures::DeviceInfo::new(RwBytes::new(payload.to_vec()));

        let old_key_fn = self.raw_inner().key_fn(None);
        let new_key_fn = incoming.key_fn(None);

        // Keep local cache in sync (best-effort; ignore write failures).
        let raw = self.raw_inner();
        if let Some(v) = incoming.model_code(None) { let _ = raw.model_code(Some(v)); }
        if let Some(v) = incoming.ver(None) { let _ = raw.ver(Some(v)); }
        if let Some(v) = incoming.usb0_ori(None) { let _ = raw.usb0_ori(Some(v)); }
        if let Some(v) = incoming.usb0_offset(None) { let _ = raw.usb0_offset(Some(v)); }
        if let Some(v) = incoming.usb1_ori(None) { let _ = raw.usb1_ori(Some(v)); }
        if let Some(v) = incoming.usb1_offset(None) { let _ = raw.usb1_offset(Some(v)); }
        if let Some(v) = incoming.batt_lv(None) { let _ = raw.batt_lv(Some(v)); }
        if let Some(v) = incoming.key_fn(None) { let _ = raw.key_fn(Some(v)); }
        if let Some(v) = incoming.cpu_load_1s(None) { let _ = raw.cpu_load_1s(Some(v)); }
        if let Some(v) = incoming.cpu_load_1ms(None) { let _ = raw.cpu_load_1ms(Some(v)); }
        if let Some(v) = incoming.api_list(None) { let _ = raw.api_list(Some(v)); }

        let key_fn_changed = match (old_key_fn, new_key_fn) {
            (Some(old), Some(new)) if old != new => Some(new),
            (None, Some(new)) => Some(new),
            _ => None,
        };

        DeviceInfoDiff { key_fn_changed }
    }
}

impl CmdResponseObject for DeviceInfo {
    type Diff = DeviceInfoDiff;

    fn from_cmd_response_bytes(uuid: u128, payload: Vec<u8>) -> Self {
        DeviceInfo::from_cmd_response_bytes(uuid, payload)
    }

    fn apply_cmd_response_local_bytes(&self, payload: &[u8]) -> Self::Diff {
        DeviceInfo::apply_cmd_response_local_bytes(self, payload)
    }
}
