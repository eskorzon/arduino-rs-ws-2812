#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
use panic_halt as _;

use arduino_hal::{self, spi};
use ws2812_spi as ws2812;

use crate::ws2812::Ws2812;
use smart_leds::{SmartLedsWrite, RGB8};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let (spi, _) = spi::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    const N_LED: usize = 11;
    const DELAY_MS: u16 = 100;
    const LED_SPACING: usize = 11;
    const ON_COLOR: RGB8 = RGB8 {r: 0, g: 1, b: 1};

    let mut data: [RGB8; N_LED] = [RGB8::default(); N_LED];
    let mut ws = Ws2812::new(spi);
    loop {
        // idea is to iterate to move lights through box in a cycle: [100] -> [010] -> [001]
        for k in 0..LED_SPACING {   // iterate through each led in the box
            for i in 0..N_LED {
                let color: RGB8 = match i % LED_SPACING == k {
                    true => ON_COLOR,
                    _ => RGB8::default()
                };
                data[i] = color;
            }
            ws.write(data.iter().cloned()).unwrap();
            arduino_hal::delay_ms(DELAY_MS);
        }

        // reverse direction at end of array
        for k in (0..LED_SPACING).rev() {   // iterate through each led in the box in reverse
            for i in 0..N_LED {
                let color: RGB8 = match i % LED_SPACING == k {
                    true => ON_COLOR,
                    _ => RGB8::default()
                };
                data[i] = color;
            }
            ws.write(data.iter().cloned()).unwrap();
            arduino_hal::delay_ms(DELAY_MS);
        }
    }
}