use anyhow::Result;
use std::sync::{Arc, Mutex};

use esp_idf_sys as _; // Link to ESP-IDF
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::wifi::{EspWifi, AccessPointConfiguration, Configuration, WifiEvent, AuthMethod};
use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpServerConfiguration, Method};
use esp_idf_hal::io::Write;
use heapless::String as HString;
use esp_idf_hal::io::EspIOError;
use esp_idf_svc::eventloop::EspSystemEventLoop;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take()?;
    let led = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio12)?));
    let led_state = Arc::new(Mutex::new(true));
     let sysloop = EspSystemEventLoop::take()?;

    let led_event = led.clone();
    // Đăng ký sự kiện Wi-Fi
    let _sub = sysloop.subscribe::<WifiEvent, _>(move|event| {
        match event {
            WifiEvent::ApStaConnected(_info) => {
                println!("Kết nối mới!");
               let _ = led_event.lock().unwrap().set_high();
            }
            WifiEvent::ApStaDisconnected(_info) => {
                println!("Thiết bị rời đi!");
                let _ = led_event.lock().unwrap().set_low();
            }
            _ => {}
        }
    })?;

    // Tạo AP Wi-Fi
   
    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), Default::default())?;
    wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: HString::<32>::try_from("ESP32_AP").unwrap(),
        password: HString::<64>::try_from("12345678").unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        channel: 1,
        ..Default::default()
    }))?;
    wifi.start()?;

    println!("AP mode started. SSID: ESP32_AP, Password: 12345678");

    // Web server điều khiển LED

    let mut server = EspHttpServer::new(&HttpServerConfiguration::default())?;
    let state_for_index = led_state.clone();
    server.fn_handler("/", Method::Get, move |req| -> Result<(), EspIOError> {
        let state = *state_for_index.lock().unwrap();
        let status_text = if state { "ON" } else { "OFF" };

        // Very small, simple single page app using fetch() so page never fully reloads
        let html = format!(
r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8"/>
  <title>ESP32 LED Control</title>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <style>
    body {{ font-family: Arial, sans-serif; text-align:center; padding:1rem; }}
    button {{ padding: 0.6rem 1.2rem; font-size:1rem; margin:0.5rem; }}
    #status {{ margin-top:1rem; font-weight:bold; }}
  </style>
</head>
<body>
  <h1>ESP32 LED Control</h1>
  <div>
    <button onclick="ledOn()">Turn ON</button>
    <button onclick="ledOff()">Turn OFF</button>
  </div>
  <p id="status">LED status: {}</p>

  <script>
    async function ledOn() {{
      try {{
        const r = await fetch('/on', {{ method: 'GET' }});
        const t = await r.text();
        document.getElementById('status').textContent = 'LED status: ' + t;
      }} catch(e) {{ console.error(e); }}
    }}
    async function ledOff() {{
      try {{
        const r = await fetch('/off', {{ method: 'GET' }});
        const t = await r.text();
        document.getElementById('status').textContent = 'LED status: ' + t;
      }} catch(e) {{ console.error(e); }}
    }}
    // Optionally: poll status every 5s (comment out if not wanted)
    // setInterval(async () => {{
    //   const r = await fetch('/status'); const t = await r.text();
    //   document.getElementById('status').textContent = 'LED status: ' + t;
    // }}, 5000);
  </script>
</body>
</html>"#, status_text);

        let mut resp = req.into_ok_response()?;
        resp.write(html.as_bytes())?;
        Ok(())
    })?;

    // Handler: turn LED on -> return "ON"
    let led_on = led.clone();
    let state_on = led_state.clone();
    server.fn_handler("/on", Method::Get, move |req| -> Result<(), EspIOError> {
        // try to set pin high; if it errors we log but still return something
        if let Err(e) = led_on.lock().unwrap().set_high() {
            println!("GPIO set_high error: {:?}", e);
        } else {
            *state_on.lock().unwrap() = true;
        }
        let mut resp = req.into_ok_response()?;
        resp.write(b"ON")?;
        Ok(())
    })?;

    // Handler: turn LED off -> return "OFF"
    let led_off = led.clone();
    let state_off = led_state.clone();
    server.fn_handler("/off", Method::Get, move |req| -> Result<(), EspIOError> {
        if let Err(e) = led_off.lock().unwrap().set_low() {
            println!("GPIO set_low error: {:?}", e);
        } else {
            *state_off.lock().unwrap() = false;
        }
        let mut resp = req.into_ok_response()?;
        resp.write(b"OFF")?;
        Ok(())
    })?;

    // Small favicon handler to avoid 404 noise from browsers
    server.fn_handler("/favicon.ico", Method::Get, |req| {
        req.into_ok_response()?.write_all(b"")?;
        Ok::<(), anyhow::Error>(())
    })?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
