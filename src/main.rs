use std::fs;
use std::dbg;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let code = contents.trim();
    println!("{}", interpret_intcode(&code));

    println!("{:?}", brute_force(&code, 19690720));
}

pub fn interpret_intcode(code: &str) -> String {
    let mut v: Vec<i64> = code.split(',').map(|x| x.parse().unwrap()).collect();
    interpret(&mut v);
    v.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

fn interpret(v: &mut [i64]) { 
    for i in (0..v.len()).step_by(4) {
        if i+3 > v.len() { break }
        let [opcode, left, right, loc] = [v[i], v[i+1], v[i+2], v[i+3]];
        match opcode {
            1 => add_op(v, left, right, loc),
            2 => mult_op(v, left, right, loc),
            99 => break,
            _ => ()
        }
    }
}

fn add_op(v: &mut [i64], left: i64, right: i64, loc: i64) {
    v[loc as usize] = v[left as usize] + v[right as usize]
}

fn mult_op(v: &mut [i64], left: i64, right: i64, loc: i64) {
    v[loc as usize] = v[left as usize] * v[right as usize]
}

pub fn brute_force(code: &str, n: i64) -> i64 {
    for j in 0..99 {
        for k in 0..99 {
            let mut v: Vec<i64> = code.split(',').map(|x| x.parse().unwrap()).collect();
            v[1] = j;
            v[2] = k;
            interpret(&mut v);
            if v[0] == n {
                return j * 100 + k;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    mod two {
        use super::*;
        #[test]
        fn test_ex_1() {
            assert_eq!(interpret_intcode("1,0,0,0,99"), "2,0,0,0,99");
        }
        
        #[test]
        fn test_ex_2() {
            assert_eq!(interpret_intcode("2,3,0,3,99"), "2,3,0,6,99");
        }
        
        #[test]
        fn test_ex_3() {
            assert_eq!(interpret_intcode("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        }
        
        #[test]
        fn test_ex_4() {
            assert_eq!(interpret_intcode("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
        }
        
        // #[test]
        // fn test_ex_5() {
        //     assert_eq!(brute_force(1202), Some((12, 2)));
        // }
    }
}
