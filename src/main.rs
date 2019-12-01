fn main() {
    day_one_p1();
    day_one_p2();
}

fn day_one_p1() {
    let input = vec![
        105311, 117290, 97762, 124678, 132753, 114635, 114137,
        96208, 82957, 148510, 75509, 120845, 80279, 112588,
        136983, 91546, 55087, 98239, 58629, 59526, 121740,
        133887, 96246, 53621, 88458, 144101, 67449, 114870,
        75125, 126117, 118155, 108888, 128347, 121556, 65380,
        106487, 149660, 89018, 118897, 91556, 147829, 123137,
        130352, 51301, 102756, 83357, 97466, 78364, 82291,
        83367, 72243, 107128, 87975, 93719, 114888, 71559,
        57757, 145975, 74254, 102427, 117302, 118842, 105979,
        134735, 123676, 83647, 101511, 117834, 70884, 88288,
        55444, 71415, 143464, 142131, 51118, 109435, 87841,
        107406, 71379, 124659, 79427, 110357, 114485, 141168,
        62923, 113921, 106154, 67468, 132601, 76112, 84953,
        124290, 55476, 88965, 107153, 148407, 62584, 112851,
        71564, 145569];

    let result = input.into_iter().map(|x| calculate_fuel(x)).sum::<i64>();
    println!("{}", result);
}

fn day_one_p2() {
    let input = vec![
        105311, 117290, 97762, 124678, 132753, 114635, 114137,
        96208, 82957, 148510, 75509, 120845, 80279, 112588,
        136983, 91546, 55087, 98239, 58629, 59526, 121740,
        133887, 96246, 53621, 88458, 144101, 67449, 114870,
        75125, 126117, 118155, 108888, 128347, 121556, 65380,
        106487, 149660, 89018, 118897, 91556, 147829, 123137,
        130352, 51301, 102756, 83357, 97466, 78364, 82291,
        83367, 72243, 107128, 87975, 93719, 114888, 71559,
        57757, 145975, 74254, 102427, 117302, 118842, 105979,
        134735, 123676, 83647, 101511, 117834, 70884, 88288,
        55444, 71415, 143464, 142131, 51118, 109435, 87841,
        107406, 71379, 124659, 79427, 110357, 114485, 141168,
        62923, 113921, 106154, 67468, 132601, 76112, 84953,
        124290, 55476, 88965, 107153, 148407, 62584, 112851,
        71564, 145569];

    let result = input.into_iter().map(|x| calculate_fuel_with_fuel_mass(x)).sum::<i64>();
    println!("{}", result);
}

pub fn calculate_fuel(mass: i64) -> i64 {
    // Fuel required to launch a given module is based on its mass. 
    // Specifically, to find the fuel required for a module, take its mass,
    // divide by three, round down, and subtract 2.
    (mass/3) - 2
}

pub fn calculate_fuel_with_fuel_mass(mass: i64) -> i64 {
    // Fuel required to launch a given module is based on its mass. 
    // Specifically, to find the fuel required for a module, take its mass,
    // divide by three, round down, and subtract 2.
    let mut fuel_cost = (mass/3) - 2;
    let mut i = 0i64;
    
    while fuel_cost > 0 {
        i += fuel_cost;
        fuel_cost = calculate_fuel(fuel_cost);
    }

    i 
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    mod day_one {
        use super::*;
        #[test]
        fn test_ex_1() {
            assert_eq!(calculate_fuel(12), 2);
        }

        #[test]
        fn test_ex_2() {
            assert_eq!(calculate_fuel(14), 2);
        }

        #[test]
        fn test_ex_3() {
            assert_eq!(calculate_fuel(1969), 654);
        }

        #[test]
        fn test_ex_4() {
            assert_eq!(calculate_fuel(100_756), 33583);
        }

        #[test]
        fn test_ex_5() {
            assert_eq!(calculate_fuel_with_fuel_mass(14), 2);
        }

        #[test]
        fn test_ex_6() {
            assert_eq!(calculate_fuel_with_fuel_mass(1969), 966);
        }

        #[test]
        fn test_ex_7() {
            assert_eq!(calculate_fuel_with_fuel_mass(100756), 50346);
        }
    }

    mod day_two {
        use super::*;
        #[test]
        fn test_ex_1() {
            assert_eq!(calculate_fuel(12), 2);
        }
        
    }
}
