use trouble_host::prelude::*;

use super::ThingyUuid;

pub const TUS: ThingyUuid = ThingyUuid(0x0300);

const TUS_LED: ThingyUuid = ThingyUuid(0x0301);
const TUS_BUTTON: ThingyUuid = ThingyUuid(0x0302);
const TUS_PIN: ThingyUuid = ThingyUuid(0x0303);

#[gatt_service(uuid = TUS)]
pub struct ThingyUiService {
    #[characteristic(uuid = TUS_BUTTON, notify)]
    pub button: u8,
    #[characteristic(uuid = TUS_LED, read, write, value = 0)]
    pub led: u64,
    #[characteristic(uuid = TUS_PIN, read, write, value = 0)]
    pub pin: u32,
}
