#![no_std]
#![no_main]

mod axis;
use axis::axis::make_axis;
use axis::axis::Enable;

use panic_halt as _;
use arduino_hal as hw;

use embedded_hal::serial::Read;
// let b = nb::block!(serial.read()).unwrap();
// ufmt::uwriteln!(&mut serial, "Got {}", b).unwrap();

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = hw::Peripherals::take().unwrap();
    let pins        = hw::pins!(peripherals);

    let mut serial = hw::default_serial!(peripherals, pins, 921600);
    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    let mut enable = Enable {
        led  : pins.d13.into_output(),
        main : pins.d32.into_output(),
        en4  : pins.a2 .into_output(),
        en5  : pins.d38.into_output(),
        en6  : pins.a8 .into_output()
    };
    enable.off(); // To make sure

    // // make_axis() arguments: Pulse, direction, main, delay, resolution, gear_ratio
    let mut ax1 = make_axis(pins.d43, pins.d41, true, 4000,  8, 4.8); // 96/20 = 4.8
    let mut ax2 = make_axis(pins.d39, pins.d37, true, 4000,  8, 4.0); // ratio 4
    let mut ax3 = make_axis(pins.d47, pins.d45, true, 4000,  8, 5.0); // ratio 5
    let mut ax4 = make_axis(pins.a6,  pins.a7,  false, 200, 32, 2.8); // 56/20 = 2.8
    let mut ax5 = make_axis(pins.a0,  pins.a1,  false, 200, 32, 2.1); // 42/20 = 2.1
    let mut ax6 = make_axis(pins.d46, pins.d48, false, 200, 32, 1.0); // ratio 1

    // ax6.rotate(&mut enable, 360.0);
    // ax6.rotate(&mut enable, -360.0);
    // ax5.rotate(&mut enable, 360.0);
    // ax5.rotate(&mut enable, -360.0);
    // ax3.rotate(&mut enable, -15.0);
    // ax4.rotate(&mut enable, 90.0);
    // ax4.rotate(&mut enable, -90.0);
    // ax3.rotate(&mut enable, -90.0);
    // ax3.rotate(&mut enable, 60.0);
    // ax2.rotate(&mut enable, -30.0);
    // ax2.rotate(&mut enable, 15.0);
    // ax1.rotate(&mut enable, -15.0);
    // ax1.rotate(&mut enable, 15.0);

    let mut i : usize = 0;

    const CAPACITY : usize = 512;
    let mut comm_buffer : [u8; CAPACITY] = [0; CAPACITY];

    loop {
        enable.led.toggle();

        let b = nb::block!(serial.read()).unwrap();
        if b == 10 && i == 5 { // Newline character
            if i != 5 {
                ufmt::uwriteln!(&mut serial, "Invalid number of bytes received!!").unwrap();
                // panic!("Ahhhhhh");
                i = 0;
                continue;
            }
            ufmt::uwriteln!(&mut serial, "parsing..").unwrap();
            let axis_index = comm_buffer.get(0).expect("Need axis index");
            match comm_buffer[1..i].try_into() {
                Ok(sub_buff) => {
                    let rotation = f32::from_le_bytes(sub_buff);
                    ufmt::uwriteln!(&mut serial, "Parsed float!").unwrap();
                    match axis_index {
                        1 => { ax1.rotate(&mut enable, rotation); }
                        2 => { ax2.rotate(&mut enable, rotation); }
                        3 => { ax3.rotate(&mut enable, rotation); }
                        4 => { ax4.rotate(&mut enable, rotation); }
                        5 => { ax5.rotate(&mut enable, rotation); }
                        6 => { ax6.rotate(&mut enable, rotation); }
                        _ => { ufmt::uwriteln!(&mut serial, "Invalid axis index!").unwrap(); }
                    }

                }
                Err(_) => {
                    ufmt::uwriteln!(&mut serial, "Error parsing float!").unwrap();
                }
            }
            ufmt::uwriteln!(&mut serial, "{}", axis_index).unwrap();
            i = 0; // Reset position
        } else {
            comm_buffer[i] = b;
            i += 1;
            if i > 5 {
                ufmt::uwriteln!(&mut serial, "too many bytes have been read..").unwrap();
            }
            if i == 512 {
                ufmt::uwriteln!(&mut serial, "fuck! we're outta space man").unwrap();
                panic!("fuck! we're outta space man");
            }
        }
    }
}
