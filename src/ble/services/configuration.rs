use heapless::String;
use trouble_host::prelude::*;

use crate::impl_fixedgattvalue;

use super::ThingyUuid;

pub const TCS: ThingyUuid = ThingyUuid(0x0100);

const TCS_DEVICE_NAME: ThingyUuid = ThingyUuid(0x0101);
const TCS_ADV_PARAMS: ThingyUuid = ThingyUuid(0x0102);
const TCS_CONN_PARAMS: ThingyUuid = ThingyUuid(0x0104);
const TCS_BEACON_DATA: ThingyUuid = ThingyUuid(0x0105);
const TCS_CLOUD_DATA: ThingyUuid = ThingyUuid(0x0106);
const TCS_FW_VERSION: ThingyUuid = ThingyUuid(0x0107);
const TCS_MTU: ThingyUuid = ThingyUuid(0x0108);
const TCS_NFC: ThingyUuid = ThingyUuid(0x0109);

pub const BLE_NAME: &str = "co2_sensor";
pub const MSP_NORDIC_COMPANY_ID: u16 = 0x0059;
pub const MSP_PAYLOAD: [u8; 4] = [0x01, 0x02, 0x03, 0x04];

#[gatt_service(uuid = TCS)]
pub struct ThingyConfigurationService {
    #[characteristic(uuid = TCS_DEVICE_NAME, read, write, value = BLE_NAME.parse().unwrap())]
    device_name: String<10>,

    #[characteristic(uuid = TCS_ADV_PARAMS, read, write)]
    adv_params: TcsAdvertisingParameters,

    #[characteristic(uuid = TCS_CONN_PARAMS, read, write)]
    conn_params: TcsConnectionParameters,

    #[characteristic(uuid = TCS_BEACON_DATA, read, write, value = "\x03goo.gl/pIWdir".parse().unwrap())]
    beacon_data: String<14>,
    #[characteristic(uuid = TCS_CLOUD_DATA, read, write, value = Default::default())]
    token_data: String<20>,
    #[characteristic(uuid = TCS_FW_VERSION, read, value = [0x02, 0x02, 0x00])]
    fw_version: [u8; 3],
    #[characteristic(uuid = TCS_MTU, read, write, value = [0x00, 23, 0x00])]
    mtu: [u8; 3],
    #[characteristic(uuid = TCS_NFC, read, write, value = "nordicsemi.com/thingy\0".parse().unwrap())]
    nfc: String<22>,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TcsAdvertisingParameters {
    interval: u16,
    timeout: u8,
}

impl ToTimeUnits for TcsAdvertisingParameters {
    const DIVISOR: usize = 625;
}

impl Default for TcsAdvertisingParameters {
    fn default() -> Self {
        AdvertisingParameters::default().into()
    }
}

impl From<AdvertisingParameters> for TcsAdvertisingParameters {
    fn from(value: AdvertisingParameters) -> Self {
        Self {
            interval: Self::from_ms(value.interval_ms) as u16,
            timeout: value.timeout_s as u8,
        }
    }
}

impl_fixedgattvalue!(TcsAdvertisingParameters);

trait ToTimeUnits {
    const DIVISOR: usize;

    fn from_ms(ms: usize) -> usize {
        Self::from_us(ms * 1000)
    }

    fn from_us(us: usize) -> usize {
        us / Self::DIVISOR
    }
}

pub struct AdvertisingParameters {
    interval_ms: usize,
    timeout_s: usize,
}

impl Default for AdvertisingParameters {
    fn default() -> Self {
        Self {
            interval_ms: 380,
            timeout_s: 180,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TcsConnectionParameters {
    min_interval: u16,
    max_interval: u16,
    slave_latency: u16,
    sup_timeout: u16,
}

impl Default for TcsConnectionParameters {
    fn default() -> Self {
        ConnectionParameters::default().into()
    }
}

impl ToTimeUnits for TcsConnectionParameters {
    const DIVISOR: usize = 25;
}

impl From<ConnectionParameters> for TcsConnectionParameters {
    fn from(value: ConnectionParameters) -> Self {
        Self {
            min_interval: Self::from_us(value.min_interval_us) as u16,
            max_interval: Self::from_us(value.max_interval_us) as u16,
            slave_latency: value.slave_latency as u16,
            sup_timeout: Self::from_ms(value.sup_timeout_ms) as u16,
        }
    }
}

impl_fixedgattvalue!(TcsConnectionParameters);

pub struct ConnectionParameters {
    min_interval_us: usize,
    max_interval_us: usize,
    slave_latency: usize,
    sup_timeout_ms: usize,
}

impl Default for ConnectionParameters {
    fn default() -> Self {
        Self {
            min_interval_us: 7_500,
            max_interval_us: 30_000,
            slave_latency: 0,
            sup_timeout_ms: 3_200,
        }
    }
}
