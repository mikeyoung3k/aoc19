use std::fs::read_to_string;
use std::path::Path;

use crate::BASE_DIR;


pub fn run() -> (usize, usize) {
    let data_path = Path::new(BASE_DIR).join("real").join("day1.txt");
    let data = get_data(&data_path);
    let pt1 = part1(&data);
    let pt2 = part2(&data);
    (pt1,pt2)
}

fn get_data(path: &Path) -> Vec<usize> {
    read_to_string(path).expect("Error reading file")
    .lines()
    .map(|line| line.parse().expect("Error parsing line to integer"))
    .collect()
}

fn part1(data: &[usize]) -> usize {
    data.iter()
    .map(|n| n.checked_div(3).expect("Bad divide").checked_sub(2).expect("Bad sub"))
    .sum()
}

fn part2(data: &[usize]) -> usize {
    data.iter()
    .map(|n| {
        fuel_calc(*n)
    }).sum()
}

fn fuel_calc(mass: usize) -> usize {
    if let Some(x) = mass.checked_div(3) {
        if let Some(res) = x.checked_sub(2) {
            res + fuel_calc(res)
        } else {
            0
        }
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pt1() {
        let test_path = Path::new(BASE_DIR).join("test").join("day1.txt");
        let test_data = get_data(&test_path);
        assert_eq!(part1(&test_data), 34241);
    }

    #[test]
    fn test_fuel_calc() {
        assert_eq!(fuel_calc(14), 2);
        assert_eq!(fuel_calc(1969), 966);
        assert_eq!(fuel_calc(100756), 50346);
    }
}