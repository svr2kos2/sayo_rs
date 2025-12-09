use std::sync::Arc;

use futures::lock::Mutex as AsyncMutex;

use super::structure::*;

type Field<T> = Arc<AsyncMutex<Option<T>>>;

pub struct SayoDevice {
     api: sayo_api_rs::device::SayoDeviceApi,
     pub device_info: Field<DeviceInfo>,
     // system_info: Field<SystemInfo>,
     // device_config: Field<DeviceConfig>,
     // rf_config: Field<RFConfig>,
     pub key_infos: Vec<Field<KeyInfo>>,
     // led_infos: Vec<Field<LEDInfo>>,
     // color_tables: Vec<Field<ColorTable>>,
     // touch_sensitivities: Vec<Field<TouchSensitivity>>,
     // analog_key_infos: Vec<Field<AnalogKeyInfo>>,
     // scripts: Vec<Field<SayoScript>>,
     // analog_key_infos2: Vec<Field<AnalogKeyInfo2>>,
     // advanced_key_binding: Field<AdvancedKeyBinding>,
     // display_assets: Field<DisplayAssets>,
     // lcd_draw_data: Field<LcdDrawData>,
     // led_effects: Vec<Field<LedEffect>>,
     // gamepad_cfg: Field<GamePadCfg>,
     // ambient_led: Field<AmbientLED>,
}

impl SayoDevice {
    pub fn new(uuid: u128) -> Self {
        SayoDevice {
            api: sayo_api_rs::device::SayoDeviceApi::from_uuid(uuid),
            device_info: Arc::new(AsyncMutex::new(None)),
            key_infos: vec![],
        }
    }

    pub async fn init_device_info(&self) {
        if self.device_info.lock().await.is_some() {
            return;
        }
        let device_info = self.api.get_device_info().await;
        if device_info.is_none() {
            return;
        }
        let mut guard = self.device_info.lock().await;
        *guard = Some(DeviceInfo::new(self.api.clone(), device_info.unwrap()));
    }

    pub async fn init_key_infos(&mut self) {
        if self.key_infos.len() > 0 {
            return;
        }
        let key_infos =
            self.api
                .get_key_infos()
                .await
                .iter()
                .enumerate()
                .map(|(idx, ki_raw)| {
                    KeyInfo::new(idx as u8, self.api.clone(), ki_raw.clone())
                })
                .collect();

        self.key_infos = key_infos;
    }
    
}

