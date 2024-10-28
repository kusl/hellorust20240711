use std::collections::{HashMap, VecDeque};

struct Graph {
    nodes: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, value: &str) {
        self.nodes.entry(value.to_string()).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        if let Some(neighbors) = self.nodes.get_mut(from) {
            neighbors.push(to.to_string());
        }
    }

    fn bfs(&self, start: &str) {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string(), true);

        while let Some(node) = queue.pop_front() {
            println!("Visited {} in BFS", node);

            if let Some(neighbors) = self.nodes.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor) {
                        queue.push_back(neighbor.to_string());
                        visited.insert(neighbor.to_string(), true);
                    }
                }
            }
        }
    }

    fn dfs(&self, start: &str) {
        let mut visited = HashMap::new();
        self.dfs_recursive(start, &mut visited);
    }

    fn dfs_recursive(&self, node: &str, visited: &mut HashMap<String, bool>) {
        visited.insert(node.to_string(), true);
        println!("Visited {} in DFS", node);

        if let Some(neighbors) = self.nodes.get(node) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    self.dfs_recursive(neighbor, visited);
                }
            }
        }
    }
}

