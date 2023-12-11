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

    let out_string = "Hello World !";


// for ch       in out_string      do ;   = Pascal
 
   for ch in out_string.chars() {  

         let _ms = morse::MORSE_CODE[0];    
         let _c          = ch;

         for ms in morse::MORSE_CODE {
             if ch == ms.0 {
                
             };
         }

         {
            led.toggle();
            arduino_hal::delay_ms(500);
         }
    }

    led.set_low();

    //loop {}
    panic!()

}
