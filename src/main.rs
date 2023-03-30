#![no_main]
#![no_std]

mod axis;
use axis::axis::make_axis;
use axis::axis::Enable;

use panic_halt as _;
use arduino_hal as hw;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = hw::Peripherals::take().unwrap();
    let pins        = hw::pins!(peripherals);

    let mut enable = Enable {
        led  : pins.d13.into_output(),
        main : pins.d32.into_output(),
        en4  : pins.a2 .into_output(),
        en5  : pins.d38.into_output(),
        en6  : pins.a8 .into_output()
    };

    // Pulse, direction, main, delay
    let mut ax1 = make_axis(pins.d43, pins.d41, true, 4000);
    let mut ax2 = make_axis(pins.d39, pins.d37, true, 4000);
    let mut ax3 = make_axis(pins.d47, pins.d45, true, 4000);
    let mut ax4 = make_axis(pins.a6,  pins.a7,  false, 200);
    let mut ax5 = make_axis(pins.a0,  pins.a1,  false, 200);
    let mut ax6 = make_axis(pins.d46, pins.d48, false, 200);

    ax1.step(&mut enable, 500, true);
    ax1.step(&mut enable, 500, false);
    ax2.step(&mut enable, 1000, false);
    ax2.step(&mut enable, 1000, true);
    ax3.step(&mut enable, 1000, false);
    ax3.step(&mut enable, 1000, true);

    ax4.step(&mut enable, 6000, true);
    ax4.step(&mut enable, 6000, false);
    ax5.step(&mut enable, 6000, true);
    ax5.step(&mut enable, 6000, false);
    ax6.step(&mut enable, 6000, true);
    ax6.step(&mut enable, 6000, false);

    loop {
        enable.led.toggle();
        hw::delay_ms(100);
    }
}
