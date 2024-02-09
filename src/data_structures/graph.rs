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

        let mut vertex_b_index = 1000;
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

    pub fn traversal(&self, start_vertex_value: T, kind: &str) -> Vec<T> {
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
    ) -> Vec<T> {
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

        for &adjacent_index in &self.vertices[start_vertex_index].adjacent_vertices {
            if visited_vertices.get(&adjacent_index).is_none() {
                self.depth_first_traversal(self.vertices[adjacent_index].data, visited_vertices);
            } else {
                continue;
            }
        }
        let mut result: Vec<T> = vec![];

        for key in visited_vertices.keys() {
            result.push(self.vertices[*key].data);
        }

        result
    }

    fn breath_first_traversal(
        &self,
        start_vertex_value: T,
        visited_vertices: &mut HashMap<usize, bool>,
    ) -> Vec<T> {
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
            let current_vertex_index = queue.remove(0);

            for &adjacent_index in &self.vertices[current_vertex_index].adjacent_vertices {
                if visited_vertices.get(&adjacent_index).is_none() {
                    visited_vertices.insert(adjacent_index, true);
                    queue.push(adjacent_index);
                } else {
                    continue;
                }
            }
        }

        let mut result: Vec<T> = vec![];

        for key in visited_vertices.keys() {
            result.push(self.vertices[*key].data);
        }

        result
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
    use super::HashMap;

    #[test]
    fn test_add_new_vertex() {
        let mut graph_int: Graph<u32> = Graph::new();
        graph_int.add_new_vertex(1);
        assert_eq!(graph_int.vertices.len(), 1);

        let mut graph_char = Graph::new();
        graph_char.add_new_vertex('a');
        assert_eq!(graph_char.vertices.len(), 1);
    }

    #[test]
    fn test_link_vertices() {
        let mut graph_int: Graph<u32> = Graph::new();
        graph_int.add_new_vertex(1);
        graph_int.add_new_vertex(2);
        graph_int.link_vertices(1, 2);

        assert_eq!(graph_int.vertices[0].adjacent_vertices, vec![1]);
        assert_eq!(graph_int.vertices[1].adjacent_vertices, vec![0]);

        let mut graph_char = Graph::new();
        graph_char.add_new_vertex('a');
        graph_char.add_new_vertex('b');
        graph_char.link_vertices('a', 'b');

        assert_eq!(graph_char.vertices[0].adjacent_vertices, vec![1]);
        assert_eq!(graph_char.vertices[1].adjacent_vertices, vec![0]);
    }

    #[test]
    #[should_panic(expected = "One of the vertices does not exist")]
    fn test_link_nonexistent_vertices() {
        let mut graph_int = Graph::new();
        graph_int.add_new_vertex(1);
        graph_int.link_vertices(1, 2);
    }

    #[test]
    fn test_depth_first_search() {
        let mut graph_int: Graph<u32> = Graph::new();
        graph_int.add_new_vertex(1);
        graph_int.add_new_vertex(2);
        graph_int.add_new_vertex(3);
        graph_int.link_vertices(1, 2);
        graph_int.link_vertices(2, 3);

        let mut visited_vertices = HashMap::new();
        let search_result = graph_int.depth_first_search(1, 3, &mut visited_vertices);
        assert!(search_result.is_some());
        assert_eq!(search_result.unwrap().data, 3);

        let mut graph_char = Graph::new();
        graph_char.add_new_vertex('a');
        graph_char.add_new_vertex('b');
        graph_char.add_new_vertex('c');
        graph_char.link_vertices('a', 'b');
        graph_char.link_vertices('b', 'c');

        let mut visited_vertices_char = HashMap::new();
        let search_result_char =
            graph_char.depth_first_search('a', 'c', &mut visited_vertices_char);
        assert!(search_result_char.is_some());
        assert_eq!(search_result_char.unwrap().data, 'c');
    }

    #[test]
    fn test_traversal() {
        // test with u32
        let mut graph_int: Graph<u32> = Graph::new();
        graph_int.add_new_vertex(1);
        graph_int.add_new_vertex(2);
        graph_int.add_new_vertex(3);
        graph_int.link_vertices(1, 2);
        graph_int.link_vertices(2, 3);
        let dft_result = graph_int.traversal(1, "dft");
        let bft_result = graph_int.traversal(1, "bft");

        assert_eq!(dft_result.len(), 3);
        assert!(dft_result.contains(&1));
        assert!(dft_result.contains(&2));
        assert!(dft_result.contains(&3));

        assert_eq!(bft_result.len(), 3);
        assert!(bft_result.contains(&1));
        assert!(bft_result.contains(&2));
        assert!(bft_result.contains(&3));

        // test with char
        let mut graph_char = Graph::new();
        graph_char.add_new_vertex('a');
        graph_char.add_new_vertex('b');
        graph_char.add_new_vertex('c');
        graph_char.link_vertices('a', 'b');
        graph_char.link_vertices('b', 'c');
        let dft_result_char = graph_char.traversal('a', "dft");
        let bft_result_char = graph_char.traversal('a', "bft");

        assert_eq!(dft_result_char.len(), 3);
        assert!(dft_result_char.contains(&'a'));
        assert!(dft_result_char.contains(&'b'));
        assert!(dft_result_char.contains(&'c'));

        assert_eq!(bft_result_char.len(), 3);
        assert!(bft_result_char.contains(&'a'));
        assert!(bft_result_char.contains(&'b'));
        assert!(bft_result_char.contains(&'c'));
    }
}

// Reference: https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
