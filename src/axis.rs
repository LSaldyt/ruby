pub mod axis {

    use panic_halt as _;
    use arduino_hal as hw;

    use hw::port::Pin;
    use hw::port::mode::Output;
    use hw::port::PinOps;
    use hw::port::mode::Io;

    pub fn set_pin<P>(pin : &mut Pin<Output, P>, setting : bool) 
        where P : PinOps {
        if setting {
            pin.set_high();
        } else {
            pin.set_low();
        }
    }

    pub struct Axis<P,D> {
        pulse  : Pin<Output,P>,
        dir    : Pin<Output,D>,
        main   : bool,
        delay  : u32
    }

    pub fn make_axis<I,P,D>(pulse  : Pin<I, P>, 
                            dir    : Pin<I, D>, 
                            main   : bool,
                            delay  : u32
                 ) -> Axis<P,D> where I: Io, P: PinOps, D: PinOps
    { 
        Axis { pulse : pulse .into_output(), 
               dir   : dir   .into_output(), 
               main  : main,
               delay : delay
        }
    }

    impl<P: PinOps, D: PinOps> Axis<P,D> {
        pub fn step<P1,P2,P3,P4,P5> (&mut self, enable : &mut Enable<P1,P2,P3,P4,P5>, steps : u32, direction : bool) 
        where P1: PinOps, P2: PinOps, P3: PinOps, P4: PinOps, P5: PinOps{
            set_pin(&mut self.dir, direction);
            enable.set(true, self.main);
            for _x in 0..steps {
                self.pulse.set_high();
                hw::delay_us(self.delay);
                self.pulse.set_low();
                hw::delay_us(self.delay);
            }
            enable.set(false, self.main);
        }
    }

    pub struct Enable<P1,P2,P3,P4,P5> {
        pub led  : Pin<Output, P1>,
        pub main : Pin<Output, P2>,
        pub en4  : Pin<Output, P3>,
        pub en5  : Pin<Output, P4>,
        pub en6  : Pin<Output, P5>
    }

    impl <P1: PinOps, P2: PinOps, P3: PinOps, P4: PinOps, P5: PinOps> Enable<P1,P2,P3,P4,P5> {
        pub fn set (&mut self, setting : bool, main : bool) {
            set_pin(&mut self.led, setting);
            if main {
                set_pin(&mut self.main, setting);
            } else {
                set_pin(&mut self.en4, setting);
                set_pin(&mut self.en5, setting);
                set_pin(&mut self.en6, setting);
            }
        }

    }
    
}
