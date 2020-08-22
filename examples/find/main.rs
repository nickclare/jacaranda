use jacaranda::tree;

#[derive(Debug)]
struct Data {
    name: String,
    num: i32,
}

impl Data {
    fn new<S: Into<String>>(name: S, num: i32) -> Self {
        Data {
            name: name.into(),
            num,
        }
    }
}

fn main() {
    let tree: tree::Tree<Data> = make_tree();

    let target = "left";
    println!(
        "num for '{}' == {:?}",
        target,
        tree.find(|_, node| node.data().name == target)
            .map(|n| n.data().num)
    );
}

fn make_tree() -> tree::Tree<Data> {
    let mut tree = tree::Tree::new(Data::new("root", 0));
    let root = tree.root();
    let left = tree.add(root, Data::new("left", 1)).unwrap();
    let _a = tree.add(left, Data::new("a", 0)).unwrap();
    let _b = tree.add(left, Data::new("b", 0)).unwrap();
    let _c = tree.add(left, Data::new("c", 0)).unwrap();
    let right = tree.add(root, Data::new("right", 0)).unwrap();
    let _d = tree.add(right, Data::new("d", 0)).unwrap();
    let _e = tree.add(right, Data::new("e", 0)).unwrap();
    tree
}
