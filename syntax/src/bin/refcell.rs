/// RefCell: 和 Rc 类似，RefCell 也绕过了 Rust 编译器的静态检查，允许我们在运行时，对某个[只读数据]进行可变借用
/// 这涉及到 rust 一个比较独特的特性，即内部可变性
///     1、用 let mut 显式地声明一个可变的值，或者通过 &mut 声明一个可变的引用，编译器可以在编译时进行严格地检查，保证只有可变的值或者可变的引用，
///         才能修改值内部的数据，这被称作外部可变性（exterior mutability），外部可变性通过 mut 关键字声明。
///     2、有时候我们希望能够绕开这个编译时的检查，对并未声明成 mut 的值或者引用，也想进行修改。也就是说，在编译器的眼里，值是只读的，但是在运行时，
///         这个值可以得到可变借用，从而修改内部的数据，这就是 RefCell 的用武之地。
///
/// 和 Rc 和 RefCell 类似，RefCell 也不是线程安全的，如果要在多线程中使用内部可变性，Rust 提供了 Mutex 和 RwLock
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    _id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            _id: id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstram: Rc<RefCell<Node>>) {
        self.downstream = Some(downstram);
    }

    pub fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let data = RefCell::new(1);
    {
        // 获得 RefCell 内部数据的可变借用
        let mut v = data.borrow_mut();
        *v += 1;
    } // 如果没有这对花括号，运行时会报错：already mutably borrowed: BorrowError，所有权的借用规则在此依旧有效，只不过它在运行时检测。
    println!("data: {:?}", data.borrow());

    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap());

    println!("node1: {:?}, node2: {:?}", node1, node2);

    // Rc 是一个只读的引用计数器，你无法拿到 Rc 结构内部数据的可变引用，来修改这个数据，所以 RefCell 就派上用场了
    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    node3
        .borrow_mut()
        .update_downstream(Rc::new(RefCell::new(node5)));

    println!("node1: {:?}, node2: {:?}", node1, node2);
}
