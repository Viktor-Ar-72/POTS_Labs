/*
/// Определяет длину последовательности Коллатца для числа n.
fn collatz_length(mut n: i32) -> u32 {
    let k : u32;
    // Если n == 1
    if n == 1 {
        k = 1;
    }
    // Если n на текущей итерации чётное
    else if n % 2 == 0 {
        n = n / 2;
        k = collatz_length(n) + 1;
    }
    // Если n на текущей итерации нечётное
    else {
        n = n * 3 + 1;
        k = collatz_length(n) + 1;
    }
    k
}

fn main() {
    //println!("Длина: {}", collatz_length(11));
    //println!("Длина: {}", collatz_length(10451));
    println!("Длина: {}", collatz_length(7));
}

// Тест для функции collatz_length() (запускается отдельно от main, сверяет результат с тем,
// что указан справа)
#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}
*/