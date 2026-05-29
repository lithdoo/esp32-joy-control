use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{PinDriver, Pull};
use esp_idf_svc::hal::peripherals::Peripherals;

const JOY_BUTTON_GPIO: i32 = 0;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let mut joy_button = PinDriver::input(peripherals.pins.gpio0)?;
    joy_button.set_pull(Pull::Up)?;

    log::info!("simple-joy-control started on ESP32");
    log::info!("Joystick button configured on GPIO {JOY_BUTTON_GPIO}");

    loop {
        let pressed = joy_button.is_low();
        log::info!("Joystick button pressed: {pressed}");
        FreeRtos::delay_ms(500);
    }
}
