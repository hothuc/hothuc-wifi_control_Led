use std::sync::{Arc, Mutex};
use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpServerConfiguration, Method};
use esp_idf_hal::io::{Write, EspIOError};
use crate::led::LedPin;

const INDEX_HTML: &str = include_str!("../static/index.html");

pub fn init_web_server(
    led: LedPin,
    led_state: Arc<Mutex<bool>>,
) -> anyhow::Result<EspHttpServer<'static>> {
    let mut server = EspHttpServer::new(&HttpServerConfiguration::default())?;

    // Handler: index.html
    let state_for_index = led_state.clone();
    server.fn_handler("/", Method::Get, move |req| -> Result<(), EspIOError> {
        let state = *state_for_index.lock().unwrap();
        let status_text = if state { "ON" } else { "OFF" };
        let html = INDEX_HTML.replace("{{STATUS}}", status_text);

        let mut resp = req.into_ok_response()?;
        resp.write(html.as_bytes())?;
        Ok(())
    })?;

    // Handler: turn LED on
    let led_on = led.clone();
    let state_on = led_state.clone();
    server.fn_handler("/on", Method::Get, move |req| -> Result<(), EspIOError> {
        if let Err(e) = led_on.lock().unwrap().set_high() {
            println!("GPIO set_high error: {:?}", e);
        } else {
            *state_on.lock().unwrap() = true;
        }
        let mut resp = req.into_ok_response()?;
        resp.write(b"ON")?;
        Ok(())
    })?;

    // Handler: turn LED off
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

    // Favicon handler
    server.fn_handler("/favicon.ico", Method::Get, |req| {
        req.into_ok_response()?.write_all(b"")?;
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(server)
}
