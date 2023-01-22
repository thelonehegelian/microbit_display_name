#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut light_it_all = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let CLEAN_BOARD = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let z = [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ];

    let o = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    let y = [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ];

    let a = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
    ];

    loop {
        for i in 0..5 {
            for j in 0..5 {
                // light_it_all[i][j - 1] = 0;
                light_it_all[i][j] = 1;
                display.show(&mut timer, light_it_all, 10);
            }
        }

        // clear the display
        display.clear();

        light_it_all = CLEAN_BOARD;
        // do an inverse loop
        for i in (0..5).rev() {
            for j in (0..5).rev() {
                light_it_all[i][j] = 1;
                display.show(&mut timer, light_it_all, 10);
            }
        }

        light_it_all = CLEAN_BOARD;

        /********************
         * DISPLAY THE NAME *
         ********************/
        display.show(&mut timer, z, 3000);
        display.show(&mut timer, o, 3000);
        display.show(&mut timer, y, 3000);
        display.show(&mut timer, a, 3000);
        
        for i in 0..5 {
            for j in 0..5 {
                // light_it_all[i][j - 1] = 0;
                light_it_all[i][j] = 1;
                display.show(&mut timer, light_it_all, 10);
            }
        }

        // clear the display
        display.clear();

        light_it_all = CLEAN_BOARD;
        // do an inverse loop
        for i in (0..5).rev() {
            for j in (0..5).rev() {
                light_it_all[i][j] = 1;
                display.show(&mut timer, light_it_all, 10);
            }
        }

        light_it_all = CLEAN_BOARD;
        timer.delay_ms(1000_u32);
    }
}
