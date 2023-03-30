#![no_main]
#![no_std]

mod axis;
use axis::axis::make_axis;
use axis::axis::make_enable;

use panic_halt as _;
use arduino_hal as hw;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = hw::Peripherals::take().unwrap();
    let pins        = hw::pins!(peripherals);

    let mut enable = make_enable(pins);

    let mut led = pins.d13.into_output();
    let mut main = pins.d32.into_output();
    let mut enable_6 = pins.a8.into_output();
    let mut enable_5 = pins.d38.into_output();
    let mut enable_4 = pins.a2.into_output();
    enable_4.set_high();
    enable_5.set_high();
    enable_6.set_high();

    // Pulse, direction, delay
    let mut ax1 = make_axis(pins.d43, pins.d41, true, 4000);
    let mut ax2 = make_axis(pins.d39, pins.d37, true, 4000);
    let mut ax3 = make_axis(pins.d47, pins.d45, true, 4000);
    let mut ax4 = make_axis(pins.a6,  pins.a7,  false, 200);
    let mut ax5 = make_axis(pins.a0,  pins.a1,  false, 200);
    let mut ax6 = make_axis(pins.d46, pins.d48, false, 200);

    main.set_low();

    led.set_high();
    ax1.step(500, true);
    ax1.step(500, false);
    ax2.step(1000, false);
    ax2.step(1000, true);
    ax3.step(1000, false);
    ax3.step(1000, true);

    enable_6.set_low();
    enable_5.set_low();
    enable_4.set_low();

    ax4.step(6000, true);
    ax4.step(6000, false);
    ax5.step(6000, true);
    ax5.step(6000, false);
    ax6.step(6000, true);
    ax6.step(6000, false);
    led.set_low();

    enable_4.set_high();
    enable_5.set_high();
    enable_6.set_high();
    main.set_high();

    loop {
        led.toggle();
        hw::delay_ms(100);
    }
}
