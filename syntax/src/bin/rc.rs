/// Rc: 通过引用计数，让值有多个所有者，但都是只读的
/// 与 Rc 对应是的 Arc，因为 Rc 为了性能，使用的不是线程安全的引用计数器；而 Arc 是一个引用计数的智能指针，它实现了线程安全的引用计数器
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    _id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            _id: id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstram: Rc<Node>) {
        self.downstream = Some(downstram);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(node4));
    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());

    println!("node1: {:?}, node2: {:?}", node1, node2);

    // Rc 是一个只读的引用计数器，你无法拿到 Rc 结构内部数据的可变引用，来修改这个数据，所以 RefCell 就派上用场了
    // let node5 = Node::new(5);
    // let node3 = node1.get_downstream().unwrap();
    // node3.update_downstream(Rc::new(node5));
}
