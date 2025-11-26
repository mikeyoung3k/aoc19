use std::path::Path;
use std::fs::read_to_string;

use crate::BASE_DIR;

pub fn run() -> (isize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day6.txt");
    let input_string = read_to_string(path).expect("Error reading file");
    let head = OrbitNode::from_string(input_string);
    (pt1(&head),pt2(&head))
}

fn pt1(head: &OrbitNode) -> isize {
    head.count_children(1)
}

fn pt2(head: &OrbitNode) -> isize {
    let mut you: Vec<&str> = vec![];
    let mut san: Vec<&str> = vec![];
    head.find_parents("YOU", &mut you);
    head.find_parents("SAN", &mut san);
    let you_count = you.iter().fold(0,|acc, s| if !san.contains(s) {acc + 1} else {acc}) - 1;
    let san_count = san.iter().fold(0,|acc, s| if !you.contains(s) {acc + 1} else {acc}) - 1;
    you_count+san_count
    
}


#[derive(Debug, PartialEq, Eq)]
struct OrbitNode {
    id: String,
    children: Option<Vec<OrbitNode>>,
}

impl OrbitNode {
    fn from_string(input: String) -> OrbitNode {
        let mut orphan_nodes: Vec<OrbitNode> = vec![];
        // Return the head node, which will own all children nodes
        let mut orbits = input.lines();


        let mut head = OrbitNode{
            id: "".to_string(),
            children: None
        };

        'outer: while let Some(orbit) = orbits.next() {
            // Check if child exists in orphan nodes and push. If not, create and push
            let mut parts = orbit.split(")").map(|s| s.to_owned());
            let id = parts.next().unwrap();
            let child_id = parts.next().unwrap();
            // Before creating a child - see if child already exists in the orphan nodes
            let child = if let Some(found_child) = orphan_nodes.extract_if(..,|n| &n.id == &child_id).next() {
                found_child
            } else {
                OrbitNode{
                    id: child_id,
                    children: None,
                }
            };

            // Searching main tree for child
            if let Some(found) = head.get_child_node(&id){
                found.insert_child(child);
            } else {
                for node in &mut orphan_nodes {
                    if let Some(parent) = node.get_child_node(&id){
                        parent.insert_child(child);
                        // If found, continue to next orbit line
                        continue 'outer;
                    }
                }
                // If we don't find it, create and push to orphan nodes
                let parent = OrbitNode{
                    id,
                    children: Some(vec![child]),
                };
                // Or - if we've found COM - make that head
                if &parent.id == "COM" {
                    head = parent
                } else {
                    orphan_nodes.push(parent);
                }
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

    fn count_children(&self,level:isize) -> isize {
        let mut count = 0;
        if let Some(children) = &self.children {
            for child in children {
                count += level;
                count += child.count_children(level + 1);
            }
        }
        count
    }

    fn find_parents<'a>(&'a self, target: &str, result: &mut Vec<&'a str>) {
        if self.id == target {
            result.push(&self.id);
            return
        }
        
        if let Some(children) = &self.children {
            for child in children {
                child.find_parents(target,result);
                if !result.is_empty() {
                    result.push(&self.id);
                    return
                }
            }
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
    fn test_pt2() {
        let test_path = Path::new(BASE_DIR).join("test").join("day6-pt2.txt");
        let test_input_string = read_to_string(test_path).expect("Error reading file");
        let test_data = OrbitNode::from_string(test_input_string);
        assert_eq!(pt2(&test_data), 4);
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
    
    #[test]
    fn test_find_parents() {
        let test_path = Path::new(BASE_DIR).join("test").join("day6-pt2.txt");
        let test_input_string = read_to_string(test_path).expect("Error reading file");
        let test_data = OrbitNode::from_string(test_input_string);
        let mut res = vec![];
        test_data.find_parents("YOU",&mut res);
        let expect = vec!["YOU","K","J","E","D","C","B","COM"];
        assert_eq!(expect,res);
    }
}