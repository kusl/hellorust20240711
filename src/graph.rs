use std::collections::{HashMap, VecDeque};

pub struct Graph {
    nodes: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, value: &str) {
        self.nodes.entry(value.to_string()).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if let Some(neighbors) = self.nodes.get_mut(from) {
            neighbors.push(to.to_string());
        }
    }

    pub fn bfs(&self, start: &str) -> Vec<String> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string(), true);

        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(neighbors) = self.nodes.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor) {
                        queue.push_back(neighbor.to_string());
                        visited.insert(neighbor.to_string(), true);
                    }
                }
            }
        }

        result
    }

    pub fn dfs(&self, start: &str) -> Vec<String> {
        let mut visited = HashMap::new();
        let mut result = Vec::new();
        self.dfs_recursive(start, &mut visited, &mut result);
        result
    }

    fn dfs_recursive(
        &self,
        node: &str,
        visited: &mut HashMap<String, bool>,
        result: &mut Vec<String>,
    ) {
        visited.insert(node.to_string(), true);
        result.push(node.to_string());

        if let Some(neighbors) = self.nodes.get(node) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    self.dfs_recursive(neighbor, visited, result);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        graph.add_node("A");
        graph.add_node("B");
        graph.add_node("C");
        graph.add_node("D");
        graph.add_edge("A", "B");
        graph.add_edge("A", "C");
        graph.add_edge("B", "D");
        graph.add_edge("C", "D");

        let bfs_result = graph.bfs("A");
        assert_eq!(bfs_result, vec!["A", "B", "C", "D"]);
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new();
        graph.add_node("A");
        graph.add_node("B");
        graph.add_node("C");
        graph.add_node("D");
        graph.add_edge("A", "B");
        graph.add_edge("A", "C");
        graph.add_edge("B", "D");
        graph.add_edge("C", "D");

        let dfs_result = graph.dfs("A");
        assert_eq!(dfs_result, vec!["A", "B", "D", "C"]);
    }
}
