/* DEBUG
pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity = {verbosity}: {message}");
    }
}


/// Логировать сообщения только заданного уровня.
// Внесенные изменения: теперь передаётся не Logger, а реализация из семейства Logger по выбору
struct VerbosityFilter <V: Logger> {
    max_verbosity: u8,
    inner: V,
}

// Внесенные изменения: теперь передаются реализации Logger-а
impl<V: Logger> Logger for VerbosityFilter<V> {
    fn log(&self, verbosity: u8, message: &str) {
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


// TODO: Добавьте определение и реализацию Filter.
// Структура Filter для реализации фильтрации сообщений по разрешению
// В качестве передаваемого дальше сообщения - трейт Logger или его имплементации
// В качестве разрешения - замыкание, есть ли указанная строка в сообщении
struct Filter<V, P>
where
    V: Logger, // Определение возможных типов логгера - Logger и его производные StderrLogger и VerbosityFilter
    P: Fn(u8, &str) -> bool, // Замыкание, возвращающее bool - можно ли пропускать сообщение
{
    inner: V, // Хранение логгера
    permission: P, // Хранение замыкания
}

// Инициализация Filter
impl<V, P> Filter<V, P>
where
    V: Logger,
    P: Fn(u8, &str) -> bool,
{
    // Создание и инициализация нового Filter
    fn new(inner: V, permission: P) -> Self {
        Self {
            inner, // Переданный логгер хранится в inner
            permission // Переданное замыкание хранится в permission
        }
    }
}

// Имплементация Logger для Filter
impl<V, C> Logger for Filter<V, C>
where
    V: Logger,
    C: Fn(u8, &str) -> bool,
{
    fn log(&self, verbosity: u8, message: &str)
    {
        // Если при вызове log с определенным сообщением в main замыкание вернуло true,
        // то дальнейшее логирование в StderrLogger и/или VerbosityFilter проходит
        if (self.permission)(verbosity, message)
        {
            self.inner.log(verbosity, message);
        }
    }
}


fn main() {
    /* Вариант А - без учёта фильтра по важности */
    // Создание логгера на основе StderrLogger
    let logger = Filter::new(
        // Тип логгера
        StderrLogger,
        // Реализация замыкания - принимает на вход:
        // 1) _verbosity - значение не важно, дальше не учитывается
        // 2) msg - возвращает true, если в msg есть указанная строка, иначе false
        |_verbosity, msg| msg.contains("yikes")
    );

    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");

    /* Вариант Б - с учётом фильтра по важности */
    // Сначала создаётся StderrLogger
    let stdeer_logger = StderrLogger;
    // На его основе создаётся VerbosityFilter
    let verbosity_logger = VerbosityFilter {
        inner: stdeer_logger,
        max_verbosity: 4
    };
    // На основе объекта VerbosityFilter создаётся объект Filter
    let main_filter_logger = Filter::new(
        verbosity_logger, // Переданный объект VerbosityLogger
        // Замыкание с разрешением всех сообщений, не содержащих ERROR
        |_verbosity, message| !message.contains("ERROR")
    );
    // Тестовые сообщения
    main_filter_logger.log(1, "ERROR: some error");
    main_filter_logger.log(5, "WARNING: Some small warning");
    main_filter_logger.log(2, "ERROR: some error");
    main_filter_logger.log(3, "WARNING: Some good warning");
}





//fn main() {
//    println!("Hello, world!");
//}

*/