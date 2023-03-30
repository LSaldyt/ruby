pub mod axis {

    use panic_halt as _;
    use arduino_hal as hw;

    use hw::port::Pin;
    use hw::port::mode::Output;
    // use hw::port::mode::Input; // I template param
    use hw::port::PinOps;
    use hw::port::mode::Io;

    pub struct Axis<P,D> {
        pulse  : Pin<Output,P>,
        dir    : Pin<Output,D>,
        delay  : u32
    }

    pub fn make_axis<I,P,D>(pulse  : Pin<I, P>, 
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
        pub fn turn (&mut self, steps : u32, direction : bool) {
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

}
