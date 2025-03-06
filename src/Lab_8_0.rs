/* DEBUG
// Подключение Ordering (Ord идёт в комплекте)
use std::cmp::Ordering;

// TODO: Сделайте функцию min которая вызывается в main.
// Ord уже реализован для i32, char, String и др -> можно использовать как универсальный тип
fn min<T: Ord>(first_element : T, second_element : T) -> T {
    // Проверка элементов между собой
    match first_element.cmp(&second_element) {
        // Если первый элемент меньше или равен второму - функция возвращает его
        Ordering::Less | Ordering::Equal => first_element,
        // Иначе возвращает второй элемент
        Ordering::Greater => second_element
    }
}

// Дополнительные тесты
#[test]
fn test_min_numbers() {
    // Тесты на сравнение чисел
    assert_eq!(min(0, 0), 0);
    assert_eq!(min(-11, 0), -11);
    assert_eq!(min(-8, -8), -8);

}

#[test]
fn test_min_chars() {
    // Тесты на сравнение символов
    assert_eq!(min('a', 'т'), 'a');
    assert_eq!(min('М', 'm'), 'm');
    assert_eq!(min('q', '7'), '7');
}

#[test]
fn test_min_strings() {
    // Тесты на сравнение строк
    assert_eq!(min("Not empty string", ""), "");
    assert_eq!(min("Строка на русском", "String on English"), "String on English");
    assert_eq!(min("qwerty1234abc", "qwerty1234abс"), "qwerty1234abc");
}

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}





//fn main() {
//    println!("Hello, world!");
//}

*/