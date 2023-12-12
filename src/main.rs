#![no_std]
#![no_main]

use panic_halt as _;

mod morse;
// Text to Morse code LED flasher

#[arduino_hal::entry]

fn main() -> ! {
    let dp   = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();

    let out_string = "HELLO WORLD !";

    //\\ let mut s : &str;

    let mut delay;

// for ch       in out_string      do ;   = Pascal
 
   for ch in out_string.chars() {  

        //  let mut s = "";
        //  for ms in morse::MORSE_CODE {
        //      if   ch == ms.0  
        //          { s =  ms.1; } 

        //  }

        let dict 
               = morse::MORSE_CODE.iter()
                                  .find( | (mch, _) | { ch == *mch } );

        // if dict.is_some() {        
        //             for blip in dict.unwrap().1.chars() {
        //                 match blip {  
        //                             '.' => { delay = 200},
        //                             '-' => { delay = 500},
        //                             _   => { delay = 0}
        //                             }
                                    
        //                 led.set_high();  arduino_hal::delay_ms(delay);
        //                 led.set_low();   arduino_hal::delay_ms(delay);
        //             }
        // }

        if let Some(s) = dict {        
            for blip in s.1.chars() {
                match blip {  
                            '.' => { delay = 200},
                            '-' => { delay = 500},
                            _   => { delay = 0}
                            }
                            
                led.set_high();  arduino_hal::delay_ms(delay);
                led.set_low();   arduino_hal::delay_ms(delay);
            }
}        

    }

    //loop {}
    panic!()

}
