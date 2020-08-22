use std::collections::BTreeMap;

#[derive(Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub struct NodeIndex(usize);

pub struct Tree<T> {
    nodes: BTreeMap<NodeIndex, Node<T>>,
    root: NodeIndex,
    next_idx: NodeIndex,
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        let mut result = Tree {
            nodes: BTreeMap::new(),
            root: NodeIndex(0),
            next_idx: NodeIndex(1),
        };

        result.nodes.insert(result.root, Node::new_root(root));

        result
    }

    fn get_mut(&mut self, index: NodeIndex) -> Option<&mut Node<T>> {
        self.nodes.get_mut(&index)
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Node<T>> {
        self.nodes.get(&index)
    }

    pub fn add_child(&mut self, parent: NodeIndex, child: T) -> Option<NodeIndex> {
        let idx = self.next_idx.clone();
        let child_node = if let Some(ref mut parent) = self.get_mut(parent) {
            let node = Node::new_child(idx, parent.index, child);
            parent.children.push(node.index);
            self.nodes.insert(idx, node);
            Some(idx)
        } else {
            // parent node doesn't exist
            None
        };

        if child_node.is_some() {
            self.next_idx = NodeIndex(self.next_idx.0 + 1);
        }
        child_node
    }

    pub fn data<I: Into<NodeIndex>>(&self, node: I) -> Option<&T> {
        self.get(node.into()).map(|n| n.data())
    }

    pub fn parent<I: Into<NodeIndex>>(&self, node: I) -> Option<NodeIndex> {
        self.get(node.into()).and_then(|n| n.parent())
    }

    pub fn children<I: Into<NodeIndex>>(&self, node: I) -> Option<&Vec<NodeIndex>> {
        self.get(node.into()).map(|n| n.children())
    }
}

pub struct Node<T> {
    data: T,
    index: NodeIndex,
    parent: Option<NodeIndex>,
    children: Vec<NodeIndex>,
}

impl<T> Node<T> {
    fn new_root(data: T) -> Self {
        Node {
            data: data,
            index: NodeIndex(0),
            parent: None,
            children: Vec::new(),
        }
    }

    fn new_child(index: NodeIndex, parent: NodeIndex, data: T) -> Self {
        Node {
            data,
            index,
            parent: Some(parent),
            children: Vec::new(),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn parent(&self) -> Option<NodeIndex> {
        self.parent
    }

    pub fn children(&self) -> &Vec<NodeIndex> {
        &self.children
    }
}

impl<T> From<Node<T>> for NodeIndex {
    fn from(n: Node<T>) -> NodeIndex {
        n.index
    }
}
