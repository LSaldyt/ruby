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
        pulse_pin  : Pin<Output,P>,
        dir_pin    : Pin<Output,D>,
        is_main    : bool,
        delay      : u32,
        resolution : u8,
        gear_ratio : f32,
        conversion : f32
    }

    pub fn make_axis<I,P,D>(pulse_pin  : Pin<I, P>, 
                            dir_pin    : Pin<I, D>, 
                            is_main    : bool,
                            delay      : u32,
                            resolution : u8,
                            gear_ratio : f32
                 ) -> Axis<P,D> where I: Io, P: PinOps, D: PinOps
    { 
        Axis { pulse_pin : pulse_pin .into_output(), 
               dir_pin   : dir_pin   .into_output(), 
               is_main,
               delay,
               resolution,
               gear_ratio,
               conversion : 360.0 / 200. / (resolution as f32) / gear_ratio
        }
    }

    impl<P: PinOps, D: PinOps> Axis<P,D> {
        pub fn step<P1,P2,P3,P4,P5> (&mut self, 
                                     enable    : &mut Enable<P1,P2,P3,P4,P5>, 
                                     steps     : u32, 
                                     direction : bool) 
        where P1: PinOps, P2: PinOps, P3: PinOps, P4: PinOps, P5: PinOps {
            set_pin(&mut self.dir_pin, direction);
            enable.set(true, self.is_main);
            for _x in 0..steps {
                self.pulse_pin.set_high();
                hw::delay_us(self.delay);
                self.pulse_pin.set_low();
                hw::delay_us(self.delay);
            }
            enable.set(false, self.is_main);
        }

        pub fn angle_to_steps(&mut self, angle : f32) -> (bool, u32) {
            let direction : bool = angle.is_sign_positive();
            let mut abs_angle : f32 = angle;
            if !direction {
                abs_angle = -1.0 * angle; // abs(), since no std:: available
            }
            return (direction, (abs_angle / self.conversion) as u32);
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
        pub fn set (&mut self, setting : bool, is_main : bool) {
            set_pin(&mut self.led, setting);
            if is_main {
                set_pin(&mut self.main, setting);
            } else {
                set_pin(&mut self.en4, setting);
                set_pin(&mut self.en5, setting);
                set_pin(&mut self.en6, setting);
            }
        }
    }
    
}
