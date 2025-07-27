#![no_std]
#![no_main]

mod ble;
mod buttons;
mod co_sensor;
mod display_task;

use co_sensor::sensor_task;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use microbit_bsp::{display, Microbit};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting...");
    let b = Microbit::default();
    let mut display = b.display;
    display.set_brightness(display::Brightness::MAX);
    spawner.must_spawn(sensor_task(b.twispi0, b.p20, b.p19));
    spawner.must_spawn(display_task::display_task(display));
    spawner.must_spawn(buttons::button_task(b.btn_a, b.btn_b));
    let (sdc, mpsl) = b.ble.init(b.timer0, b.rng).unwrap();
    ble::run(sdc, mpsl, spawner).await;
}
