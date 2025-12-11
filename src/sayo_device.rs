use std::sync::OnceLock;

use futures::executor::block_on;
use futures::Future;
use hid_rs::{SafeCallback, SafeCallback2};
use pollster::FutureExt;
use sayo_api_rs::device::{sub_broadcast, sub_cmd_response};
use sayo_api_rs::structures::{BroadCast, HidReportHeader};

use super::structure::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SayoDevice {
    api: sayo_api_rs::device::SayoDeviceApi,
}

impl SayoDevice {
    fn run_async<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> T {
        std::thread::spawn(move || block_on(future)).join().expect("async worker panicked")
    }

    pub fn ensure_runtime() {
        static INIT: OnceLock<()> = OnceLock::new();
        INIT.get_or_init(|| {
            println!("Initializing SayoDevice runtime...");
            Self::run_async(sayo_api_rs::device::init_sayo_device());
        });
    }

    pub fn new(uuid: u128) -> Self {
        Self::from_api(sayo_api_rs::device::SayoDeviceApi::from_uuid(uuid))
    }

    pub fn from_api(api: sayo_api_rs::device::SayoDeviceApi) -> Self {
        Self { api }
    }

    pub fn uuid(&self) -> u128 {
        self.api.uuid
    }

    pub async fn sub_cmd_response(&self, callback: &SafeCallback2<u128, (HidReportHeader, Vec<u8>), ()>,
        ) -> Result<(), String> {
        sub_cmd_response(self.api.uuid, callback).await
    }
    pub async fn unsub_cmd_response(&self) -> Result<(), String> {
        sayo_api_rs::device::unsub_cmd_response(self.api.uuid).await
    }

    pub async fn sub_broadcast(&self, callback: &SafeCallback2<u128, BroadCast, ()>) -> Result<(), String> {
        sub_broadcast(self.api.uuid, callback).await
    }

    pub async fn unsub_broadcast(&self) -> Result<(), String> {
        sayo_api_rs::device::unsub_broadcast(self.api.uuid).await
    }

    pub fn get_device_list() -> Vec<SayoDevice> {
        Self::ensure_runtime();
        Self::run_async(sayo_api_rs::device::get_device_list())
            .into_iter()
            .map(SayoDevice::from_api)
            .collect()
    }

    pub fn device_info(&self) -> Result<DeviceInfo, String> {
        Self::ensure_runtime();
        let api = self.api.clone();
        match Self::run_async(async move { api.get_device_info().await }) {
            Some(raw) => Ok(DeviceInfo::new(self.api.clone(), raw)),
            None => Err("device_info unavailable".to_string()),
        }
    }

    pub fn key_infos(&self) -> Result<Vec<KeyInfo>, String> {
        Self::ensure_runtime();
        let api = self.api.clone();
        let raws = Self::run_async(async move { api.get_key_infos().await });
        let infos = raws
            .into_iter()
            .enumerate()
            .map(|(idx, raw)| KeyInfo::new(idx as u8, self.api.clone(), raw))
            .collect();
        Ok(infos)
    }
}

