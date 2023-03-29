#![no_main]
#![no_std]

use panic_halt as _;
use arduino_hal as hw;

use hw::port::Pin;
use hw::port::mode::Output;
use hw::port::mode::Input;
use hw::port::PinOps;
use hw::port::mode::Io;

struct Axis<P,D> {
    pulse  : Pin<Output,P>,
    dir    : Pin<Output,D>,
    delay  : u32
}

fn make_axis<I,P,D>(pulse  : Pin<I, P>, 
                      dir    : Pin<I, D>, 
                      delay  : u32
             ) -> Axis<P,D> where I: Io, P: PinOps, D: PinOps
{ 
    Axis { pulse  : pulse .into_output(), 
           dir    : dir   .into_output(), 
           delay  : delay
    }
}

impl<P: PinOps, D: PinOps> Axis<P,D> {
    fn turn (&mut self, steps : u32, direction : bool) {
        if direction {
            self.dir.set_high();
        } else {
            self.dir.set_low();
        }
        for _x in 0..steps {
            self.pulse.set_high();
            hw::delay_us(self.delay);
            self.pulse.set_low();
            hw::delay_us(self.delay);
        }
        true;
    }
}



#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins        = arduino_hal::pins!(peripherals);

    let mut led = pins.d13.into_output();

    let mut main = pins.d32.into_output();

    let mut enable_6 = pins.a8.into_output();
    let mut enable_5 = pins.d38.into_output();
    let mut enable_4 = pins.a2.into_output();
    enable_6.set_low();
    enable_5.set_low();
    enable_4.set_low();

    // Pulse, direction, delay
    let mut ax1 = make_axis(pins.d43, pins.d41, 10000);
    let mut ax2 = make_axis(pins.d39, pins.d37, 10000);
    let mut ax3 = make_axis(pins.d47, pins.d45, 10000);
    let mut ax4 = make_axis(pins.a6,  pins.a7,  200);
    let mut ax5 = make_axis(pins.a0,  pins.a1,  200);
    let mut ax6 = make_axis(pins.d46, pins.d48, 200);

    led.set_high();
    main.set_low();
    main.set_high();
    //ax1.turn(1000, true);
    //ax1.turn(1000, false);
    // ax2.turn(1000, true);
    // ax2.turn(1000, false);
    // ax3.turn(1000, true);
    // ax3.turn(1000, false);

    main.set_high();

    // ax4.turn(6000, true);
    // ax4.turn(6000, false);
    // ax5.turn(6000, true);
    // ax5.turn(6000, false);
    // ax6.turn(6000, true);
    // ax6.turn(6000, false);
    led.set_low();

    enable_4.set_high();
    enable_5.set_high();
    enable_6.set_high();

    loop {
        led.toggle();
        hw::delay_ms(100);
    }
}
