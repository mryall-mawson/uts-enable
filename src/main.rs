#[cfg(target_os = "linux")]
use gpio::GpioOut;

use log::info;

fn main() {
    let _ = env_logger::try_init(); // don't fail if called multiple times

    gpio_set_low(45);
    gpio_set_high(47);
    gpio_set_low(27);
}

#[cfg(target_os = "linux")]
pub fn gpio_set_low(pin: u16) {
    if let Ok(mut pin) = gpio::sysfs::SysFsGpioOutput::open(pin) {
        let _ = pin.set_low();
    } else {
        error!("Can't open pin for writing: {}", pin);
    }
}

#[cfg(target_os = "linux")]
pub fn gpio_set_high(pin: u16) {
    if let Ok(mut pin) = gpio::sysfs::SysFsGpioOutput::open(pin) {
        let _ = pin.set_high();
    } else {
        error!("Can't open pin for writing: {}", pin);
    }
}

#[cfg(not(target_os = "linux"))]
pub fn gpio_set_low(_pin: u16) {
    info!("GPIO operations only implemented for Linux");
}

#[cfg(not(target_os = "linux"))]
pub fn gpio_set_high(_pin: u16) {
    info!("GPIO operations only implemented for Linux");
}
