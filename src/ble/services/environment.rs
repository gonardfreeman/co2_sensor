use defmt::info;
use trouble_host::{prelude::*, types::gatt_traits::FromGattError};

use crate::impl_fixedgattvalue;

use super::ThingyUuid;

pub const TES: ThingyUuid = ThingyUuid(0x200);

pub const TES_TEMPERATURE: ThingyUuid = ThingyUuid(0x0201);
pub const TES_PRESSURE: ThingyUuid = ThingyUuid(0x0202);
pub const TES_HUMIDITY: ThingyUuid = ThingyUuid(0x0203);
pub const TES_GAS: ThingyUuid = ThingyUuid(0x0204);
pub const TES_COLOR: ThingyUuid = ThingyUuid(0x0205);
pub const TES_CONFIG: ThingyUuid = ThingyUuid(0x0206);

#[gatt_service(uuid = TES)]
pub struct ThingyEnvironmentService {
    #[characteristic(uuid = TES_GAS, notify)]
    pub gas: TesGas,
    #[characteristic(uuid = TES_HUMIDITY, notify)]
    pub humidity: TesHumidity,
    #[characteristic(uuid = TES_TEMPERATURE, notify)]
    pub temperature: TesTemperature,
}

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct TesGas {
    co2_ppm: u16,
    tvoc_ppb: u16,
}

impl TesGas {
    pub const fn new(co2_ppm: u16) -> Self {
        Self {
            co2_ppm,
            tvoc_ppb: 0,
        }
    }
}

impl_fixedgattvalue!(TesGas);

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct TesTemperature {
    integer: i8,
    decimal: u8,
}

impl TesTemperature {
    pub const fn new(int: i8, dec: u8) -> Self {
        Self {
            integer: 20,
            decimal: 20,
        }
    }
}

impl_fixedgattvalue!(TesTemperature);

pub type TesHumidity = u8;

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct TesColor {
    red: u16,
    green: u16,
    blue: u16,
    clear: u16,
}

impl TesColor {
    pub const fn new(red: u16, green: u16, blue: u16, clear: u16) -> Self {
        Self {
            red,
            green,
            blue,
            clear,
        }
    }
}

impl_fixedgattvalue!(TesColor);

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TesConfiguration {
    temperature_interval_ms: u16,
    pressure_interval_ms: u16,
    humidity_interval_ms: u16,
    color_interval_ms: u16,
    gas_interval_mode: u8,
    color_config: [u8; 3],
}

impl Default for TesConfiguration {
    fn default() -> Self {
        Self {
            temperature_interval_ms: 2000,
            pressure_interval_ms: 2000,
            humidity_interval_ms: 2000,
            color_interval_ms: 1500,
            gas_interval_mode: 2,
            color_config: [107, 78, 29],
        }
    }
}

impl_fixedgattvalue!(TesConfiguration);
