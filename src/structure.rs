// Typed, field-based definitions for all protocol structures.
// Legacy byte-wrapper implementations are kept below in a comment block for reference.

use std::sync::RwLock;

use pollster::block_on;
use hid_rs::SafeCallback2;

pub trait SayoObject {
    fn end_change(&self);
}

fn missing(field: &str) -> String {
    format!("{field} unavailable")
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct StringContent {
    raw: Option<sayo_api_rs::structures::StringContent>,
}
#[repr(C)]
#[derive(Debug)]
pub struct DeviceInfo {
    api: sayo_api_rs::device::SayoDeviceApi,
    raw: sayo_api_rs::structures::DeviceInfo,
    key_fn_event: RwLock<Vec<SafeCallback2<u128, bool, ()>>>,
}
impl SayoObject for DeviceInfo {
    fn end_change(&self) {
        let _ = block_on(self.api.set_device_info(&self.raw));
    }
}
impl DeviceInfo {
    pub fn new(api: sayo_api_rs::device::SayoDeviceApi, raw: sayo_api_rs::structures::DeviceInfo) -> Self {
        Self {
            api,
            raw,
            key_fn_event: RwLock::new(Vec::new()),
        }
    }

    pub fn model_code(&self) -> Result<u16, String> { self.raw.model_code(None).ok_or_else(|| missing("model_code")) }
    pub fn ver(&self) -> Result<u16, String> { self.raw.ver(None).ok_or_else(|| missing("ver")) }
    pub fn usb0_ori(&self) -> Result<u8, String> { self.raw.usb0_ori(None).ok_or_else(|| missing("usb0_ori")) }
    pub fn usb0_offset(&self) -> Result<u8, String> { self.raw.usb0_offset(None).ok_or_else(|| missing("usb0_offset")) }
    pub fn usb1_ori(&self) -> Result<u8, String> { self.raw.usb1_ori(None).ok_or_else(|| missing("usb1_ori")) }
    pub fn usb1_offset(&self) -> Result<u8, String> { self.raw.usb1_offset(None).ok_or_else(|| missing("usb1_offset")) }
    pub fn batt_lv(&self) -> Result<u8, String> { self.raw.batt_lv(None).ok_or_else(|| missing("batt_lv")) }
    pub fn key_fn(&self) -> Result<u8, String> { self.raw.key_fn(None).ok_or_else(|| missing("key_fn")) }
    pub fn set_key_fn(&self, key_fn: u8) {
        let _ = self.raw.key_fn(Some(key_fn));
        self.end_change();
    }

    /// 仅本地更新缓存值，不触发设备写入（用于广播同步）。
    pub fn set_key_fn_local(&self, key_fn: u8) {
        let _ = self.raw.key_fn(Some(key_fn));
    }
    pub fn cpu_load_1s(&self) -> Result<u8, String> { self.raw.cpu_load_1s(None).ok_or_else(|| missing("cpu_load_1s")) }
    pub fn cpu_load_1ms(&self) -> Result<u8, String> { self.raw.cpu_load_1ms(None).ok_or_else(|| missing("cpu_load_1ms")) }
    pub fn api_list(&self) -> Result<Vec<u8>, String> { self.raw.api_list(None).ok_or_else(|| missing("api_list")) }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SystemInfo {
    raw: sayo_api_rs::structures::SystemInfo,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DeviceConfig {
    raw: sayo_api_rs::structures::DeviceConfig,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RFConfig {
    raw: sayo_api_rs::structures::RFConfig,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeyData {
    raw: sayo_api_rs::structures::KeyData,
}

impl KeyData {
    pub fn new(raw: sayo_api_rs::structures::KeyData) -> Self {
        Self { raw }
    }

    pub fn key_mode(&self) -> Result<u8, String> { self.raw.key_mode(None).ok_or_else(|| missing("key_mode")) }
    pub fn key_opt0(&self) -> Result<u8, String> { self.raw.key_opt0(None).ok_or_else(|| missing("key_opt0")) }
    pub fn key_opt1(&self) -> Result<u8, String> { self.raw.key_opt1(None).ok_or_else(|| missing("key_opt1")) }
    pub fn key_opt2(&self) -> Result<u8, String> { self.raw.key_opt2(None).ok_or_else(|| missing("key_opt2")) }
    pub fn key_val(&self) -> Result<Vec<u8>, String> { self.raw.key_val(None).ok_or_else(|| missing("key_val")) }

    pub fn set_key_mode(&self, value: u8) {
        let _ = self.raw.key_mode(Some(value));
    }

    pub fn set_key_opt0(&self, value: u8) {
        let _ = self.raw.key_opt0(Some(value));
    }

    pub fn set_key_opt1(&self, value: u8) {
        let _ = self.raw.key_opt1(Some(value));
    }

    pub fn set_key_opt2(&self, value: u8) {
        let _ = self.raw.key_opt2(Some(value));
    }

    pub fn set_key_val(&self, value: Vec<u8>) {
        let _ = self.raw.key_val(Some(value));
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeyInfo {
    id: u8,
    api: sayo_api_rs::device::SayoDeviceApi,
    raw: sayo_api_rs::structures::KeyInfo,
}

impl SayoObject for KeyInfo {
    fn end_change(&self) {
        block_on(self.api.set_key_info(self.id, &self.raw));
    }
}

impl KeyInfo {
    pub fn new(id: u8, api: sayo_api_rs::device::SayoDeviceApi, raw: sayo_api_rs::structures::KeyInfo) -> Self {
        Self { id, api, raw }
    }

    pub fn valid(&self) -> Result<u8, String> { self.raw.valid(None).ok_or_else(|| missing("valid")) }
    pub fn key_class(&self) -> Result<u8, String> { self.raw.key_class(None).ok_or_else(|| missing("key_class")) }
    pub fn x(&self) -> Result<u16, String> { self.raw.key_site_x(None).ok_or_else(|| missing("x")) }
    pub fn y(&self) -> Result<u16, String> { self.raw.key_site_y(None).ok_or_else(|| missing("y")) }
    pub fn width(&self) -> Result<u16, String> { self.raw.key_width(None).ok_or_else(|| missing("width")) }
    pub fn height(&self) -> Result<u16, String> { self.raw.key_height(None).ok_or_else(|| missing("height")) }
    pub fn corner_radius(&self) -> Result<u16, String> { self.raw.fillet_angle(None).ok_or_else(|| missing("corner_radius")) }

    pub fn key_fn(&self) -> Vec<KeyData> {
        self
            .raw
            .key_fn()
            .unwrap_or_default()
            .into_iter()
            .map(KeyData::new)
            .collect()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LedData {
    raw: sayo_api_rs::structures::LedData,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LEDInfo {
    raw: sayo_api_rs::structures::LEDInfo,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ColorTable {
    raw: sayo_api_rs::structures::ColorTable,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TouchSensitivity {
    raw: sayo_api_rs::structures::TouchSensitivity,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnalogKeyInfo {
    raw: sayo_api_rs::structures::AnalogKeyInfo,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SayoScript {
    raw_name: sayo_api_rs::structures::StringContent,
    raw_content: sayo_api_rs::structures::SayoScriptContent,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnalogKeyInfo2 {
    raw: sayo_api_rs::structures::AnalogKeyInfo2,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AdvancedKeyBinding {
    raw: sayo_api_rs::structures::AdvancedKeyBinding,
}
// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct TriggerKeyboardHid {
//     pub modifier_keys: u8,
//     pub reserve0: u8,
//     pub key_code: [u8; 4],
// }
// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct TriggerMouseHid {
//     pub mouse_keys: u8,
//     pub x: u8,
//     pub y: u8,
//     pub scroll: u8,
// }
// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct TriggerMeidaHid {
//     pub key_code: u16,
// }
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DisplayData {
    raw: sayo_api_rs::structures::DisplayData,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DisplayAssets {
    raw: sayo_api_rs::structures::DisplayAssets,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LCDFill {
    raw: sayo_api_rs::structures::LCDFill,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LCDWidget {
    raw: sayo_api_rs::structures::LCDWidget,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LCDFont {
    raw: sayo_api_rs::structures::LCDFont,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LCDPic {
    raw: sayo_api_rs::structures::LCDPic,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LCDInfo {
    raw: sayo_api_rs::structures::LCDInfo,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LcdDrawData {
    raw: sayo_api_rs::structures::LcdDrawData,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LedEffect {
    raw: sayo_api_rs::structures::LedEffect,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GamePadCfg {
    raw: sayo_api_rs::structures::GamePadCfg,
}
#[repr(C)]
#[derive(Clone)]
pub struct AmbientLED {
    raw: sayo_api_rs::structures::AmbientLED,
}