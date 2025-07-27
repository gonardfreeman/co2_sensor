use microbit_bsp::lsm303agr::Acceleration;
use trouble_host::{prelude::*, types::gatt_traits::FromGattError};

use crate::impl_fixedgattvalue;

use super::ThingyUuid;

pub const TMS: ThingyUuid = ThingyUuid(0x0400);

const TMS_CONFIG: ThingyUuid = ThingyUuid(0x0401);
const TMS_TAP: ThingyUuid = ThingyUuid(0x0402);
const TMS_ORIENTATION: ThingyUuid = ThingyUuid(0x403);
const TMS_QUATERNION: ThingyUuid = ThingyUuid(0x404);
const TMS_PEDOMETER: ThingyUuid = ThingyUuid(0x405);
const TMS_RAW: ThingyUuid = ThingyUuid(0x406);
const TMS_EULER: ThingyUuid = ThingyUuid(0x407);
const TMS_ROTATION_MATRIX: ThingyUuid = ThingyUuid(0x408);
const TMS_HEADING: ThingyUuid = ThingyUuid(0x409);
const TMS_GRAVITY: ThingyUuid = ThingyUuid(0x40A);

#[gatt_service(uuid = TMS)]
pub struct ThingyMotionService {
    #[characteristic(uuid = TMS_CONFIG, read, write)]
    pub config: TmsConfiguration,
    #[characteristic(uuid = TMS_TAP, notify)]
    pub tap: u16,
    #[characteristic(uuid = TMS_ORIENTATION, notify)]
    pub orientation: u8,
    #[characteristic(uuid = TMS_QUATERNION, notify)]
    pub quaternion: [u8; 16],
    #[characteristic(uuid = TMS_PEDOMETER, notify)]
    pub pedometer: u64,
    #[characteristic(uuid = TMS_RAW, notify)]
    pub raw: [u8; 18],
    #[characteristic(uuid = TMS_EULER, notify)]
    pub euler: [u8; 12],
    #[characteristic(uuid = TMS_ROTATION_MATRIX, notify)]
    pub rotation_matrix: [u8; 18],
    #[characteristic(uuid = TMS_HEADING, notify)]
    pub heading: i32,
    #[characteristic(uuid = TMS_GRAVITY, notify)]
    pub gravity: TmsGravity,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TmsConfiguration {
    pedometer_interval_ms: u16,
    temperature_interval_ms: u16,
    compass_interval_ms: u16,
    motion_frequency_hz: u16,
    wake_on_motion: u8,
}

impl Default for TmsConfiguration {
    fn default() -> Self {
        Self {
            pedometer_interval_ms: 1000,
            temperature_interval_ms: 500,
            compass_interval_ms: 500,
            motion_frequency_hz: 10,
            wake_on_motion: 1,
        }
    }
}

impl_fixedgattvalue!(TmsConfiguration);

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct TmsGravity {
    x: f32,
    y: f32,
    z: f32,
}

impl From<Acceleration> for TmsGravity {
    fn from(value: Acceleration) -> Self {
        Self {
            x: value.x_mg() as f32 * 0.0098f32,
            y: value.y_mg() as f32 * 0.0098f32,
            z: value.z_mg() as f32 * 0.0098f32,
        }
    }
}

impl_fixedgattvalue!(TmsGravity);
