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

    pub fn depth_first_traversal(
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

    pub fn breath_first_traversal(
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

#[test]
fn test_graph() {
    let mut graph: Graph<&str> = Graph::new();

    graph.add_new_vertex("Maria");
    graph.add_new_vertex("John");
    graph.add_new_vertex("Rupert");
    graph.add_new_vertex("Harry");

    graph.link_vertices("Maria", "John");
    graph.link_vertices("Maria", "Rupert");
    graph.link_vertices("John", "Harry");
    graph.link_vertices("Rupper", "Harry");

    let mut hsh: HashMap<usize, bool> = HashMap::new();
    // let result = graph.depth_first_search("Maria", "Harry", &mut hsh);

    // println!("{:?}", result);

    // graph.depth_first_traversal("Maria", &mut hsh);
    graph.breath_first_traversal("Maria", &mut hsh)
}

// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
