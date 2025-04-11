#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use microbit::{self as _, hal::{delay::Delay, twim::Twim}, hal::uarte::{Baudrate, Parity, Uarte}};
use microbit::Board;

use lsm303agr::Lsm303agr;
use microbit::pac::twim0::frequency::FREQUENCY_A;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // PANIC!
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
    // Twim inizialisation
    let twim0 = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    // Create sensor 
    let mut sensor = Lsm303agr::new_with_i2c(twim0);
    sensor.init().unwrap();
    // For delay
    let mut delay = Delay::new(board.SYST);
    // Get sensor data
    sensor.set_mag_mode_and_odr( &mut delay, lsm303agr::MagMode::HighResolution, lsm303agr::MagOutputDataRate::Hz10,).unwrap();

    let Ok(mut sensor) = sensor.into_mag_continuous() else {
        panic!("Error enabling continuous mode");
    };

    loop {
        //writeln!(serial, "New iteration of Loop");
        if sensor.mag_status().unwrap().xyz_new_data() {
            //writeln!(serial, "Found new data!");
            let data = sensor.magnetic_field().unwrap();
            writeln!( serial, "Magnetic-3D field: x {} y {} z {}", data.x_nt(), data.y_nt(), data.z_nt()).ok();
            }
    }
}
