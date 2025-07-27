use trouble_host::{prelude::*, types::gatt_traits::FromGattError};

use super::{as_bytes, ThingyUuid};

pub const TSS: ThingyUuid = ThingyUuid(0x0500);

const TSS_CONFIG: ThingyUuid = ThingyUuid(0x0501);
const TSS_SPEAKER: ThingyUuid = ThingyUuid(0x0502);
const TSS_SPEAKER_STATUS: ThingyUuid = ThingyUuid(0x0503);
const TSS_MICROPHONE: ThingyUuid = ThingyUuid(0x0504);

#[gatt_service(uuid = TSS)]
pub struct ThingySoundService {
    #[characteristic(uuid = TSS_CONFIG, read, write)]
    pub configuration: [u8; 8],
    #[characteristic(uuid = TSS_SPEAKER, write_without_response)]
    pub speaker: TssSpeaker,
    #[characteristic(uuid = TSS_SPEAKER_STATUS, notify)]
    pub speaker_status: u8,
    #[characteristic(uuid = TSS_MICROPHONE, notify)]
    pub microphone: TssMicrophone,
}

#[repr(C, packed)]
pub struct TssSpeaker {
    pcm: [u8; 273],
}

impl Default for TssSpeaker {
    fn default() -> Self {
        Self { pcm: [0; 273] }
    }
}

impl FixedGattValue for TssSpeaker {
    const SIZE: usize = size_of::<Self>();

    fn from_gatt(data: &[u8]) -> Result<Self, FromGattError> {
        if data.len() != Self::SIZE {
            Err(FromGattError::InvalidLength)
        } else {
            Ok(Self {
                pcm: data.try_into().unwrap(),
            })
        }
    }

    fn as_gatt(&self) -> &[u8] {
        as_bytes(self)
    }
}

#[repr(C, packed)]
pub struct TssMicrophone {
    raw: [u8; 131],
}

impl Default for TssMicrophone {
    fn default() -> Self {
        Self { raw: [0; 131] }
    }
}

impl FixedGattValue for TssMicrophone {
    const SIZE: usize = size_of::<Self>();

    fn from_gatt(data: &[u8]) -> Result<Self, FromGattError> {
        if data.len() != Self::SIZE {
            Err(FromGattError::InvalidLength)
        } else {
            Ok(Self {
                raw: data.try_into().unwrap(),
            })
        }
    }

    fn as_gatt(&self) -> &[u8] {
        as_bytes(self)
    }
}
