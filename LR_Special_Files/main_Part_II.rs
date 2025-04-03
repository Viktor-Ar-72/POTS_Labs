#![no_main]
#![no_std]

// Можно использовать writeln!() с UART (jy ;t COM-порт)
use core::fmt::Write;
// Точка входа для программы
use cortex_m_rt::entry;
// Подключение BBC Micro:bit V2, с UART
//use microbit::{self as _, hal::uarte::{Baudrate, Parity, Uarte}};
// Ещё одно подключение BBC Micro:bit V2
use microbit::Board;
// Импорт крейтов для работы с датчиком LSM303AGR
use lsm303agr::{Lsm303agr, interface::I2cInterface, mode::MagOneShot};
use linux_embedded_hal::{Delay, I2cdev};
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
// Подключение BBC Micro:bit V2, с UART + дополнительных модулей платы + Delay
use microbit::{self as _, hal::{Delay, uarte::{Baudrate, Parity, Uarte}, twim::Twim}, Board};

// Обработчик PanicInfo
#[panic_handler]
//fn painc_handler(_info: &core::panic::PanicInfo) -> ! {
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // write!(DEBUG_OUTPUT, "panicked: {}", panic_info.message());
    // write!(serial, "panicked: {}", panic_info.message());
    // Бесконечный цикл
    loop {}
}

// Точка входа в программу
#[entry]
fn main() -> ! {
    // 0) Получение доступа к плате
    let board = Board::take().unwrap();

    // 1) Настройка UART для вывода данных в терминал Minicom
    // Неплохой пример с объяснениями - https://docs.rust-embedded.org/discovery/microbit/07-uart/send-a-single-byte.html
    let mut serial = Uarte::new(
        // Аппаратный модуль - UART0
        board.UARTE0,
        // Активизация контактов (TX/RX) модуля
        board.uart.into(),
        // Отключение чётности (чтобы не занимать время проверкой того, повреждены пакеты при доставке или нет)
        Parity::EXCLUDED,
        // Установка скорости в 115200 бод (символов/секунду, это НЕ РАВНО (обычно) бит/сек)
        Baudrate::BAUD115200,
    );

    // 2) Настройка I2C (TWIM) для работы с LSM303AGR
    /* Почему используется отдельная настройка каждого контакта через SDA и SCL
    Если верить интернету, то это позволяет более точно контролировать I2C, на форуме даже было развёрнутое объяснение:
        1) Больше контроля, если есть несколько устройств на I2C;
        2) Чтобы можно было работать на разных версиях платы;
        3) Лучше читается код.
    Решил пока настроить так, там народ по крайней мере вроде как настроили плату
    */
    // Настройка контакта SCL (Clock)
    let scl = board.pins.p0_26.into_floating_input();
    // Настройка контакта SDA (Data)
    let sda = board.pins.p0_25.into_floating_input();
    // Инициализация I2C
    let i2c = Twim::new(
        // Аппаратный модуль I2C (TWIM)
        board.TWIM0,
        // Вариант Б - использовать контакты SDA и SCL
        sda, scl,
        // Вариант А - стандартная настройка I2C
        // board.i2c_internal.into(),
        // Частота 100 кГц (стандартная)
        microbit::hal::twim::Frequency::K100,
    );

    // 3) Инициализация датчика LSM303AGR - по сути,
    // компаса (3D-акселерометр, 3D-магнитометр) с ультранизким энергопотреблением
    // Создание объекта датчика (для соединения используется I2C, еще можно SPI)
    let mut imu = Lsm303agr::new_with_i2c(i2c);
    // Инициализация датчика
    imu.init().unwrap();
    // Установка режима магнитометра - однократное измерение магнитного поля
    imu.set_mag_mode(MagOneShot);

    // 4) Создание задержки (на основе системного таймера, через board.SYST) для более-менее плавных измерений
    let mut delay = Delay::new(board.SYST);

    // 5) Основной бесконечный цикл работы платы
    loop {
        // Ожидание идёт до тех пор, пока датчик не соберёт новые данные
        while !(imu.mag_status().unwrap().xyz_new_data)
        {
            // Пустой цикл, ничего не происходит
        }
        // Чтение данных с магнитометра
        let mag_data = imu.mag_data().unwrap();
        // Вывод собранных данных осей X, Y, Z в Minicom через UART
        writeln!(
            // Указание на UART
            serial,
            // Данные в виде строки
            "3D-Magnetometer, current data == X: {}, Y: {}, Z: {}",
            mag_data.x, mag_data.y, mag_data.z
        ).ok(); // ok() вызывается для дополнительного подтверждения успешной передачи. Если что - будет PANIC

        // 6) Задержка на 500 мс перед следующей итерацией цикла - чтобы успели передаться данные, а датчик мог уже что-нибудь измерить
        delay.delay_ms(500u16);
    }
}
