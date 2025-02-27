/*

mod Lab_6_0;

/// Операция над двумя выражениями.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// Выражение в форме узла дерева.
#[derive(Debug)]
enum Expression {
    /// Операция над двумя дочерними выражениями.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// Значение
    Value(i64),
}

// Непосредственное выполнение операции, в зависимости от текущего узла дерева
fn eval(e: Expression) -> i64 {
    // Выбор дальнейших действий в зависимости от типа узла
    match e {
        // Если это узел с числом - возвращает число
        Expression::Value(v) => v,
        // Если это узел с выражением - выполняет операцию, возвращает число
        Expression::Op {op, left, right} => {
            // Получение значения левого операнда через рекурсивный вызов
            let left_elem = eval(*left);
            // Получение значения правого операнда через рекурсивный вызов
            let  right_elem = eval(*right);
            // Выбор арифметической операции
            match op {
                Operation::Add => left_elem + right_elem,
                Operation::Mul => left_elem * right_elem,
                Operation::Sub => left_elem - right_elem,
                Operation::Div => {
                    if right_elem != 0 {
                        left_elem / right_elem
                    }
                    else {
                        //return -1
                        panic!("ERROR: Попытка делить на 0!")
                    }
                }
            }
        }
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

#[test]
fn test_zeros() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
}

// Функция main - в заготовке не было, чтобы компилятор не ругался + ещё парочка тестов
fn main() {
    // Перемножение отрицательных чисел даёт положительное число
    let mut result = eval(Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(-27)),
        right: Box::new(Expression::Value(-17))
    });
    println!("Result == {}", result);
    // Деление на 0 должно вызвать panic!()
    result = eval(Expression::Op {
        op: Operation::Div,
        left: Box::new(Expression::Value(-1)),
        right: Box::new(Expression::Value(0))
    });
    println!("Div by null result == {}", result);

}

*/