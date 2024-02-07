use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Vertex<T> {
    data: T,
    pub adjacent_vertices: Vec<usize>, // Store indices of adjacent vertices
}

pub struct Graph<T> {
    vertices: Vec<Vertex<T>>,
}

impl<T: PartialEq + Eq + Hash + Copy + std::fmt::Debug> Graph<T> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
        }
    }

    pub fn add_new_vertex(&mut self, data: T) {
        self.vertices.push(Vertex {
            data,
            adjacent_vertices: Vec::new(),
        })
    }

    pub fn link_vertices(&mut self, vertex_a_value: T, vertex_b_value: T) {
        let mut vertex_a_index = 0;
        for (i, vertex) in self.vertices.iter().enumerate() {
            if vertex.data == vertex_a_value {
                vertex_a_index = i;
            }
        }

        let mut vertex_b_index = 0;
        for (i, vertex) in self.vertices.iter().enumerate() {
            if vertex.data == vertex_b_value {
                vertex_b_index = i;
            }
        }

        let vertex_a = self.vertices.get(vertex_a_index);
        let vertex_b = self.vertices.get(vertex_b_index);

        if vertex_a.is_some() && vertex_b.is_some() {
            self.vertices[vertex_a_index]
                .adjacent_vertices
                .push(vertex_b_index);
            self.vertices[vertex_b_index]
                .adjacent_vertices
                .push(vertex_a_index);
        } else {
            panic!("One of the vertices does not exist");
        }
    }

    pub fn depth_first_search(
        &self,
        start_vertex_value: T,
        target_value: T,
        visited_vertices: &mut HashMap<usize, bool>,
    ) -> Option<&Vertex<T>> {
        let start_vertex_index = self.vertices.iter().enumerate().find_map(|(i, v)| {
            if v.data == start_vertex_value {
                Some(i)
            } else {
                None
            }
        })?;

        visited_vertices.insert(start_vertex_index, true);

        if self.vertices[start_vertex_index].data == target_value {
            return Some(&self.vertices[start_vertex_index]);
        }

        visited_vertices.insert(start_vertex_index, true);

        for &adjacent_index in &self.vertices[start_vertex_index].adjacent_vertices {
            if !visited_vertices.get(&adjacent_index).unwrap_or(&false) {
                if let Some(found) = self.depth_first_search(
                    self.vertices[adjacent_index].data,
                    target_value,
                    visited_vertices,
                ) {
                    return Some(found);
                }
            }
        }

        None
    }

    pub fn traversal(&self, start_vertex_value: T, kind: &str) {
        let mut visited_vertices: HashMap<usize, bool> = HashMap::new();

        if kind == "bft" {
            Self::breath_first_traversal(self, start_vertex_value, &mut visited_vertices)
        } else if kind == "dft" {
            Self::depth_first_traversal(self, start_vertex_value, &mut visited_vertices)
        } else {
            panic!("Pick between bft(breath-first traversal) and dft(depth-first traversal)")
        }
    }

    fn depth_first_traversal(
        &self,
        start_vertex_value: T,
        visited_vertices: &mut HashMap<usize, bool>,
    ) {
        let start_vertex_index = self
            .vertices
            .iter()
            .enumerate()
            .find_map(|(i, v)| {
                if v.data == start_vertex_value {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();

        visited_vertices.insert(start_vertex_index, true);
        println!("{:?}", self.vertices.get(start_vertex_index));

        for &adjacent_index in &self.vertices[start_vertex_index].adjacent_vertices {
            if visited_vertices.get(&adjacent_index).is_none() {
                self.depth_first_traversal(self.vertices[adjacent_index].data, visited_vertices)
            } else {
                continue;
            }
        }
    }

    fn breath_first_traversal(
        &self,
        start_vertex_value: T,
        visited_vertices: &mut HashMap<usize, bool>,
    ) {
        let start_vertex_index = self
            .vertices
            .iter()
            .enumerate()
            .find_map(|(i, v)| {
                if v.data == start_vertex_value {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();

        visited_vertices.insert(start_vertex_index, true);

        let mut queue: Vec<usize> = Vec::new();
        queue.push(start_vertex_index);

        while !queue.is_empty() {
            let current_vertex = queue.remove(0);

            for &adjacent_index in &self.vertices[start_vertex_index].adjacent_vertices {
                if visited_vertices.get(&adjacent_index).is_none() {
                    visited_vertices.insert(current_vertex, true);
                    queue.push(adjacent_index);
                    println!("{:?}", self.vertices.get(adjacent_index));
                } else {
                    continue;
                }
            }
        }
    }
}

impl<T: PartialEq + Eq + Hash + Copy + std::fmt::Debug> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
    struct TestNode {
        id: i32,
    }

    #[test]
    fn test_add_new_vertex_i32() {
        let mut graph: Graph<i32> = Graph::new();
        graph.add_new_vertex(1);
        assert_eq!(graph.vertices.len(), 1);
        assert_eq!(graph.vertices[0].data, 1);
    }

    #[test]
    fn test_add_new_vertex_char() {
        let mut graph: Graph<char> = Graph::new();
        graph.add_new_vertex('A');
        assert_eq!(graph.vertices.len(), 1);
        assert_eq!(graph.vertices[0].data, 'A');
    }

    #[test]
    fn test_add_new_vertex_custom_struct() {
        let mut graph: Graph<TestNode> = Graph::new();
        let node = TestNode { id: 1 };
        graph.add_new_vertex(node);
        assert_eq!(graph.vertices.len(), 1);
        assert_eq!(graph.vertices[0].data, node);
    }

    #[test]
    fn test_link_vertices_i32() {
        let mut graph = Graph::new();
        graph.add_new_vertex(1);
        graph.add_new_vertex(2);
        graph.link_vertices(1, 2);

        assert_eq!(graph.vertices[0].adjacent_vertices, vec![1]);
        assert_eq!(graph.vertices[1].adjacent_vertices, vec![0]);
    }

    #[test]
    fn test_link_vertices_char() {
        let mut graph = Graph::new();
        graph.add_new_vertex('A');
        graph.add_new_vertex('B');
        graph.link_vertices('A', 'B');

        assert_eq!(graph.vertices[0].adjacent_vertices, vec![1]);
        assert_eq!(graph.vertices[1].adjacent_vertices, vec![0]);
    }

    #[test]
    fn test_link_vertices_custom_struct() {
        let mut graph = Graph::new();
        let node1 = TestNode { id: 1 };
        let node2 = TestNode { id: 2 };
        graph.add_new_vertex(node1);
        graph.add_new_vertex(node2);
        graph.link_vertices(node1, node2);

        assert_eq!(graph.vertices[0].adjacent_vertices, vec![1]);
        assert_eq!(graph.vertices[1].adjacent_vertices, vec![0]);
    }
}

// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
