fn main() {
    let mut tree = BST::new();
    tree.insert(2);

    println!("{}", tree.search(2))
}

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;
        while let Some(ref mut node) = current {
            if value < node.value {
                current = &mut node.left;
            } else {
                current = &mut node.right;
            }
        }
        *current = Some(Box::new(Node::new(value)));
    }
}

impl BST {
    // ...

    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(ref node) = current {
            if value == node.value {
                return true;
            }
            if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }

    fn delete(&mut self, value: i32) {
        let mut current = &mut self.root;
        let mut parent = &mut None;
        while let Some(ref node) = current {
            if value == node.value {
                break;
            }
            parent = current;
            if value < node.value {
                current = &mut node.left;
            } else {
                current = &mut node.right;
            }
        }
        if current.is_none() {
            return;
        }
        let node = current.take().unwrap();
        if node.left.is_none() && node.right.is_none() {
            *parent = None;
        } else if node.left.is_none() {
            *parent = node.right.take();
        } else if node.right.is_none() {
            *parent = node.left.take();
        } else {
            let mut min = &mut node.right;
            while min.as_ref().unwrap().left.is_some() {
                min = &mut min.as_mut().unwrap().left;
            }
            let min_node = min.take().unwrap();
            *min = node.right.take();
            *parent = Some(min_node);
        }
    }
}
