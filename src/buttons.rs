use defmt::info;
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    watch::{DynReceiver, Watch},
};
use embassy_time::Timer;
use microbit_bsp::Button;

const CONSUMERS: usize = 2;

#[derive(Clone, Copy)]
pub enum ShowSensorData {
    Temperature,
    Humidity,
    CO2,
}

static SHOW_STATE: Watch<ThreadModeRawMutex, ShowSensorData, CONSUMERS> = Watch::new();

pub fn get_show_receiver() -> Option<DynReceiver<'static, ShowSensorData>> {
    SHOW_STATE.dyn_receiver()
}

pub async fn buttons_task(mut btn: Button, btn_id: &str) {
    const DEBOUNSE_DELAY: u64 = 200;
    let tx = SHOW_STATE.sender();
    let mut display_state: ShowSensorData = ShowSensorData::CO2;
    loop {
        btn.wait_for_low().await;
        info!("Btn {} is pressed", btn_id);
        display_state = next_state(display_state);
        tx.send(display_state);
        Timer::after_millis(DEBOUNSE_DELAY).await;
        btn.wait_for_high().await;
    }
}

fn next_state(state: ShowSensorData) -> ShowSensorData {
    match state {
        ShowSensorData::CO2 => ShowSensorData::Humidity,
        ShowSensorData::Humidity => ShowSensorData::Temperature,
        ShowSensorData::Temperature => ShowSensorData::CO2,
    }
}
