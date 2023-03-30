pub mod axis {

    use panic_halt as _;
    use arduino_hal as hw;

    use hw::port::Pins;
    use hw::port::Pin;
    use hw::port::mode::Output;
    // use hw::port::mode::Input; // I template param
    use hw::port::PinOps;
    use hw::port::mode::Io;
    use hw::port::PB7;
    use hw::port::PC5;
    use hw::port::PF2;
    use hw::port::PD7;
    use hw::port::PK0;

    pub struct Axis<P,D> {
        pulse  : Pin<Output,P>,
        dir    : Pin<Output,D>,
        large  : bool,
        delay  : u32
    }

    pub fn make_axis<I,P,D>(pulse  : Pin<I, P>, 
                        dir    : Pin<I, D>, 
                        large  : bool,
                        delay  : u32
                 ) -> Axis<P,D> where I: Io, P: PinOps, D: PinOps
    { 
        Axis { pulse  : pulse .into_output(), 
               dir    : dir   .into_output(), 
               large  : large,
               delay  : delay
        }
    }

    impl<P: PinOps, D: PinOps> Axis<P,D> {
        pub fn step (&mut self, steps : u32, direction : bool) {
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

    pub struct Enable<P1,P2,P3,P4,P5> {
        led  : Pin<Output, P1>,
        main : Pin<Output, P2>,
        en4  : Pin<Output, P3>,
        en5  : Pin<Output, P4>,
        en6  : Pin<Output, P5>
    }

    pub fn make_enable (pins : Pins) -> Enable <PB7, PC5, PF2, PD7, PK0>{
        Enable {
            led  : pins.d13.into_output(),
            main : pins.d32.into_output(),
            en4  : pins.a2.into_output(),
            en5  : pins.d38.into_output(),
            en6  : pins.a8.into_output()
        }
    }

}
