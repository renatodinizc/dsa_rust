pub struct Vertex {
    data: String,
    adjacent_vertices: Vec<Box<Vertex>>,
}

pub struct Graph {
    vertices: Vec<Vertex>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new()
        }
    }

    pub fn add_new_vertex(&mut self, data: String) {
        self.vertices.push(Vertex { data, adjacent_vertices: Vec::new() })
    }

    // pub fn depth_first_search<'a>(
    //     &'a self,
    //     vertex: &'a Vertex,
    //     search_value: String,
    //     visited_vertices: &mut HashMap<String, bool>,
    // ) -> Option<&Vertex> {
    //     if vertex.value == search_value {
    //         return Some(vertex);
    //     }

    //     visited_vertices.insert(vertex.value.clone(), true);

    //     for children_vertex in &vertex.adjacent_vertices {
    //         match visited_vertices.get(&children_vertex.value) {
    //             Some(_value) => continue,
    //             None => {
    //                 return Self::depth_first_search(
    //                     self,
    //                     &children_vertex,
    //                     search_value,
    //                     visited_vertices,
    //                 )
    //             }
    //         }
    //     }

    //     None
    // }
}

#[test]
fn test_graph() {
}
