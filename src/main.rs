// Функция траснпонирования матрицы 3x3
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // Создание дефолтной матрицы 3x3
    let mut transposed_matrix = [[0; 3]; 3];
    // Транспонирование матрицы
    for i in 0..3  {
        for j in 0..3 {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }
    // Возвращает транспонированную матрицу
    transposed_matrix
}

// Тесты
#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    //let matrix = [
    //    [101, 102, 103], // <-- комментарий заставляет rustfmt добавить новую строку
    //    [201, 202, 203],
    //    [301, 302, 303],
    //];
    let matrix = [
        [17, 24, 29], //
        [10, 41, 54],
        [63, 88, 72],
    ];
    // Вывод первоначальной и транспонированной матрицы
    println!("Матрица: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("Транспонированная матрица: {:#?}", transposed);
}




//fn main() {
//    println!("Hello, world!");
//}