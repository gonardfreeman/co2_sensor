use crate::buttons::{get_show_receiver, ShowSensorData};

use defmt::info;
use defmt_rtt as _;
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    watch::{DynReceiver, Watch},
};
use embassy_time::{Delay, Timer};
use libscd::asynchronous::scd4x::Scd4x;
use microbit_bsp::embassy_nrf::{
    bind_interrupts,
    peripherals::{P0_26, P1_00, TWISPI0},
    twim::{self, Twim},
    Peri,
};
use panic_probe as _;
use static_cell::ConstStaticCell;

const SENSOR_DATA_CONSUMERS: usize = 2;
static SENSOR_DATA: Watch<ThreadModeRawMutex, u16, SENSOR_DATA_CONSUMERS> = Watch::new();

pub fn get_receiver() -> Option<DynReceiver<'static, u16>> {
    SENSOR_DATA.dyn_receiver()
}

#[embassy_executor::task]
pub async fn sensor_task(
    twi: Peri<'static, TWISPI0>,
    sda: Peri<'static, P1_00>,
    scl: Peri<'static, P0_26>,
) {
    static RAM_BUFFER: ConstStaticCell<[u8; 4]> = ConstStaticCell::new([0; 4]);
    bind_interrupts!(struct Irqs {
        TWISPI0 => twim::InterruptHandler<TWISPI0>;
    });
    let i2c: Twim<'_, TWISPI0> =
        Twim::new(twi, Irqs, sda, scl, Default::default(), RAM_BUFFER.take());
    let mut scd = Scd4x::new(i2c, Delay);

    Timer::after_millis(30).await;

    _ = scd.stop_periodic_measurement().await;

    info!("Sensor sensor number: {:?}", scd.serial_number().await);

    if let Err(e) = scd.start_periodic_measurement().await {
        defmt::panic!("Failed to start periodic measurement: {:?}", e);
    }
    let mut rx = get_show_receiver().unwrap();
    let tx = SENSOR_DATA.sender();

    loop {
        if scd.data_ready().await.unwrap() {
            let m = scd.read_measurement().await.unwrap();
            let show_sensor: ShowSensorData = rx.get().await;

            match show_sensor {
                ShowSensorData::Temperature => tx.send(m.temperature as u16),
                ShowSensorData::Humidity => tx.send(m.humidity as u16),
                ShowSensorData::CO2 => tx.send(m.co2 as u16),
            }
            info!(
                "CO2: {}, Humidity: {}, Temperature: {}",
                m.co2 as u16, m.humidity as u16, m.temperature as u16
            );
        }
        Timer::after_millis(1000).await;
    }
}
