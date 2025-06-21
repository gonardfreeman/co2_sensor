use crate::buttons::{get_show_receiver, ShowSensorData};
use crate::co_sensor;
use core::fmt::Write;
use embassy_time::Timer;
use heapless::String;
use microbit_bsp::{display::LedMatrix, embassy_nrf::gpio::Output};

const ROWS: usize = 5;
const COLS: usize = 5;

#[embassy_executor::task]
pub async fn display_task(mut matrix: LedMatrix<Output<'static>, ROWS, COLS>) {
    let mut rx = co_sensor::get_receiver().unwrap();
    let mut rx_show = get_show_receiver().unwrap();
    let mut txt: String<6> = String::new();
    loop {
        let cur_level: u16 = rx.get().await;
        let cur_display: ShowSensorData = rx_show.get().await;
        match cur_display {
            ShowSensorData::CO2 => write!(&mut txt, "{} ppm", cur_level).ok(),
            ShowSensorData::Humidity => write!(&mut txt, "{}%", cur_level).ok(),
            ShowSensorData::Temperature => write!(&mut txt, "{}C", cur_level).ok(),
        };
        matrix.scroll(txt.as_str()).await;
        txt.clear();
        Timer::after_millis(3000).await;
    }
}
