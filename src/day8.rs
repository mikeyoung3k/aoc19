use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;
use std::collections::HashMap;

pub fn run() -> (isize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day8.txt");
    let input_string = read_to_string(path).expect("Error reading file");
    let layers = build_image(&input_string, 25, 6);
    (pt1(&layers), pt2(layers))
}

fn pt2(image: Image) -> isize {
    let res = image_compositor(image);
    for row in res {
        for x in row {
            if x == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    0
}

fn pt1(image: &Image) -> isize {
    let mut min_zeros = isize::MAX;
    let mut res = 0;
    for layer in image {
        let mut map: HashMap<&isize, isize> = HashMap::new();
        for row in layer {
            for x in row {
                *map.entry(x).or_insert(0) += 1;
            }
        }
        if count_map(&map, 0) < min_zeros {
            min_zeros = count_map(&map, 0);
            res = count_map(&map, 1) * count_map(&map, 2);
        }
    }
    res
}

fn count_map(map: &HashMap<&isize, isize>, target: isize) -> isize {
    map.get(&target).unwrap_or(&0).to_owned()
}

type Row = Vec<isize>;
type Layer = Vec<Row>;
type Image = Vec<Layer>;

fn build_image(data: &str, width: usize, height: usize) -> Image {
    let mut r = Image::new();
    let mut layer = Layer::new();
    let mut row = Row::new();
    let mut w = 0;
    let mut h = 0;

    for x in data.chars().map(|c| c.to_digit(10).unwrap() as isize) {
        row.push(x);
        w += 1;
        if w == width {
            layer.push(row);
            row = Row::new();
            w = 0;
            h += 1;
            if h == height {
                r.push(layer);
                layer = Layer::new();
                h = 0;
            }
        }
    }
    r
}

fn update_pixel(current: isize, next:isize) -> isize {
    match next {
        2 => current,
        _ => next,
    }
}

fn image_compositor(mut image: Image) -> Layer {
    let mut res = image.pop().unwrap();
    for layer in image.iter().rev() {
        res = layer_compositor(&res, &layer);
    }

    res
}

fn layer_compositor(current: &Layer, next:&Layer) -> Layer {
    let mut res = Layer::new();
    let mut row = Row::new();

    let mut w = 0;

    let width = current.first().unwrap().len();

    let current = current.into_iter().flatten();
    let next = next.into_iter().flatten();

    for pixel in current.zip(next) {
        row.push(update_pixel(*pixel.0, *pixel.1));
        w += 1;
        if w == width {
            res.push(row);
            row = Row::new();
            w = 0;
        }
    }
    res
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image_builder() {
        let test_data = "123456789012".to_string();
        
        let layer_1 = vec![vec![1,2,3],vec![4,5,6]];
        let layer_2 = vec![vec![7,8,9],vec![0,1,2]];
        let expect_image = vec![layer_1, layer_2];

        let got_image = build_image(&test_data, 3, 2);

        assert_eq!(expect_image,got_image);
    }

    #[test]
    fn test_pt1() {
        let test_data = "123456789012".to_string();
        let image = build_image(&test_data, 3, 2);
        println!{"Image: {:?}", image};

        let got_res = pt1(&image);

        assert_eq!(got_res,1);
    }

    #[test]
    fn test_image_compositer() {
        let test_data = "0222112222120000".to_string();
        let image = build_image(&test_data, 2, 2);

        let expect_image = vec![
            vec![0,1],
            vec![1,0],
        ];

        let got_image = image_compositor(image);

        assert_eq!(expect_image, got_image);
    }
}