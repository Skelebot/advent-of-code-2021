fn main() {
    let input: Vec<String> = input::read_lines("puzzles/day3.txt");

    println!("solution 1: {}", solve1(&input));
    println!("solution 2: {}", solve2(&input));
}

fn solve1(nums: &[String]) -> u32 {
    let mut gamma: u32 = 0;

    let mut bitpos = 0;
    'main: loop {
        let mut one_counter = 0;
        for line in nums {
            match line.chars().nth(bitpos) {
                Some('1') => one_counter += 1,
                None => break 'main,
                _ => (),
            }
        }
        let zeros = nums.len() as i32 - one_counter as i32;
        if one_counter > zeros {
            gamma |= 1 << bitpos;
        }
        bitpos += 1;
    }

    gamma = gamma.reverse_bits();
    gamma >>= u32::BITS - bitpos as u32;

    let mut epsilon = 0;
    for i in 0..bitpos {
        if gamma & (1 << i) == 0 {
            epsilon |= 1 << i;
        }
    }

    gamma * epsilon
}

fn solve2(nums: &[String]) -> u32 {
    let mut onums: Vec<&str> = nums.iter().map(|s| s.as_ref()).collect();
    let mut conums: Vec<&str> = nums.iter().map(|s| s.as_ref()).collect();
    
    let mut bitpos = 0;
    'main: loop {
        if onums.len() == 1 { break 'main; }
        let mut one_counter = 0;
        for line in &onums {
            match line.chars().nth(bitpos) {
                Some('1') => one_counter += 1,
                None => break 'main,
                _ => (),
            }
        }
        let zeros = onums.len() as i32 - one_counter as i32;
        if one_counter >= zeros {
            onums.retain(|n| n.chars().nth(bitpos).unwrap() == '1');
        } else {
            onums.retain(|n| n.chars().nth(bitpos).unwrap() == '0');
        }
        bitpos += 1;
    }

    bitpos = 0;
    'main2: loop {
        if conums.len() == 1 { break 'main2; }
        let mut one_counter = 0;
        for line in &conums {
            match line.chars().nth(bitpos) {
                Some('1') => one_counter += 1,
                None => break 'main2,
                _ => (),
            }
        }
        let zeros = conums.len() as i32 - one_counter as i32;
        if one_counter >= zeros {
            conums.retain(|n| n.chars().nth(bitpos).unwrap() == '0');
        } else {
            conums.retain(|n| n.chars().nth(bitpos).unwrap() == '1');
        }
        bitpos += 1;
    }
    
    assert_eq!(onums.len(), 1);
    assert_eq!(conums.len(), 1);

    let mut oval = 0;
    let mut coval = 0;
    let bits = onums[0].len();
    for bit in 0..bits {
        if onums[0].chars().nth(bit)  == Some('1') {
            oval |= 1 << (bits - bit - 1);
        }
        if conums[0].chars().nth(bit)  == Some('1') {
            coval |= 1 << (bits - bit - 1);
        }
    }
    
    oval * coval
}

#[test]
fn test1() {
    let input = vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
    ];

    assert_eq!(solve1(&input), 198);
    assert_eq!(solve2(&input), 230);
}
