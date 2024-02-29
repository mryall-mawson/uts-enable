use std::{fs, io};
use std::io::Write;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Input,
    Output,
}

fn main() {
    gpio_set_low(45);
    gpio_set_high(47);
    gpio_set_low(27);
}

fn export_gpio_if_unexported(gpio_num: u16) -> io::Result<()> {
    // export port first if not exported
    let file = format!("/sys/class/gpio/gpio{}", gpio_num);
    println!("checking for file existence: {}", file);
    if let Err(_) = fs::metadata(&file) {
        println!("pin file not found: {}", file);
        let mut export_fp = fs::File::create("/sys/class/gpio/export")?;
        println!("created export file");
        write!(export_fp, "{}", gpio_num)?;
        println!("wrote to export file: {}", gpio_num);
    } else {
        println!("pin file was found: {}", file);
    }

    // ensure we're using '0' as low
    let mut low_file = fs::File::create(format!("/sys/class/gpio/gpio{}/active_low", gpio_num))?;
    println!("opened active_low file for write: {}", gpio_num);
    low_file.write_all(b"0")?;
    println!("wrote active_low: {}", gpio_num);
    Ok(())
}

#[derive(Debug)]
struct Gpio {
    gpio_num: u16,
    sysfp: fs::File,
}

impl Gpio {
    fn set_low(&mut self) -> io::Result<()> {
        println!("Writing 0 to set pin low: {}", self.gpio_num);
        self.sysfp.write_all(b"0")
    }

    fn set_high(&mut self) -> io::Result<()> {
        println!("Writing 1 to set pin high: {}", self.gpio_num);
        self.sysfp.write_all(b"1")
    }
}

fn open(gpio_num: u16, direction: Direction) -> io::Result<Gpio> {
    println!("exporting gpio if unexported: {}", gpio_num);
    export_gpio_if_unexported(gpio_num)?;

    println!("setting gpio direction: {}", gpio_num);
    set_gpio_direction(gpio_num, direction)?;

    println!("finally, we can open the device: {}", gpio_num);
    Ok(Gpio {
        gpio_num,
        sysfp: open_gpio(gpio_num, direction)?,
    })
}

fn open_gpio(gpio_num: u16, direction: Direction) -> io::Result<fs::File> {
    let p = format!("/sys/class/gpio/gpio{}/value", gpio_num);

    match direction {
        Direction::Input => fs::File::open(p),
        Direction::Output => fs::File::create(p),
    }
}

fn set_gpio_direction(gpio_num: u16, direction: Direction) -> io::Result<()> {
    let mut file = fs::File::create(format!("/sys/class/gpio/gpio{}/direction", gpio_num))?;
    println!("open direction file to set: {}", gpio_num);
    file.write_all(
        match direction {
            Direction::Input => b"in",
            Direction::Output => b"out",
        },
    )
}

pub fn gpio_set_low(pin: u16) {
    if let Ok(mut pin) = open(pin, Direction::Output) {
        let _ = pin.set_low();
    } else {
        println!("Can't open pin for writing: {}", pin);
    }
}

#[cfg(target_os = "linux")]
pub fn gpio_set_high(pin: u16) {
    if let Ok(mut pin) = open(pin, Direction::Output) {
        let _ = pin.set_high();
    } else {
        println!("Can't open pin for writing: {}", pin);
    }
}
