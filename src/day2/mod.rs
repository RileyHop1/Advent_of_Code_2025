use std::fs::File;
use std::io::Read;

fn check_id_valid(id: u64, invalid_ids: &mut Vec<u64>) {
    let digits = id.to_string().len() as u32;

    if digits % 2 != 0 {
        return;
    }

    let half: u32 = digits / 2;
    let pow:u64 = 10_u64.pow(half);

    let first_half = id / pow;
    let second_half = id % pow;

    if first_half == second_half {
        invalid_ids.push(id);
    }
}

fn check_range(start: &str, end: &str, invalid_ids: &mut Vec<u64>) {
    let start: u64 = start.trim().parse().unwrap();
    let end: u64 = end.trim().parse().unwrap();

    for i in start..=end {
        check_id_valid(i, invalid_ids);
    }
}

pub fn day2(file_path: &str) {

    //This is the set of all invalid ids
    let mut invalid: Vec<u64> = Vec::new();

    let mut file = File::open(file_path).expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read file");

    let contents = contents.split(",");

    for i in contents {

        if let Some((start_range, end_range)) = i.split_once("-") {
            check_range(start_range, end_range, &mut invalid);
        }



    }

    let mut adder: u64 = 0;
    for i in invalid.iter() {
        println!("{}", i);
        adder += *i as u64;
    }

    println!("Adding up all the invalid IDs in this produces : {}", adder);

}
