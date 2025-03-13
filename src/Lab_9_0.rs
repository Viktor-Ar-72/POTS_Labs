/* DEBUG
use std::collections::HashMap;
use std::hash::Hash;

/// Counter определяет количество значений типа T в коллекции.
struct Counter<T> {
    //values: HashMap<u32, u64>,
    values: HashMap<T, u64>,
}

impl<T> Counter<T>
// Добавление ограничения на тип T, HashMap требует, чтобы ключи реализовывали Eq и Hash
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
where T : Eq + Hash
{
    /// Создаем новый счетчик Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Считает количество появлений заданного значения.
    //fn count(&mut self, value: u32) {
    fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// Возвращает количество появлений заданного значения.
    //fn times_seen(&self, value: u32) -> u64 {
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    //let mut ctr = Counter::new();
    let mut ctr: Counter<u32> = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);
    for i in 10..20 {
        //println!("Значение {} видели {} раз", ctr.times_seen(i), i);
        println!("Значение {} видели {} раз", i, ctr.times_seen(i));
    }


    let mut strctr : Counter<&str> = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("Получили {} яблок", strctr.times_seen("apple"));

    let mut dop_struct : Counter<bool> = Counter::new();
    dop_struct.count(false);
    dop_struct.count(true);
    dop_struct.count(false);
    dop_struct.count(true);
    dop_struct.count(true);
    for bool_value in [false, true] {
        println!("Было найдено {} значений {}", dop_struct.times_seen(bool_value), bool_value);
    }

}
*/