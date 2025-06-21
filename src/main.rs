#![no_std]
#![no_main]

mod buttons;
mod co_sensor;
mod display_task;

use buttons::buttons_task;
use co_sensor::sensor_task;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
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
    let btn_a_task = buttons_task(b.btn_a, "A");
    let btn_b_task = buttons_task(b.btn_b, "B");
    join(btn_a_task, btn_b_task).await;
}
