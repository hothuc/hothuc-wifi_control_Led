use std::sync::{Arc, Mutex};
use esp_idf_hal::gpio::{AnyOutputPin, PinDriver, Output};

pub type LedPin = Arc<Mutex<PinDriver<'static, AnyOutputPin, Output>>>;
pub type LedState = Arc<Mutex<bool>>;

pub fn init_led(pin: AnyOutputPin) -> (LedPin, LedState) {
    let led = Arc::new(Mutex::new(
        PinDriver::output(pin).unwrap(),
    ));
    let led_state = Arc::new(Mutex::new(true));
    (led, led_state)
}
