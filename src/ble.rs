#![allow(unused)]

pub mod services;

use core::error::Error;

use defmt::{info, warn};
use embassy_executor::Spawner;
use embassy_futures::select::select;
use microbit_bsp::{
    ble::{MultiprotocolServiceLayer, SoftdeviceController},
    embassy_nrf::pac::saadc::vals::Status,
};
use static_cell::StaticCell;
use trouble_host::{advertise, prelude::*};

use crate::{
    ble::services::{
        battery::BatteryService,
        configuration::{ThingyConfigurationService, BLE_NAME},
        environment::{TesGas, TesTemperature, ThingyEnvironmentService},
        motion::{ThingyMotionService, TmsGravity},
        sound::ThingySoundService,
        ui::ThingyUiService,
    },
    co_sensor,
};

const MAX_CONNECTIONS: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 2;

type BleHostResources = HostResources<DefaultPacketPool, MAX_CONNECTIONS, L2CAP_CHANNELS_MAX>;

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) {
    mpsl.run().await
}

#[gatt_server]
struct Server {
    env: ThingyEnvironmentService,
    config: ThingyConfigurationService,
    ui: ThingyUiService,
    sound: ThingySoundService,
    motion: ThingyMotionService,
    battery: BatteryService,
}

#[embassy_executor::task]
async fn host_task(mut runner: Runner<'static, SoftdeviceController<'static>, DefaultPacketPool>) {
    runner.run().await.unwrap();
}

async fn env_notifier(conn: &GattConnection<'_, '_, DefaultPacketPool>, server: &Server<'_>) {
    let gas = &server.env.gas;
    let mut rx = co_sensor::get_receiver().unwrap();
    loop {
        let co2 = rx.changed().await;
        if let Err(e) = gas.notify(conn, &TesGas::new(co2)).await {
            warn!("[gatt] notification error: {}", e);
        }
    }
}

pub async fn run(
    sdc: SoftdeviceController<'static>,
    mpsl: &'static MultiprotocolServiceLayer<'static>,
    spawner: Spawner,
) {
    spawner.must_spawn(mpsl_task(mpsl));
    // random static addr
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    let resources = {
        static RESOURCES: StaticCell<BleHostResources> = StaticCell::new();
        RESOURCES.init(BleHostResources::new())
    };
    let stack = {
        static STACK: StaticCell<Stack<'_, SoftdeviceController<'_>, DefaultPacketPool>> =
            StaticCell::new();
        STACK.init(trouble_host::new(sdc, resources).set_random_address(address))
    };
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();
    spawner.must_spawn(host_task(runner));

    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: BLE_NAME,
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .expect("Failied to run GAT server");
    loop {
        match advertise(&mut peripheral, &server).await {
            Ok(conn) => {
                select(gatt_events(&conn), env_notifier(&conn, &server)).await;
            }
            Err(e) => warn!("[adv-run] {:?}", e),
        }
    }
}

async fn gatt_events(conn: &GattConnection<'_, '_, DefaultPacketPool>) {
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                warn!("[conn] disconected with reason: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => match event.accept() {
                Ok(reply) => reply.send().await,
                Err(e) => warn!("[conn] processing failed: {:?}", e),
            },
            _ => (),
        }
    }
}

async fn advertise<'a, 'b, C: Controller>(
    periferal: &mut Peripheral<'a, C, DefaultPacketPool>,
    server: &'b Server<'_>,
) -> Result<GattConnection<'a, 'b, DefaultPacketPool>, BleHostError<C::Error>> {
    const GAP_ADV_LIMIT: usize = 31;
    let mut ad_data: [u8; GAP_ADV_LIMIT] = [0u8; GAP_ADV_LIMIT];
    let ad_len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids128(&[services::configuration::TCS.into()]),
            AdStructure::CompleteLocalName(BLE_NAME.as_bytes()),
        ],
        &mut ad_data[..],
    )?;
    let mut sr_data = [0u8; GAP_ADV_LIMIT];
    let sr_len = AdStructure::encode_slice(
        &[AdStructure::ManufacturerSpecificData {
            company_identifier: services::configuration::MSP_NORDIC_COMPANY_ID,
            payload: &services::configuration::MSP_PAYLOAD,
        }],
        &mut sr_data[..],
    )?;
    let advertiser = periferal
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &ad_data[0..ad_len],
                scan_data: &sr_data[0..sr_len],
            },
        )
        .await?;
    info!("[adv] Advertisement: waiting for connection");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("[adv] Advertisement: connection established");
    Ok(conn)
}
