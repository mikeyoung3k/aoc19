use std::path::Path;
use std::fs::read_to_string;

use crate::BASE_DIR;

pub fn run() -> (isize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day6.txt");
    let input_string = read_to_string(path).expect("Error reading file");
    let test_data = OrbitNode::from_string(input_string);
    (0,0)
}

fn pt1(head: &OrbitNode) -> isize {
    0
}

fn pt2() -> isize {
    0
}

#[derive(Debug, PartialEq, Eq)]
struct OrbitNode {
    id: String,
    children: Option<Vec<OrbitNode>>,
}

impl OrbitNode {
    fn from_string(input: String) -> OrbitNode {
        // Return the head node, which will own all children nodes
        let mut orbits = input.lines();
        let mut parts = orbits.next().unwrap().split(")").map(|s| s.to_owned());
        let id = parts.next().unwrap();
        let child_id = parts.next().unwrap();
        let child = OrbitNode{
            id: child_id,
            children: None,
        };

        let mut head = OrbitNode{
            id,
            children: Some(vec![child]),
        };

        while let Some(orbit) = orbits.next() {
            let mut parts = orbit.split(")").map(|s| s.to_owned());
            let id = parts.next().unwrap();
            let child_id = parts.next().unwrap();
            let child = OrbitNode{
                id: child_id,
                children: None,
            };
            if let Some(found) = head.get_child_node(&id){
                found.insert_child(child);
            }
            else {
                panic!("Owning node not found for {}", child.id);
            }
        }


        head
    }

    fn get_child_node(&mut self, id: &str) -> Option<&mut OrbitNode> {
        if self.id == id {
            return Some(self);
        }
        if let Some(children) = &mut self.children {
            for child in children {
                if child.id == id {
                    return Some(child);
                } else {
                    if let Some(node) = child.get_child_node(id) {
                        return Some(node);
                    }
                }
            }
        }
        None
    }

    fn insert_child(&mut self, child: OrbitNode) {
        if let Some(children) = &mut self.children {
            children.push(child);
        } else {
            self.children = Some(vec![child]);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pt1() {
        let test_path = Path::new(BASE_DIR).join("test").join("day6.txt");
        let test_input_string = read_to_string(test_path).expect("Error reading file");
        let test_data = OrbitNode::from_string(test_input_string);
        assert_eq!(pt1(&test_data), 42);
    }

    #[test]
    fn test_map_build() {
        // COM - B - C
        //         - D -E
        let test_path = Path::new(BASE_DIR).join("test").join("day6-simple.txt");
        let test_input_string = read_to_string(test_path).expect("Error reading file");
        let test_data = OrbitNode::from_string(test_input_string);
        let node_e = OrbitNode{id: "E".to_string(), children: None};
        let node_d = OrbitNode{id: "D".to_string(), children: Some(vec![node_e])};
        let node_c = OrbitNode{id: "C".to_string(), children: None};
        let node_b = OrbitNode{id: "B".to_string(), children: Some(vec![node_c, node_d])};
        let node_com = OrbitNode{id: "COM".to_string(), children: Some(vec![node_b])};

        assert_eq!(test_data, node_com);
    }
}