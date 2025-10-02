use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// GPIO uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

fn main() {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(GPIO_LED).unwrap().into_output();

    loop {
        // Blink the LED by setting the pin's logic level high for a second.
        pin.set_high();
        thread::sleep(Duration::from_millis(1000));

        pin.set_low();
        thread::sleep(Duration::from_millis(1000));
    }
}
