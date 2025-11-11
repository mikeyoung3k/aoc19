use std::collections::{HashSet,HashMap};
use std::path::Path;
// Right is positive
// Up is positive

pub fn run() -> (isize, isize) {
    let path = Path::new(crate::BASE_DIR).join("real").join("day3.txt");
    let data = std::fs::read_to_string(path).expect("Failed to read file");
    let mut lines = data.lines();
    let map_a = create_map(lines.next().unwrap());
    let map_b = create_map(lines.next().unwrap());
    let p1 = pt1(&map_a, &map_b);
    let p2 = pt2(&map_a, &map_b);
    (p1,p2)
}

fn pt1(set_a: &HashMap<(isize,isize),isize>, set_b: &HashMap<(isize,isize),isize>) -> isize {
    let set_a = create_set(set_a);
    let set_b = create_set(set_b);
    find_min_distance(set_a, set_b)
}

fn pt2(map_a: &HashMap<(isize,isize),isize>, map_b: &HashMap<(isize,isize), isize>) -> isize {
    find_min_delay(map_a, map_b)
}

fn find_min_distance(set_a: HashSet<&(isize,isize)>, set_b: HashSet<&(isize,isize)>) -> isize {
    let mut min = isize::MAX;
    for point in set_a.intersection(&set_b) {
        let dist = point.0.abs() + point.1.abs();
        if dist < min {
            min = dist;
        }
    }
    min
}

fn find_min_delay(map_a: &HashMap<(isize, isize), isize>, map_b: &HashMap<(isize, isize), isize>) -> isize {
    let set_a = create_set(map_a);
    let set_b = create_set(map_b);
    let mut min = isize::MAX;
    for intersection in set_a.intersection(&set_b){
        let delay = map_a.get(intersection).unwrap() + map_b.get(intersection).unwrap();
        if delay < min {
            min = delay;
        }
    }
    min

}

fn create_set(input: &HashMap<(isize,isize),isize>) -> HashSet<&(isize, isize)> {
    HashSet::from_iter(input.keys())  
}
fn create_map(input: &str) -> HashMap<(isize, isize),isize> {
    let mut set = HashMap::new();
    let mut start = (0,0);
    let mut steps = 0;
    for s in input.split(','){
        let dir = s.chars().next().unwrap();
        let dist = s[1..].parse::<isize>().unwrap();
        match dir {
            'R' => {
                for _ in 0..dist {
                    steps += 1;
                    start.0 += 1;
                    set.insert(start,steps);
                }
            },
            'L' => {
                for _ in 0..dist {
                    steps += 1;
                    start.0 -= 1;
                    set.insert(start,steps);
                }
            },
            'U' => {
                for _ in 0..dist {
                    steps += 1;
                    start.1 += 1;
                    set.insert(start,steps);
                }
            },
            'D' => {
                for _ in 0..dist {
                    steps += 1;
                    start.1 -= 1;
                    set.insert(start,steps);
                }
            },
            _ => panic!("Invalid direction: {}", dir),
        }
    }

    set
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT_LINE1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    const TEST_INPUT_LINE2: &str = "U62,R66,U55,R34,D71,R55,D58,R83";

    const TEST_HASH_INPUT: &str = "R2,D3,L1,U2";
    const TEST_COORDS: [(isize,isize);8] = [ (1, 0), (2, 0), (2, -1),(2,-2), (2,-3), (1,-3),(1,-2),(1,-1)];
    const TEST_COORDS_MAPS: [((isize,isize),isize);8] = [ ((1, 0),1), ((2, 0),2), ((2, -1),3),((2,-2),4), ((2,-3),5), ((1,-3),6),((1,-2),7),((1,-1),8)];

    #[test]
    fn test_create_set() {
        let expect = HashSet::from(TEST_COORDS);
        let map = create_map(TEST_HASH_INPUT);
        let coords = create_set(&map);

        for coord in expect {
            assert!(coords.contains(&coord))
        }
    }
    
    #[test]
    fn test_pt1() {
        let map_a = create_map(TEST_INPUT_LINE1);
        let map_b = create_map(TEST_INPUT_LINE2);
        let set_a = create_set(&map_a);
        let set_b = create_set(&map_b);
        let result = find_min_distance(set_a, set_b);
        let expect = 159;
        assert_eq!(result, expect);
    }
    
    #[test]
    fn test_create_map() {
        let coords = create_map(TEST_HASH_INPUT);
        let expect = HashMap::from(TEST_COORDS_MAPS);


        assert_eq!(coords, expect);
    }

    #[test]
    fn test_pt2() {
        let expect = 610;
        let map_a = create_map(TEST_INPUT_LINE1);
        let map_b = create_map(TEST_INPUT_LINE2);

        let result = find_min_delay(&map_a, &map_b);

        assert_eq!(result,expect);

    }

}