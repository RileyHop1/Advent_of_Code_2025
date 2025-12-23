use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct CircularArray {
    pub pointer: i32,
    length: i32,
    array: Vec<i32>,
    pass: u32,
}

impl CircularArray {
    fn new(length: i32) -> CircularArray {
        return CircularArray {
            pointer: 50,
            length,
            array: vec![0; length as usize],
            pass: 0,
        };
    }

    fn read_in_file(&mut self,file_path: &str) -> io::Result<(u32)> {
        let file = File::open(file_path).expect("File not found");
        let reader = BufReader::new(file);


        for line in reader.lines() {
            let line = line?;

            self.move_index(&line);
        }

        return Ok(self.pass);

    }

    fn move_index(&mut self ,line: &str) {


        let mut change : i32 = 0;
        let mut index :u32 = 0;
        for c in line.chars().rev() {

            match c {
                'L' => self.pointer -= change,
                'R' => self.pointer += change,
                '0' ..='9' => {
                    let digit = c.to_digit(10).unwrap() as i32;
                    change += digit * 10_i32.pow(index);
                },
                _ => panic!("invalid input {}", line),
            }

            index += 1;
        }

        self.pointer = self.pointer.rem_euclid(self.length);

        if self.pointer == 0 {
            self.pass += 1;
        }



    }

    pub fn secret_entrance(the_path: &str ) -> u32{
        let mut arr = CircularArray::new(100);

        return arr.read_in_file(the_path).expect("Unable to read file");
    }


}

