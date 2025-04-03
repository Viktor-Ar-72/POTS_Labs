#![no_main]
#![no_std]


use core::fmt::Write;
use cortex_m_rt::entry;
use microbit::{self as _, hal::uarte::{Baudrate, Parity, Uarte}};
use microbit::Board;

#[panic_handler]
//fn painc_handler(_info: &core::panic::PanicInfo) -> ! {
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // write!(DEBUG_OUTPUT, "panicked: {}", panic_info.message());
    // write!(serial, "panicked: {}", panic_info.message());
    loop {}
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    writeln!(serial, "Hello from microbit!").ok();
    loop {}
}

