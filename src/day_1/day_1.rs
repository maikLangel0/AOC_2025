use std::{fs::File, io::{BufRead, BufReader, Error, Read}, result::Result};

struct SafeDial<T: Sized + Read> {
    max_num: u16,
    reader: BufReader<T>
}

#[derive(Debug)]
enum SolveError {
    LineReadError,
    ValueParseError,
    SignParseError,
    LineEmptyError
}

impl<T: Sized + Read> SafeDial<T> {
    pub fn new(max_num: u16, reader_of_inputs: BufReader<T>) -> Self {
        SafeDial { max_num: max_num + 1, reader: reader_of_inputs }
    }

    pub fn new_reader(&mut self, reader_of_inputs: BufReader<T>) {
        self.reader = reader_of_inputs;
    }

    pub fn solve_first(&mut self, mut current_pos: u16) -> Result<u32, SolveError> {
        let mut hit: u32 = 0;

        for line in self.reader.by_ref().lines() {
            let (sign, value) = Self::parse_line(line)?;

            current_pos = match sign {
                'R' => {
                    (current_pos + value % self.max_num) % self.max_num
                },
                'L' => {
                    (current_pos + self.max_num - (value % self.max_num)) % self.max_num
                },
                _ => unreachable!("Should always be either R or L")
            };

            if current_pos == 0 { hit += 1 }
        }
        Ok(hit)
    }

    pub fn solve_second(&mut self, mut current_pos: u16) -> Result<u32, SolveError> {
        let mut rotations: u16;
        let mut passed: u32 = 0;

        for line in self.reader.by_ref().lines() {
            let (sign, value) = Self::parse_line(line)?;

            match sign {
                'R' => {
                    rotations = (current_pos + value) / self.max_num;

                    current_pos = (current_pos + value) % self.max_num;
                },
                'L' => {
                    let raw_rotations = value / self.max_num;
                    let rot_remainders = (current_pos != 0 && value % self.max_num >= current_pos) as u16;

                    rotations = raw_rotations + rot_remainders;

                    current_pos = (current_pos + self.max_num - (value % self.max_num)) % self.max_num;
                },
                _ => unreachable!()
            }

            passed += rotations as u32;
        }

        Ok(passed)
    }
}

impl<T: Read> SafeDial<T> {
    pub fn parse_line(line: Result<String, Error>) -> Result<(char, u16), SolveError> {
        let line = line.map_err(|_| SolveError::LineReadError)?;

        if line.is_empty() { return Err(SolveError::LineEmptyError)}

        let (sign, value) = line.split_at(1);
        let value = value.parse::<u16>().map_err(|_| SolveError::ValueParseError)?;
        let sign = sign.chars().next().ok_or( SolveError::SignParseError )?;

        Ok((sign, value))
    }
}

fn main() -> Result<(), SolveError> {
    let file = File::open("src\\day_1\\input.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut safe = SafeDial::new(99, reader);
    match safe.solve_first(50) {
        Ok(res) => println!("{}", res),
        Err(e) => println!("{:?}", e)
    }

    let file = File::open("src\\day_1\\input.txt").unwrap();
    let reader = BufReader::new(&file);

    safe.new_reader(reader);

    match safe.solve_second(50) {
        Ok(res) => println!("{}", res),
        Err(e) => println!("{:?}", e)
    };

    Ok(())
}
