/* DEBUG
pub trait Logger {
    /// Логирует сообщение указанного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Логировать сообщения только заданного уровня.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

// TODO: Реализовать типаж`Logger` для `VerbosityFilter`.
impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        /// Вариант А, где логирование в else
        //if verbosity > self.max_verbosity {
            // Вывод сообщения об ошибке
            //eprintln!("Сообщение нельзя публиковать, так как многословность сообщения {verbosity} \
            //больше установленного максимума {}", self.max_verbosity)
        //}
        //else {
        //   //println!("{}", message)
        //    self.inner.log(verbosity, message)
        //}
        //
        /// Вариант Б, где логирование в true
        // Если многословность не превышает заданный уровень
        if verbosity <= self.max_verbosity {
        // Производится логирование через StderrLogger
            self.inner.log(verbosity, message)
        }
        else {
            // Вывод сообщения об ошибке
            eprintln!("Сообщение нельзя публиковать, так как многословность сообщения {verbosity} \
            больше установленного максимума {}", self.max_verbosity)
        }
    }
}

fn main() {
    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    logger.log(5, "Какое то");
    logger.log(2, "Сообщение");
    // Дополнительные тесты
    logger.log(3, "Сообщение с максимально возможной многословностью")
}


*/