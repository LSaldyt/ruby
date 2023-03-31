#![no_main]
#![no_std]

mod axis;
use axis::axis::make_axis;
use axis::axis::Enable;

use panic_halt as _;
use arduino_hal as hw;

// use embedded_hal::serial::Read;
// let b = nb::block!(serial.read()).unwrap();
// ufmt::uwriteln!(&mut serial, "Got {}", b).unwrap();

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = hw::Peripherals::take().unwrap();
    let pins        = hw::pins!(peripherals);

    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();


    let mut enable = Enable {
        led  : pins.d13.into_output(),
        main : pins.d32.into_output(),
        en4  : pins.a2 .into_output(),
        en5  : pins.d38.into_output(),
        en6  : pins.a8 .into_output()
    };
    enable.off(); // To make sure

    // make_axis() arguments: Pulse, direction, main, delay, resolution, gear_ratio
    let mut ax1 = make_axis(pins.d43, pins.d41, true, 4000,  8, 4.8); // 96/20 = 4.8
    let mut ax2 = make_axis(pins.d39, pins.d37, true, 4000,  8, 4.0); // ratio 4
    let mut ax3 = make_axis(pins.d47, pins.d45, true, 4000,  8, 5.0); // ratio 5
    let mut ax4 = make_axis(pins.a6,  pins.a7,  false, 200, 32, 2.8); // 56/20 = 2.8
    let mut ax5 = make_axis(pins.a0,  pins.a1,  false, 200, 32, 2.1); // 42/20 = 2.1
    let mut ax6 = make_axis(pins.d46, pins.d48, false, 200, 32, 1.0); // ratio 1

    ax6.rotate(&mut enable, 360.0);
    ax6.rotate(&mut enable, -360.0);
    ax5.rotate(&mut enable, 360.0);
    ax5.rotate(&mut enable, -360.0);
    ax3.rotate(&mut enable, -15.0);
    ax4.rotate(&mut enable, 90.0);
    ax4.rotate(&mut enable, -90.0);
    ax3.rotate(&mut enable, -90.0);
    ax3.rotate(&mut enable, 60.0);
    ax2.rotate(&mut enable, -30.0);
    ax2.rotate(&mut enable, 15.0);
    ax1.rotate(&mut enable, -15.0);
    ax1.rotate(&mut enable, 15.0);

    loop {
        enable.led.toggle();
        hw::delay_ms(100);
    }
}
