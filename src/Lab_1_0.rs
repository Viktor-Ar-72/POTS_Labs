/* LR 1 version 1.0
fn fib(n: u32) -> u32 {
    // if (n < 2) {
    if (n < 3) {
        // первый шаг.
        return 1;
    } else {
        // рекурсия.
        return fib(n-1) + fib(n-2);
    }
}
fn main() {
    // Примечание
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
*/