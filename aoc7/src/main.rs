use std::{
    env,
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Node>
}

#[derive(Debug)]
struct NodeZipper {
    node: Node,
    parent: Option<Box<NodeZipper>>,
    index_in_parent: usize,
}

impl NodeZipper {
    fn child(mut self, index: usize) -> Option<NodeZipper> {
        let child = self.node.children.swap_remove(index);
        return Some(NodeZipper {
            node: child,
            parent: Some(Box::new(self)),
            index_in_parent: index,
        });
    }
    fn parent(self) -> NodeZipper {
        let NodeZipper {node,parent,index_in_parent} = self;
        let NodeZipper { node: mut parent_node, parent: parent_parent, index_in_parent: parent_index_in_parent } = *parent.unwrap();
        parent_node.children.push(node);
        let len = parent_node.children.len();
        parent_node.children.swap(index_in_parent, len-1);
        NodeZipper { node: parent_node, parent: parent_parent, index_in_parent: parent_index_in_parent }
    }
    fn finish(mut self) -> Node {
        while let Some(_) = self.parent {
            self = self.parent();
        }

        self.node
    }
    fn add(self, input: String) -> NodeZipper {
        if input.starts_with("dir ") {
            return self.add_dir(input[4..].to_string());
        }
        let mut spl = input.split(" ");
        let size: usize = spl.next().unwrap().parse().unwrap();
        let name = spl.next().unwrap().to_string();
        self.add_file(name, size)

    }
    fn add_file(mut self, name: String, size: usize) -> NodeZipper {
         self.node.children.push(Node { name, size, children: vec![] });
        NodeZipper { node: self.node, parent: self.parent, index_in_parent: self.index_in_parent }
    }
    fn add_dir(mut self, name: String) -> NodeZipper {
        self.node.children.push(Node{ name, children: vec![], size: 0 });
        NodeZipper { node: self.node, parent: self.parent, index_in_parent: self.index_in_parent }
    }
    fn cd(self, name: String) -> Option<NodeZipper> {
        if name == ".." {
            return Some(self.parent())
        }
        let ind = self.node.children.iter().position(|x| x.name == name).unwrap();
        self.child(ind)
    }
}

impl Node {
    fn new() -> Self {
        Node{ name: "".to_string(), children: vec![], size: 0 }
    }
    fn zipper(self) -> NodeZipper {
        NodeZipper { node: self, parent: None, index_in_parent: 0 }
    }
    fn get_child_sizes(&self, pn: String, cache: &mut HashMap<String,usize>) -> usize {
        if self.children.len() == 0 {
            return self.size;
        } else {
            let mut size: usize = 0;
            for n in self.children.iter() {
                size += n.get_child_sizes(pn.clone()+"/"+&n.name, cache);
            };
            cache.insert(pn, size);
            size
        }
    }
}

fn count_sizes_1(node: &Node) -> usize {
    let mut cache: HashMap<String,usize> = HashMap::new();
    node.get_child_sizes("".to_string(), &mut cache);
    let flt =
    cache.values().filter(|&x| x <= &100000);
    flt.sum()

}
fn calculate_2(node: Node) -> usize {
    let mut cache: HashMap<String,usize> = HashMap::new();
    node.get_child_sizes("".to_string(), &mut cache);
    let total = cache.get("").unwrap();
    let free = 70000000 - *total;
    let space_needed = 30000000 - free;
    *cache.values().filter(|x| *x >= &space_needed).min().unwrap()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut node = Node::new().zipper();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("$ cd /") || line.starts_with("$ ls") {  }
        else if line.starts_with("$ cd ") {
            node = node.cd(line[5..].to_string()).unwrap();
        }
        else {
            node = node.add(line);
        }
    }
    let node = node.finish();
    println!("1: {:?}", count_sizes_1(&node));
    println!("2: {:?}", calculate_2(node));
}
