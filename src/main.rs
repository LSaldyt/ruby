#![no_main]
#![no_std]

use panic_halt as _;
use arduino_hal as hw;

use hw::port::Pin;
use hw::port::mode::Output;
use hw::port::mode::Input;
use hw::port::PinOps;
use hw::port::mode::Io;

struct Axis<P,D,E> {
    pulse  : Pin<Output,P>,
    dir    : Pin<Output,D>,
    enable : Pin<Output,E>,
    delay  : u32
}

fn make_axis<I,P,D,E>(pulse  : Pin<I, P>, 
                      dir    : Pin<I, D>, 
                      enable : Pin<I, E>, 
                      delay  : u32
             ) -> Axis<P,D,E> where I: Io, P: PinOps, D: PinOps, E: PinOps { 
    Axis { pulse  : pulse.into_output(), 
           dir    : dir.into_output(), 
           enable : enable.into_output(),
           delay  : delay
    }
}

impl<P: PinOps, D: PinOps, E: PinOps> Axis<P,D,E> {
    fn turn (&mut self, steps : u32, direction : bool) {
        self.enable.set_low();
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
        self.enable.set_high();
        true;
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins        = arduino_hal::pins!(peripherals);

    let mut led = pins.d13.into_output();

    let mut ax6 = Axis{
        pulse:  pins.d46.into_output(),
        dir:    pins.d48.into_output(),
        enable: pins.a8.into_output(),
        delay:  200
    };

    // let mut ax1 = make_axis(pins.d43, pins.d41, pins.d32, 4000);
    // let mut ax2 = make_axis(pins.d39, pins.d37, pins.d32, 4000);
    // let mut ax3 = make_axis(pins.d47, pins.d45, pins.d32, 4000);
    // let mut ax4 = make_axis(pins.a6,  pins.a7,  pins.a2,  600);
    // let mut ax5 = make_axis(pins.a0,  pins.a1,  pins.d38, 1000);
    // let mut ax6 = make_axis(pins.d46, pins.d48, pins.a8,  200);

    led.set_high();
    ax6.turn(10000, true);
    // ax6.turn(6000, false);
    led.set_low();

    loop {
        led.toggle();
        hw::delay_ms(100);
    }
}