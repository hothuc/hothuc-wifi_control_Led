// src/wifi.rs
use esp_idf_svc::wifi::{EspWifi, AccessPointConfiguration, Configuration, WifiEvent, AuthMethod};
use esp_idf_svc::eventloop::{EspSubscription, EspSystemEventLoop, System};
use heapless::String as HString;
use crate::led::LedPin;

pub fn init_wifi_ap(
    modem: esp_idf_hal::modem::Modem,
    sysloop: EspSystemEventLoop,
    led: LedPin,
) -> anyhow::Result<(EspWifi<'static>, EspSubscription<'static, System>)>
 {
    let led_event = led.clone();

    let sub = sysloop.subscribe::<WifiEvent, _>(move |event| {
        match event {
            WifiEvent::ApStaConnected(_) => {
                println!("Kết nối mới!");
                let _ = led_event.lock().unwrap().set_high();
            }
            WifiEvent::ApStaDisconnected(_) => {
                println!("Thiết bị rời đi!");
                let _ = led_event.lock().unwrap().set_low();
            }
            _ => {}
        }
    })?;

    let mut wifi = EspWifi::new(modem, sysloop.clone(), Default::default())?;
    wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: HString::<32>::try_from("ESP32_AP").unwrap(),
        password: HString::<64>::try_from("12345678").unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        channel: 1,
        ..Default::default()
    }))?;
    wifi.start()?;

    println!("AP mode started. SSID: ESP32_AP, Password: 12345678");

    Ok((wifi, sub))
}
