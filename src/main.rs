use std::net::Ipv4Addr;
use std::time::Duration;

use anyhow::{bail, Result};
use embedded_svc::wifi::{self, Configuration, Wifi};
use esp_idf_hal::peripheral;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::netif::{EspNetif, EspNetifWait};
use esp_idf_svc::wifi::{EspWifi, WifiWait};
use log::*;
use matter::data_model::cluster_basic_information::BasicInfoConfig;
use matter::data_model::cluster_on_off::OnOffCluster;
use matter::data_model::device_types::DEV_TYPE_ON_OFF_LIGHT;
use matter::secure_channel::spake2p::VerifierData;
use matter::{core, CommissioningData};

mod dev_att;

const SSID: &str = "network";
const PASSWORD: &str = "password";

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    unsafe {
        esp_idf_sys::nvs_flash_init();
    }

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let _espwifi = start_wifi(peripherals.modem, sysloop.clone(), SSID, PASSWORD);
    info!("Wi-Fi started");

    // This is needed for async-io, otherwise we get error:
    // thread 'main' panicked at 'cannot initialize I/O event notification: Kind(Other)
    //https://matrix.to/#/!LdaNPfUfvefOLewEIM:matrix.org/$5V1776lebeE-UMXKFMdej304EGZsXGc7MLecpuAuoYY?via=matrix.org&via=tchncs.de&via=matrix.coredump.ch
    esp_idf_sys::esp!(unsafe {
        esp_idf_sys::esp_vfs_eventfd_register(&esp_idf_sys::esp_vfs_eventfd_config_t {
            max_fds: 5,
            ..Default::default()
        })
    })
    .unwrap();

    let comm_data = CommissioningData {
        // TODO: Hard-coded for now
        verifier: VerifierData::new_with_pw(123456),
        discriminator: 250,
    };

    // vid/pid should match those in the DAC
    let dev_info = BasicInfoConfig {
        vid: 0xFFF1,
        pid: 0x0011,
        hw_ver: 1,
        sw_ver: 1,
        sw_ver_str: "1".to_string(),
        serial_no: "abcdef00".to_string(),
        device_name: "Matter Device".to_string(),
    };

    let dev_att = Box::new(dev_att::HardCodedDevAtt::new());

    let mut matter =
        core::Matter::new(dev_info, dev_att, comm_data).expect("Unable to start matter");
    let dm = matter.get_data_model();
    {
        info!("Got data model");
        let mut node = dm.node.write().unwrap();
        let endpoint = node.add_endpoint(DEV_TYPE_ON_OFF_LIGHT).unwrap();
        let mut cluster_onoff = OnOffCluster::new().unwrap();
        let callback_on = Box::new(|| info!("Device is on"));
        let callback_off = Box::new(|| info!("Device is off"));
        cluster_onoff.add_callback(
            matter::data_model::cluster_on_off::Commands::On,
            callback_on,
        );
        cluster_onoff.add_callback(
            matter::data_model::cluster_on_off::Commands::Off,
            callback_off,
        );
        node.add_cluster(endpoint, cluster_onoff).unwrap();
    }

    println!("free memory: {}", unsafe {
        esp_idf_sys::esp_get_free_heap_size()
    });
    matter.start_daemon().unwrap();

    Ok(())
}

/// Start wifi
///
/// TODO: This presumes that the wifi SSID is already known.
/// The correct approach would be to enter provisioning mode,
/// and then only start wifi after provisioning.
fn start_wifi(
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
    ssid: &str,
    password: &str,
) -> Result<Box<EspWifi<'static>>> {
    let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), None)?);
    let config = wifi::ClientConfiguration {
        ssid: ssid.into(),
        password: password.into(),
        channel: None, // TODO
        ..Default::default()
    };
    wifi.set_configuration(&Configuration::Client(config))?;

    wifi.start()?;

    info!("Starting wifi...");

    if !WifiWait::new(&sysloop)?
        .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
    {
        bail!("Wifi did not start");
    }

    info!("Connecting wifi...");

    wifi.connect()?;

    if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
        Duration::from_secs(20),
        || {
            wifi.is_connected().unwrap()
                && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
        },
    ) {
        bail!("Wifi did not connect or did not receive a DHCP lease");
    }

    let ip_info = wifi.sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(wifi)
}
