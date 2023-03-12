#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::delay_ms;
use arduino_hal::delay_us;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins        = arduino_hal::pins!(peripherals);

    let mut led = pins.d13.into_output();

    // Axis 6 (Working, calibrated)
    // let mut pulse     = pins.d46.into_output();
    // let mut direction = pins.d48.into_output();
    // let mut enable    = pins.a8.into_output();
    // let step_delay: u32 = 200;
    
    // Axis 5
    let mut pulse     = pins.a0.into_output();
    let mut direction = pins.a1.into_output();
    let mut enable    = pins.d38.into_output();
    let step_delay: u32 = 1000;
    
    // Axis 4
    // let mut pulse     = pins.a6.into_output();
    // let mut direction = pins.a7.into_output();
    // let mut enable    = pins.a2.into_output();
    // let step_delay: u32 = 600;
    
    //enable.set_low();
    
    let mut main_enable  = pins.d32.into_output();
    
    // Axis 3
    // let mut pulse     = pins.d47.into_output();
    // let mut direction = pins.d45.into_output();
    
    // Axis 2
    // let mut pulse     = pins.d39.into_output();
    // let mut direction = pins.d37.into_output();
    
    // Axis 1
    // let mut pulse     = pins.d43.into_output();
    // let mut direction = pins.d41.into_output();
    
    main_enable.set_high();
    main_enable.set_low();
    
    // let step_delay: u32 = 4000;

    direction.set_low(); // positive direction
    // direction.set_high(); // negative direction
    enable.set_low();
                         
    let steps: u32      = 10000;

    led.set_high();

    for _x in 0..steps {
        pulse.set_high();
        led.set_high();
        delay_us(step_delay);
        pulse.set_low();
        led.set_low();
        delay_us(step_delay);
    }

    enable.set_high();
    main_enable.set_high();

    for _x in 0..1000 {
        led.set_high();
        delay_ms(100);
        led.set_low();
    }

    led.set_low();

    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}
