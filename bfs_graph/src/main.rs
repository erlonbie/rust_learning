use std::collections::VecDeque;

fn bfs(start: usize, graph: &Graph) {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; graph.size];

    queue.push_back(start);
    visited[start] = true;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        println!("Visiting node {}", current);

        // for &neighbor in &graph.edges[current] {
        //     if !visited[neighbor] {
        //         queue.push_back(neighbor);
        //         visited[neighbor] = true;
        //     }
        // }

        for &neighbor in graph.edges.get(current).expect("falhou") {
            if !visited[neighbor] {
                queue.push_back(neighbor);
                visited[neighbor] = true;
            }
        }
    }
}

fn dfs_non_recursive(graph: &Graph, start: usize) {
    let mut stack = VecDeque::new();
    let mut visited = vec![false; graph.size];

    stack.push_back(start);
    while let Some(node) = stack.pop_back() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        println!("Visiting node: {}", node);

        if let Some(neighbors) = graph.edges.get(node) {
            for &neighbor in neighbors {
                stack.push_back(neighbor);
            }
        }
    }
}

fn dfs(graph: &Graph, node: usize, visited: &mut Vec<bool>) {
    visited[node] = true;
    println!("Visiting node: {}", node);

    if let Some(neighbors) = graph.edges.get(node) {
        for &neighbor in neighbors {
            if !visited[neighbor] {
                dfs(graph, neighbor, visited);
            }
        }
    }
}

fn main() {

    let mut graph = Graph::new(7);

    graph.add_edge(0, 2);
    graph.add_edge(0, 1);
    graph.add_edge(1, 4);
    graph.add_edge(1, 3);
    graph.add_edge(2, 5);
    graph.add_edge(3, 6);
    // let graph = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];

    bfs(0, &graph);

    println!("Has edge 0 -> 1: {}", graph.has_edge(0, 1)); // true
    println!("Has edge 1 -> 3: {}", graph.has_edge(1, 3)); // true
    println!("Has edge 1 -> 4: {}", graph.has_edge(1, 4)); // true
    println!("Has edge 2 -> 6: {}", graph.has_edge(2, 6)); // false
    println!("Has edge 3 -> 5: {}", graph.has_edge(3, 5)); // false

    let mut visited = vec![false; graph.size];
    dfs(&graph, 0, &mut visited);

    println!("Has edge 0 -> 1: {}", graph.has_edge(0, 1)); // true
    println!("Has edge 1 -> 3: {}", graph.has_edge(1, 3)); // true
    println!("Has edge 1 -> 4: {}", graph.has_edge(1, 4)); // true
    println!("Has edge 2 -> 6: {}", graph.has_edge(2, 6)); // false
    println!("Has edge 3 -> 5: {}", graph.has_edge(3, 5)); // false

    dfs_non_recursive(&graph, 0);
}

struct Graph {
    size: usize,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        let edges = vec![Vec::new(); size];
        Self { size, edges }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
        // self.edges[from].sort();
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.edges.get(from).expect("não achou").contains(&to)
    }
    // fn remove_edge(&mut self, from: usize, to: usize) {
    //     self.edges[from][to] = false;
    // }
    //
}
