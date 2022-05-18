use crate::{
    node::Node,
    style::Style,
    utils::quote_string
};

/// `Graph`'s subgraph
#[derive(Clone)]
pub struct Subgraph {
    pub name: String,
    nodes: Vec<Node>,
    label: String,
    style: Style,
    color: Option<String>,
}

impl Subgraph {
    pub fn new(name: &str) -> Self {
        Subgraph { name: String::from(name), nodes: vec![], label: String::new(), style: Style::None, color: None}
    }

    pub fn add_node(&mut self, node: Node) -> () {
        self.nodes.push(node);
    }

    pub fn add_nodes(&mut self, nodes: Vec<Node>) -> () {
        self.nodes.append(&mut nodes.clone());
    }

    pub fn label(&self, label: &str) -> Self {
        let mut subg = self.clone();
        subg.label = String::from(label);
        subg
    }

    pub fn style(&self, style: Style) -> Self {
        let mut subg = self.clone();
        subg.style = style;
        subg
    }

    pub fn color(&self, color: Option<&str>) -> Self {
        let mut subg = self.clone();
        subg.color = match color {
            Some(c) => Some(String::from(c)),
            None => None
        };
        subg
    }

    pub fn to_dot_string(&self) -> String {
        let mut text = vec!["subgraph ", self.name.as_str(), " {\n        "];

        text.push("label=\"");
        text.push(self.label.as_str());
        text.push("\";\n        ");

        if self.style != Style::None {
            text.push("style=\"");
            text.push(self.style.as_slice());
            text.push("\";\n        ");
        }

        let colorstring: String;
        if let Some(c) = &self.color {
            colorstring = quote_string(c.to_string());
            text.push("color=");
            text.push(&colorstring);
            text.push(";\n        ");
        }

        let subgraph_node_names = self.nodes
            .iter()
            .map(|n| n.to_dot_string())
            .collect::<Vec<String>>()
            .join("\n        ");
        text.push(&subgraph_node_names);

        text.push("\n    }");
        return text.into_iter().collect();
    }
}