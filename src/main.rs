#[cfg(target_os = "linux")]
use gpio::GpioOut;

fn main() {
    gpio_set_low(45);
    gpio_set_high(47);
    gpio_set_low(27);
}

#[cfg(target_os = "linux")]
pub fn gpio_set_low(pin: u16) {
    if let Ok(mut pin) = gpio::sysfs::SysFsGpioOutput::open(pin) {
        let _ = pin.set_low();
    } else {
        println!("Can't open pin for writing: {}", pin);
    }
}

#[cfg(target_os = "linux")]
pub fn gpio_set_high(pin: u16) {
    if let Ok(mut pin) = gpio::sysfs::SysFsGpioOutput::open(pin) {
        let _ = pin.set_high();
    } else {
        println!("Can't open pin for writing: {}", pin);
    }
}

#[cfg(not(target_os = "linux"))]
pub fn gpio_set_low(_pin: u16) {
    println!("GPIO operations only implemented for Linux");
}

#[cfg(not(target_os = "linux"))]
pub fn gpio_set_high(_pin: u16) {
    println!("GPIO operations only implemented for Linux");
}
