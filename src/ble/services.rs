use trouble_host::prelude::Uuid;

pub mod battery;
pub mod configuration;
pub mod environment;
pub mod motion;
pub mod sound;
pub mod ui;

// Thingy base UUID:
// EF68xxxx-9B35-4933-9B10-52FFA9740042
const THINGY_BASE_UUID: [u8; 16] = [
    0x42, 0x00, 0x74, 0xA9, 0xFF, 0x52, 0x10, 0x9B, 0x33, 0x49, 0x35, 0x9B, 0x00, 0x00, 0x68, 0xEF,
];

pub struct ThingyUuid(pub u16);

impl ThingyUuid {
    const fn into_u128(self) -> u128 {
        u128::from_le_bytes(THINGY_BASE_UUID) | ((self.0 as u128) << 96)
    }
}

impl From<ThingyUuid> for Uuid {
    fn from(value: ThingyUuid) -> Self {
        value.into_u128().into()
    }
}

impl From<ThingyUuid> for [u8; 16] {
    fn from(value: ThingyUuid) -> Self {
        value.into_u128().to_le_bytes()
    }
}

trait PackedCStruct: Copy {}

#[macro_export]
macro_rules! impl_fixedgattvalue {
    ($type:ty) => {
        impl super::PackedCStruct for $type {}

        impl trouble_host::types::gatt_traits::FixedGattValue for $type {
            const SIZE: usize = core::mem::size_of::<Self>();

            fn from_gatt(
                data: &[u8],
            ) -> Result<Self, trouble_host::types::gatt_traits::FromGattError> {
                if data.len() != Self::SIZE {
                    Err(trouble_host::types::gatt_traits::FromGattError::InvalidLength)
                } else {
                    // SAFETY
                    // - Pointer is considered "valid" as per the rules outlined for validity in std::ptr v1.82.0
                    // - Pointer was generated from a slice of bytes matching the size of the type, and all packed C structures composed of primatives are valid for all possible configurations of bits
                    // - PackedCStruct trait is constrained to require Copy
                    unsafe { Ok((data.as_ptr() as *const Self).read_unaligned()) }
                }
            }

            fn as_gatt(&self) -> &[u8] {
                super::as_bytes(self)
            }
        }
    };
}

pub const fn as_bytes<T>(t: &T) -> &[u8] {
    // SAFETY
    // - Slice is of type u8 so data is guaranteed valid for reads of any length
    // - Data and len are tied to the address and size of the type
    unsafe { core::slice::from_raw_parts((t as *const T) as *const u8, core::mem::size_of::<T>()) }
}
