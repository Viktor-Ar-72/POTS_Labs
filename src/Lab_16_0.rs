/* DEBUG
/// Вычислите разность между значениями values на расстоянии offset друг от друга,
/// переходя по модулю в начало коллекции.
///
/// Элемент n результата это разность values[(n+offset)%len] - values[n].
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
    //todo!()
    let mut results : Vec<i32> = Vec::new();
    let mut i = 0;
    while i < values.len() {
        results.push(values[(i + offset) % values.len()] - values[i]);
        i += 1;
    }
    results
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}

fn main() {
    // Просто, чтобы было main - компилятор не будет Warning-и кидать
}




//fn main() {
//    println!("Hello, world!");
//}
*/