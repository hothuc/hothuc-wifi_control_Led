use anyhow::Result;
use std::thread;
use std::time::Duration;
use esp_idf_sys as _; // Link to ESP-IDF
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::AnyOutputPin;
use esp_idf_svc::eventloop::EspSystemEventLoop;

mod led;
mod wifi;
mod web;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    // Lấy GPIO12 và chuyển sang AnyOutputPin
    let gpio12: AnyOutputPin = peripherals.pins.gpio12.into();
    let (led_pin, led_state) = led::init_led(gpio12);

    // Khởi động Wi-Fi AP mode
    let _wifi = wifi::init_wifi_ap(peripherals.modem, sysloop.clone(), led_pin.clone())?;

    // Khởi động web server
    let _server = web::init_web_server(led_pin.clone(), led_state.clone())?;

    // Vòng lặp chính
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
