use std::collections::HashMap;

const MAX: usize = 789860;
const MIN: usize = 254032;

pub fn run() -> (usize, usize) {
    (pt1(),pt2())
}

fn pt1() -> usize {
    let mut res = Vec::new();
    for i in 2..=7 {
        gen_pw(i,MIN,MAX,5,&mut res,&is_valid);
    }

    res.len()
}
fn pt2() -> usize {
    let mut res = Vec::new();
    for i in 2..=7 {
        gen_pw(i,MIN,MAX,5,&mut res,&is_valid_p2);
    }

    res.len()
}


fn gen_pw<F>(initial: usize,min:usize, max:usize,extra_digits: usize, result: &mut Vec<usize>,f:&F)  
where 
    F: Fn(usize) -> bool,
    {

    if extra_digits == 0 {
        if f(initial) && initial <= max && initial >= min && initial <= max {
            result.push(initial);
        }
        return;
    }

    let last_digit = initial % 10;
    for next_digit in last_digit..=9 {
        let dig = initial * 10 + next_digit;
        gen_pw(dig,min, max,extra_digits - 1,result,f);
    }
}


fn is_valid(num: usize) -> bool {
    let digits: Vec<u32> = num.to_string()
            .chars()
            .map(|c| c.to_digit(10).expect("Failed to get digit")).collect();

    if digits.len() != 6 {
        return false;
    }

    digits.windows(2).all(|pair| pair[0] <= pair[1])
    &&
    digits.windows(2).any(|pair| pair[0] == pair[1])
}

fn is_valid_p2(num: usize) -> bool {
    let digits: Vec<u32> = num.to_string()
            .chars()
            .map(|c| c.to_digit(10).expect("Failed to get digit")).collect();

    if digits.len() != 6 {
        return false;
    }

    let res = digits.windows(2).all(|pair| pair[0] <= pair[1]);
    let mut map = HashMap::new();
    for (i,val) in digits.iter().enumerate()  {
        map.entry(val).or_insert(Vec::new()).push(i);
    }

    let num_pairs = map.values().fold(0, |acc,val| {
        if val.len() == 2 && val[0] + 1 == val[1] {
            acc + 1
        } else {
            acc
        }
    });
    
    num_pairs >= 1 && res
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_pw_is_valid() {
        // Valid
        assert!(is_valid(111111));
        assert!(is_valid(122456));
        assert!(is_valid(145569));

        // Not valid
        assert!(!is_valid(223450));
        assert!(!is_valid(123789));
        assert!(!is_valid(111110));
        assert!(!is_valid(1));
        assert!(!is_valid(11234));
    }


    #[test]
    fn test_gen_pw() {
        let initial = 23345;
        let expected = vec!(233455,233456,233457,233458);

        let mut pws = Vec::new();
        gen_pw(initial,1,233458,1,&mut pws,&is_valid);
        pws.sort();
        assert_eq!(expected,pws);
    }

    #[test]
    fn test_pw_is_valid_p2() {
        // Valid
        assert!(is_valid_p2(111122));
        assert!(is_valid_p2(112233));
        assert!(is_valid_p2(122456));
        assert!(is_valid_p2(145569));

        // Not valid
        assert!(!is_valid_p2(223450));
        assert!(!is_valid_p2(123789));
        assert!(!is_valid_p2(111110));
        assert!(!is_valid_p2(1));
        assert!(!is_valid_p2(11234));
        assert!(!is_valid_p2(123444));
        assert!(!is_valid_p2(111234));
    }

}