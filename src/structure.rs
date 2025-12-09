// Typed, field-based definitions for all protocol structures.
// Legacy byte-wrapper implementations are kept below in a comment block for reference.

use std::sync::{Arc, RwLock};

use futures::{executor::block_on, lock::Mutex};
use hid_rs::SafeCallback2;

pub trait SayoObject {
    fn end_change(&self);
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
    key_fn_event: RwLock<Vec<SafeCallback2<u128, bool, ()>>,>,
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

    pub fn model_code(&self) -> Option<u16> {
        self.raw.model_code(None)
    }
    pub fn ver(&self) -> Option<u16> {
        self.raw.ver(None)
    }
    pub fn usb0_ori(&self) -> Option<u8> {
        self.raw.usb0_ori(None)
    }
    pub fn usb1_ori(&self) -> Option<u8> {
        self.raw.usb1_ori(None)
    }
    pub fn batt_lv(&self) -> Option<u8> {
        self.raw.batt_lv(None)
    }
    pub fn key_fn(&self) -> Option<u8> {
        self.raw.key_fn(None)
    }
    pub fn set_key_fn(&self, key_fn: u8) {
        let _ = self.raw.key_fn(Some(key_fn));
        self.end_change();
    }
    pub fn cpu_load_1s(&self) -> Option<u8> {
        self.raw.cpu_load_1s(None)
    }
    pub fn cpu_load_1ms(&self) -> Option<u8> {
        self.raw.cpu_load_1ms(None)
    }
    pub fn api_list(&self) -> Option<Vec<u8>> {
        self.raw.api_list(None)
    }
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
pub struct KeyData<T: SayoObject> {
    parent: Arc<Mutex<Option<T>>>,
    raw: sayo_api_rs::structures::KeyData,
}
impl<T: SayoObject> KeyData<T> {
    pub fn key_mode(&self) -> Option<u8> {
        self.raw.key_mode(None)
    }
    pub fn set_key_mode(&self, mode: u8) {
        let _ = self.raw.key_mode(Some(mode));
        block_on(self.parent.lock()).as_ref().unwrap().end_change();
    }
    pub fn key_opt0(&self) -> Option<u8> {
        self.raw.key_opt0(None)
    }
    pub fn set_key_opt0(&self, opt0: u8) {
        let _ = self.raw.key_opt0(Some(opt0));
        block_on(self.parent.lock()).as_ref().unwrap().end_change();
    }
    pub fn key_opt1(&self) -> Option<u8> {
        self.raw.key_opt1(None)
    }
    pub fn set_key_opt1(&self, opt1: u8) {
        let _ = self.raw.key_opt1(Some(opt1));
        block_on(self.parent.lock()).as_ref().unwrap().end_change();
    }
    pub fn key_opt2(&self) -> Option<u8> {
        self.raw.key_opt2(None)
    }
    pub fn set_key_opt2(&self, opt2: u8) {
        let _ = self.raw.key_opt2(Some(opt2));
        block_on(self.parent.lock()).as_ref().unwrap().end_change();
    }
    pub fn key_val(&self) -> Option<Vec<u8>> {
        self.raw.key_val(None)
    }
    pub fn set_key_val(&self, val: Vec<u8>) {
        let _ = self.raw.key_val(Some(val));
        block_on(self.parent.lock()).as_ref().unwrap().end_change();
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct KeyInfo {
    id: u8,
    api: sayo_api_rs::device::SayoDeviceApi,
    raw: sayo_api_rs::structures::KeyInfo,
    key_data: RwLock<Vec<KeyData<KeyInfo>>>,
}
impl SayoObject for KeyInfo {
    fn end_change(&self) {
        block_on(self.api.set_key_info(self.id, &self.raw));
    }
}
impl KeyInfo {
    pub fn new(id: u8, api: sayo_api_rs::device::SayoDeviceApi, raw: sayo_api_rs::structures::KeyInfo) -> Arc<Mutex<Option<Self>>> {
        let wrapper = Arc::new(Mutex::new(None));
        let res = Self {
            id,
            api,
            raw,
            key_data: RwLock::new(Vec::new()),
        };
        let key_datas = res.raw
            .key_fn()
            .unwrap_or_default()
            .into_iter()
            .map(|kd_raw| KeyData {
                parent: wrapper.clone(),
                raw: kd_raw,
            })
            .collect();
        *res.key_data.write().unwrap() = key_datas;
        block_on(wrapper.lock()).replace(res);
        wrapper
    }

    pub fn valid(&self) -> Option<u8> {
        self.raw.valid(None)
    }
    pub fn key_class(&self) -> Option<u8> {
        self.raw.key_class(None)
    }
    pub fn x(&self) -> Option<u16> {
        self.raw.key_site_x(None)
    }
    pub fn y(&self) -> Option<u16> {
        self.raw.key_site_y(None)
    }
    pub fn width(&self) -> Option<u16> {
        self.raw.key_width(None)
    }
    pub fn height(&self) -> Option<u16> {
        self.raw.key_height(None)
    }
    pub fn corner_radius(&self) -> Option<u16> {
        self.raw.fillet_angle(None)
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