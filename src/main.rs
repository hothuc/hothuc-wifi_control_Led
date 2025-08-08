use esp_idf_sys as _; // Link to ESP-IDF
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::wifi::{EspWifi, AccessPointConfiguration, Configuration};
use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpServerConfiguration, Method};
use esp_idf_hal::io::Write;
use heapless::String as HString;
use esp_idf_hal::io::EspIOError;


fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take()?;
    use std::sync::{Arc, Mutex};
    let led = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio12)?));

    // Tạo AP Wi-Fi
    let sysloop = esp_idf_svc::eventloop::EspSystemEventLoop::take()?;
    let mut wifi = EspWifi::new(peripherals.modem, sysloop, Default::default())?;
    wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: HString::<32>::try_from("ESP32_AP").unwrap(),
        password: HString::<64>::try_from("12345678").unwrap(),
        channel: 1,
        ..Default::default()
    }))?;
    wifi.start()?;

    println!("AP mode started. SSID: ESP32_AP, Password: 12345678");

    // Web server điều khiển LED
    use esp_idf_sys::EspError; // thêm dòng này

    let mut server = EspHttpServer::new(&HttpServerConfiguration::default())?;

    server.fn_handler("/", Method::Get, |req| -> Result<(), EspIOError> {
        let html = r#"
            <html>
                <body>
                    <h1>ESP32 LED Control</h1>
                    <a href="/led/on">Turn ON</a>
                    <a href="/led/off">Turn OFF</a>
                </body>
            </html>
        "#;
        req.into_ok_response()?.write_all(html.as_bytes())?;
        Ok(())
    })?;

    let led_on = led.clone();
    server.fn_handler("/led/on", Method::Get, move |req| -> Result<(), EspIOError> {
        let mut led = led_on.lock().unwrap();
        led.set_high()?;
        req.into_ok_response()?.write(b"LED ON")?;
        Ok(())
    })?;

    let led_off = led.clone();
    server.fn_handler("/led/off", Method::Get, move |req| -> Result<(), EspIOError> {
        let mut led = led_off.lock().unwrap();
        led.set_low()?;
        req.into_ok_response()?.write(b"LED OFF")?;
        Ok(())
    })?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
