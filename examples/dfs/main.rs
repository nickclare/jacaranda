use jacaranda::tree;

#[derive(Debug)]
struct Data {
    name: String,
}

impl Data {
    fn new<S: Into<String>>(name: S) -> Self {
        Data { name: name.into() }
    }
}

fn main() {
    let tree = make_tree();
    dfs(&tree);
}

fn dfs(tree: &tree::Tree<Data>) {
    search_node(tree, tree.root(), 0);
}

fn search_node(tree: &tree::Tree<Data>, node: tree::NodeIndex, level: usize) {
    // visit node
    let n = tree.get(node).unwrap();
    println!(
        "{text: >width$}",
        width = level * 2 + n.data().name.len(),
        text = n.data().name
    );
    for c in n.children() {
        search_node(tree, *c, level + 1);
    }
}

fn make_tree() -> tree::Tree<Data> {
    let mut tree = tree::Tree::new(Data::new("root"));
    let root = tree.root();
    let left = tree.add_child(root, Data::new("left")).unwrap();
    let _a = tree.add_child(left, Data::new("a")).unwrap();
    let _b = tree.add_child(left, Data::new("b")).unwrap();
    let right = tree.add_child(root, Data::new("right")).unwrap();
    let _c = tree.add_child(right, Data::new("c")).unwrap();
    let _d = tree.add_child(right, Data::new("d")).unwrap();
    tree
}
