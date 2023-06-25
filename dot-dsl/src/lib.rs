pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge, node::Node};

    pub mod graph_items {

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }

                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(&name.to_string()).map(|x| &**x)
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                nodes: Vec<String>,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(first_node: &str, second_node: &str) -> Self {
                    Edge {
                        nodes: Vec::from_iter([first_node.to_string(), second_node.to_string()]),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }

                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(&name.to_string()).map(|x| &**x)
                }
            }
        }
    }

    #[derive(PartialEq, Eq, Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            nodes
                .iter()
                .for_each(|node| self.nodes.push((*node).clone()));
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            edges
                .iter()
                .for_each(|edge| self.edges.push((*edge).clone()));
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            let string_name = String::from(name);
            self.nodes.iter().find(|node| node.name.eq(&string_name))
        }
    }
}
