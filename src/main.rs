fn fib(n: u32) -> u32 {
    // if (n < 2) {
    if n < 3 {
        // первый шаг.
        return 1;
    } else {
        // рекурсия.
        return fib(n-1) + fib(n-2);
    }
}
fn main() {
    // Примечание - всё же корректнее считать, что и 2 число также равняется 1
    // 0, 1, 1, 2, 3, 5, ...
    let n :u32 = 22;
    println!("fib({n}) = {}", fib(n));
}


//fn main() {
//    println!("Hello, world!");
//}