use std::collections::{HashMap, HashSet};
use std::fmt;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

// this like an alias
pub type AttrMap = HashMap<String, String>;

impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> Self {
        Self {
            tag_name,
            attributes,
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    // HashSet is like a hashmap but its all keys
    pub fn get_classes(&self) -> HashSet<&str> {
        let classes = self.attributes.get("class");
        match classes {
            Some(s) =>  s.split(' ').collect::<HashSet<&str>>(),
            None => HashSet::new()
        }
    }
}

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Self>) -> Self {
        Self {
            node_type,
            children,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // ref create a reference
            // https://doc.rust-lang.org/beta/rust-by-example/flow_control/match/destructuring/destructure_pointers.html
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref t) => write!(f, "{:?}", t),
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attributes_string = String::new();
        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!(" {}=\"{}\"", attr, value));
        }
        write!(f, "<{}, {}>", self.tag_name, attributes_string)
    }
}

pub fn pretty_print(node: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match &node.node_type {
        NodeType::Comment(c) => println!("{}<!--{}-->", indent, c),
        NodeType::Text(t) => println!("{}{}", indent, t),
        NodeType::Element(e) => println!("{}{:?}", indent, e)
    }

    for child in node.children.iter() {
        pretty_print(child, indent_size + 2);
    }

    match &node.node_type {
        NodeType::Element(e) => println!("{}<{}/>", indent, e.tag_name),
        _ => {}
    }
}