/*
/// Узел двоичного дерева.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// Возможно пустое поддерево.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// Контейнер сохраняющий множество значений, с помощью двоичного дерева.
///
/// Если одно значение добавляется несколько раз, сохраняется только одно.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self {
            root: Subtree::new()
        }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

// Реализуйте методы new, insert, len, и has для `Subtree`.
impl<W: Ord> Subtree<W> {
    // Создание пустого поддерева
    fn new() -> Self{
        Subtree(None)
    }
    // Вставка нового значения в поддерево
    fn insert(&mut self, value: W) {
        // Сначала проверка, есть ли элементы в поддереве
        match self.0 {
            // Если элементов нет, то нужно вставить новое значение
            None => {
                // Инициализации структуры узла
                self.0 = Some(Box::new(Node {
                    value,
                    left: Subtree::new(),
                    right: Subtree::new(),
                }));
            },
            // Если элементы все же есть, то происходит сравнение с ними
            Some(ref mut current_node) => {
                if value < current_node.value {
                    current_node.left.insert(value);
                }
                else if value > current_node.value {
                    current_node.right.insert(value);
                }
                // Если значение равно, то для уникальности ничего не добавляется
                else {}
            },
        }
    }
    // Поиск элемента в поддереве (рекурсивно)
    fn has(&self, value: &W) -> bool {
        match self.0 {
            // Если поддерево отсутствует
            None => false,
            // Если поддерево присутствует
            Some(ref current_node) => {
                // Если нужный элемент совпадает
                //if *value == current_node.value {
                if value == &current_node.value {
                    true
                }
                // Если больше текущего - проверка правого узла
                //else if *value > current_node.value {
                else if value > &current_node.value {
                    current_node.right.has(value)
                }
                // Иначе - левого узла
                else {
                    current_node.left.has(value)
                }
            },
        }
    }
    // Подсчёт глубины поддерева (рекурсивно)
    fn len(&self) -> usize {
        match self.0 {
            // Если поддерево отсутствует
            None => 0,
            // Если присутствует - +1 к глубине и проверка его узлов
            Some(ref current_node) => 1 + current_node.left.len() + current_node.right.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}

// Чтобы не было ошибок, что отсутствует main
fn main() {}




//fn main() {
//    println!("Hello, world!");
//}
*/