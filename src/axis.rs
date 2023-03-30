pub mod axis {

    use panic_halt as _;
    use arduino_hal as hw;

    use hw::port::Pin;
    use hw::port::mode::Output;
    use hw::port::PinOps;
    use hw::port::mode::Io;

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
        pub fn step<P1,P2,P3,P4,P5> (&mut self, enable : &mut Enable<P1,P2,P3,P4,P5>, steps : u32, direction : bool) 
        where P1: PinOps, P2: PinOps, P3: PinOps, P4: PinOps, P5: PinOps{
            if direction {
                self.dir.set_high();
            } else {
                self.dir.set_low();
            }
            if self.large {
                enable.main.set_low();
            } else {
                enable.en4.set_low();
                enable.en5.set_low();
                enable.en6.set_low();
            }
            enable.led.set_high();
            for _x in 0..steps {
                self.pulse.set_high();
                hw::delay_us(self.delay);
                self.pulse.set_low();
                hw::delay_us(self.delay);
            }
            if self.large {
                enable.main.set_high();
            } else {
                enable.en4.set_high();
                enable.en5.set_high();
                enable.en6.set_high();
            }
            enable.led.set_low();
            true;
        }
    }

    pub struct Enable<P1,P2,P3,P4,P5> {
        pub led  : Pin<Output, P1>,
        pub main : Pin<Output, P2>,
        pub en4  : Pin<Output, P3>,
        pub en5  : Pin<Output, P4>,
        pub en6  : Pin<Output, P5>
    }
}
